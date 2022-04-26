use simple_ledger::{initialize, process_txs, output_acccount_balances};

fn main() {

    if let Err(err) = initialize()
        .and_then(process_txs)
        .and_then(output_acccount_balances)
    {
        eprintln!("{err}");
        std::process::exit(1);
    }
}