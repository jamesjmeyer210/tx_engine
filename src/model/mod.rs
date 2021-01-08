mod serial_transaction;
mod transaction;
mod account;

pub type SerialTransaction = serial_transaction::SerialTransaction;

pub type Transaction = transaction::Transaction;
pub type TxType = transaction::TxType;
pub type ClientId = transaction::ClientId;

pub type Account = account::Account;
