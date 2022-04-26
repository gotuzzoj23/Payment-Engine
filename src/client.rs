use tinyset::SetU32;
use std::collections::HashMap;
use anyhow::Result;

use crate::{
    transaction::TxType
};

#[derive(Debug, PartialEq, Clone)]
pub struct Client {
    // Available balance
    balance_available: f32,
    // Held balance
    balance_held: f32,
    // Total balance
    balance_total: f32,
    // Map of client's transactions
    transactions: HashMap<u32,(TxType, f32)>,
    // List of disputed transactions
    disputed_tx: SetU32,
    // Previous transaction ID
    previous_tx_id: u32,
    // Flag indicating the account is frozen (chargeback)
    frozen: bool
}

impl Client{
    // Returns new client
    pub fn new(tx_id: u32, tx_type: TxType, tx_value:f32) -> Self {
        let balance = { if tx_type == TxType::Deposit {
            tx_value
        } else {
            0.0
        }};

        let mut client = Client {
            balance_available: balance,
            balance_held: 0.0,
            balance_total: balance,
            transactions: HashMap::new(),
            disputed_tx: SetU32::new(),
            previous_tx_id: tx_id,
            frozen: false,
        };

        client.log_tx(tx_id, tx_type, tx_value);
        return client
    }

    // Checks if the account is currently frozen.
    // Returns true if it's frozen
    pub fn account_frozen(&self, tx_id: u32) -> Result<bool> {
        if self.frozen {
            Err(anyhow!("Account is currently frozen, transaction ID: {} was not processed!", tx_id))
        } else {
            return Ok(self.frozen); 
        }
    }

    // Check current transaction ID is valid
    pub fn check_tx_id(&self, tx_id: u32) -> Result<()> {
        if self.previous_tx_id < tx_id {
            return Ok(())
        }
        return Err(anyhow!("Current transaction ID should be greater than previous processed
             transaction! \n Previous TX ID: {} \n Currrent TX ID: {tx_id}", self.previous_tx_id))
    }

    // Store current transaction to map and previous transaction 
    pub fn log_tx(&mut self, tx_id: u32, tx_type: TxType, tx_value:f32) {
        self.previous_tx_id = tx_id;
        self.transactions.insert(tx_id,(tx_type, tx_value));
    }

    // Checks if there is sufficient funds available to process transaction
    pub fn sufficient_funds(&self, tx_value: f32) -> Result<()> {
        if self.balance_available >= tx_value {
            return Ok(())
        }
        return Err(anyhow!("Not enough available balance to process withdrawal!
        \n Balance: {}
        \n Withdrawal Amount: {tx_value}", self.balance_available))
    }

    // Checks the disputed status of a past transaction and compares
    // it to the value passed into the the call 
    pub fn disputed_status(&self, tx_id: u32, status: bool) -> Result<()> {
        if self.disputed_tx.contains(tx_id) == status {
            return Ok(())
        }
        return Err(anyhow!("Transaction ID: {tx_id} is already labeled as disputed!"))
    }

    // Searches the logs for the transaction ID and if found returns value of transaction
    pub fn get_tx_val(&self, tx_id: u32) -> Result<f32> {
        match self.transactions.get(&tx_id) {
            Some((_, value)) => return Ok(*value),
            None => {
                Err(anyhow!("Failed to get value! Transaction ID: {tx_id} does not exist!"))
            }
        }
    }

    // Processes the current transaction depending 
    pub fn process_tx(&mut self, tx_id: u32, tx_type: TxType, tx_value:f32) -> Result<()> {
        self.account_frozen(tx_id)?;

        match tx_type {
            TxType::Deposit =>  {
                self.balance_available += tx_value;
                self.balance_total = self.balance_available + self.balance_held;
                self.log_tx(tx_id, tx_type, tx_value);
            },
            // If client does not have suffecient funds available, the withdraw will fail
            // and the account's state will remain unchanged.
            TxType::Withdrawal => {
                self.sufficient_funds(tx_value)?;
                self.balance_available -= tx_value;
                self.balance_total = self.balance_available + self.balance_held;
                self.log_tx(tx_id, tx_type, tx_value);
            }
            // If the transaction ID is valid, held funds will increase and 
            // available balance will decrease by the funds asscociated to the 
            // provided transaction ID.
            TxType::Dispute => {
                self.disputed_status(tx_id, false)?;
                let disputed_val = self.get_tx_val(tx_id)?;
                self.sufficient_funds(disputed_val)?;
                self.balance_available -= disputed_val;
                self.balance_held += disputed_val;
                self.disputed_tx.insert(tx_id);
            },
            // If the transaction ID is valid and it is under dispute, held 
            // funds will decrease and available balance will increase by the
            // funds asscociated to the provided transaction ID.
            TxType::Resolve => {
                self.disputed_status(tx_id, true)?;
                let disputed_val = self.get_tx_val(tx_id)?;
                if disputed_val <= self.balance_held {
                    self.balance_available += disputed_val;
                    self.balance_held -= disputed_val;
                    self.disputed_tx.remove(tx_id);
                }
            },
            // If the transaction ID is valid and it is under dispute, funds
            // that were held will be withdrawn.
            // Held funds and total funds will decrease by the funds previously
            // disputed. 
            TxType::Chargeback => {
                self.disputed_status(tx_id, true)?;
                let disputed_val = self.get_tx_val(tx_id)?;
                if disputed_val <= self.balance_held {
                    self.frozen = true;
                    self.balance_held -= disputed_val;
                    self.balance_total -= disputed_val;
                    self.disputed_tx.remove(tx_id);
                }
            },
        }
        Ok(())
    }

    // Retrieves client's account infomation
    pub fn get_acc(&self, client_id: u16) -> Vec<String> {
        vec![
            client_id.to_string(),
            format!("{:.4}", self.balance_available),
            format!("{:.4}", self.balance_held),
            format!("{:.4}", self.balance_total),
            self.frozen.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    pub fn client_creation() { 
        let client1 = Client::new(5546465,TxType::Deposit, 4549.5411);

        let mut tx_log: HashMap<u32, (TxType, f32)> = HashMap::new();
        tx_log.insert(5546465,(TxType::Deposit, 4549.5411));

        let client2 = Client {
            balance_available: 4549.5411,
            balance_held: 0.0,
            balance_total: 4549.5411,
            transactions: tx_log,
            disputed_tx: SetU32::new(),
            previous_tx_id: 5546465,
            frozen: false,
        };
        assert_eq!(client1, client2);
    }
}