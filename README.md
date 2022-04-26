# Payment-Engine
A simple toy transaction engine that can process transactions, deposits, withdrawals, deposits, resolves, and chargebacks. The engine updates client's accounts and outputs the state of the clients accounts as a CSV file.

## Usage
To run the payments engine enter the input file and output file like the example below:

`$ cargo run -- transactions.csv > accounts.csv`

## Assumptions
* The input is a CSV file olumns type, client, tx, and amount.
* Type is a string, the client column is a valid u16 client ID, the tx is a valid u32 transaction ID, and the amount is a decimal value with a precision of up to four places past the decimal.
* Disputes will only work for deposits.
* A transaction can be disputed/resolved many times, but charged back only once.
* If account is frozen all operations are blocked.

**Input example:**
type | client | tx | amount | 
--- | --- | --- | --- 
deposit | 1 | 1 | 10.0
deposit | 2 | 2 | 4.0
deposit | 1 | 3 | 5.0
withdrawal | 1 | 4 | 3.0
deposit | 2 | 5 | 5.0
dispute | 1 | 3 | 
chargeback | 1 | 3 |
dispute | 2 | 5 | 

**Output example:**
client | available | held | total | locked
--- | --- | --- | --- | ---
1 | 7.0 | 0.0 | 7.0 | false
2 | 4.0 | 5.0 | 9.0 | true

## Design
* Used a component based with a single thread. This allows for updates to the program so we can implement concurrent threads using a runtime like Tokio.
* The App struct has the state of the app, saves the CSV input file name entered in the comand line, and stores all the clients in an unorder data structure (hashmap).
* The Client struct holds all the data important for an account like available balance, held balance, log of past transactions, and if the account is frozen.

