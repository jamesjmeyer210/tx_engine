mod account;
mod serial_transaction;
mod transaction;

pub type SerialTransaction = serial_transaction::SerialTransaction;

pub type Transaction = transaction::Transaction;
pub type TransactionError = transaction::TransactionError;
pub type TxType = transaction::TxType;
pub type ClientId = transaction::ClientId;
pub type TxId = transaction::TxId;

pub type Account = account::Account;
pub type AccountError = account::AccountError;
