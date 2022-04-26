use serde::Deserialize;

// Type of transactions enum
// Using aliasis in case first leter of transaction type is lowercase
#[derive(Clone, Copy, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

// Holds all the information for a transaction
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Transaction {
    // Transaction type
    #[serde(rename = "type")]
    pub tx_type:  TxType,
    // Client ID
    #[serde(rename = "client")]
    pub client_id: u16,
    #[serde(rename = "tx")]
    // Transaction ID
    pub tx_id: u32,
    #[serde(rename = "amount")]
    #[serde(default = "default_resource")]
    // Transaction amount
    pub tx_amount: f32,
}

impl Transaction {
    // Returns a copy of internal transaction data, including
    // the transaction type and amount.
    pub fn get_data(&self) -> (TxType,f32) {
        (self.tx_type, self.tx_amount)
    }
}

// Used for dispute, resolve, chargeback transactions because they 
// don't include the amount field.
fn default_resource() -> f32 {
    0.0
}

#[cfg(test)]
mod tests {
    use super::{Transaction, TxType};

    #[test]
    fn retrieve_data() {
        let tx = Transaction{
            tx_type: TxType::Deposit,
            client_id: 01,
            tx_id: 02032,
            tx_amount: 123.34
        };
        assert_eq!((TxType::Deposit, 123.34), tx.get_data());
    }
}


