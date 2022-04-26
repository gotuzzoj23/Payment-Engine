#[macro_use]
extern crate anyhow;

pub mod app_state;
pub mod client;
pub mod transaction;

use crate::{
    app_state::AppState,
    client::Client,
};

use std::{
    fs::File,
    collections::HashMap,
};

use csv::{ByteRecord, ReaderBuilder, Reader, Trim, Writer};
use transaction::Transaction;
use anyhow::Result;

lazy_static::lazy_static! {
    // Deposits and Withdrawals have 4 inputs
    pub static ref IN_HEADER_L: ByteRecord = ByteRecord::from(
        vec!["type", "client", "tx", "amount"]
    );

    // Disputes, Resolves, and Chargebacks have 3 inputs 
    pub static ref IN_HEADER_S: ByteRecord = ByteRecord::from(
        vec!["type", "client", "tx"]
    ); 
}


// Opens file name read from command line.
// Returns App's state and a CSV parser
pub fn initialize() -> Result<(AppState, Reader<File>)> {
    let file_name = std::env::args().nth(1).expect("Unable to get arguments");
    let app = AppState::new(&file_name);

    match File::open(file_name) {
        // Ok(file) => return Ok((app, Reader::from_reader(file))), 
        Ok(file) => {
            return Ok((app, ReaderBuilder::new().delimiter(b',')
                .flexible(true).trim(Trim::All).from_reader(file)))
        }, 
        Err(e) => return Err(e)?,
    };
        
}

// Processes transactions from file and returns a map of the resulting balances
pub fn process_txs(utils: (AppState,Reader<File>)) -> Result<HashMap<u16,Client>>{
    let mut app = utils.0;
    let mut reader = utils.1;
    let mut record = ByteRecord::new();

    while reader.read_byte_record(&mut record)? { 
        let tx: Transaction = record.deserialize(match record.len() {
            3 => Some(&IN_HEADER_S),
            4 => Some(&IN_HEADER_L),
            _ => {
                return Err(anyhow!(
                    "Error reading data, invalid length of {}.",
                    record.len()))
            },
        })?;

        app.clients.entry(tx.client_id)
            .and_modify(|client| {
                if let Err(err) = client.process_tx(tx.tx_id, tx.tx_type, tx.tx_amount) {
                    eprintln!("Error processing transaction! {tx:?}\n{err}")
                }
            }).or_insert_with(|| Client::new(tx.tx_id, tx.tx_type, tx.tx_amount));
        }

        return Ok(app.clients);
    }


// Prints out the status of the acounts to the standard output
pub fn output_acccount_balances(clients: HashMap<u16,Client>) -> Result<()>{
    let mut writer = Writer::from_writer(std::io::stdout());
    // Write the header values to the record to printout in the output
    _ = writer.write_record(&["client", "available", "held", "total", "locked"]);
    
    for (client_id, client) in clients {
        if let Err(error) = writer.write_byte_record(&ByteRecord::from(client.get_acc(client_id))) {
            eprintln!("Error in writing records! \n {}", error)
        }
    }

    Ok(())
}