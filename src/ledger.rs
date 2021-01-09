use csv::{Reader};
use std::fs::File;
use std::convert::TryFrom;
use crate::model::{SerialTransaction, Transaction, TransactionError, TxType, ClientId, TxId};
use crate::model::{Account, AccountError};
use crate::{FindBy, TryAdd, Verify};

#[derive(Debug)]
pub enum LedgerError {
    TransactionNotFound,
    DuplicateTransaction,
    AccountError(AccountError),
    TransactionError(TransactionError),
}
// Enable propagation of errors up from Account mutation layer
impl From<AccountError> for LedgerError {
    fn from(e: AccountError) -> Self {
        LedgerError::AccountError(e)
    }
}
// Enable propagation of errors from Transaction creation layer
impl From<TransactionError> for LedgerError {
    fn from(e: TransactionError) -> Self {
        LedgerError::TransactionError(e)
    }
}
// Ledger is a data structure that tracks transactions and accounts, keeping the respective
// entities on the heap
pub struct Ledger {
    transactions: Vec<Box<Transaction>>,
    accounts: Vec<Box<Account>>,
}

impl Ledger {
    pub fn display(&self) -> () {
        self.accounts.iter().for_each(|a|{
            println!("{:?}", a.as_ref())
        })
    }
}

impl TryFrom<&mut Reader<File>> for Ledger {
    type Error = LedgerError;

    fn try_from(buffer: &mut Reader<File>) -> Result<Self, Self::Error> {
        let mut ledger = Ledger {
            transactions: Vec::new(),
            accounts: Vec::new(),
        };

        for result in buffer.deserialize() {
            let tx: SerialTransaction = result.unwrap();
            let tx = Transaction::try_from(tx)?;
            ledger.try_add(tx)?;
        }

        Ok(ledger)
    }
}

impl FindBy<TxId> for Vec<Box<Transaction>> {
    fn find_by(&self, target: u32) -> Option<usize> {
        let mut index = 0;
        for transaction in self.iter() {
            if transaction.tx == target {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}

// Verify<&Transaction> for Vec<Box<Transaction>>
// There are 2 types of transactions regarding how they are added: 1) value transactions
// and 2) reference transactions. Value transactions are new items, with unique ids. Reference
// transactions are dependent on an existing transaction id in order to exist.
// Transactions are also unique, so transaction ids must never be duplicated.
impl Verify<Box<Transaction>> for Vec<Box<Transaction>> {
    type Error = LedgerError;

    fn verify(&self, tx: Box<Transaction>) -> Result<Option<Box<Transaction>>,LedgerError> {
        match tx.tx_type {
            // case of the reference transactions
            TxType::Dispute | TxType::Deposit | TxType::Chargeback => {
                match self.find_by(tx.tx) {
                    Some(index) => {
                        let reference = self.get(index).unwrap();
                        let result = Transaction {
                            tx_type: tx.tx_type,
                            tx: tx.tx,
                            client: tx.client,
                            amount: reference.amount,
                        };
                        Ok(Some(Box::new(result)))
                    },
                    None => Ok(None),
                }
            },
            // case of the value transactions
            _ => {
                match self.find_by(tx.tx) {
                    None => Ok(Some(tx)),
                    _ => Err(LedgerError::DuplicateTransaction),
                }
            }
        }
    }
}

impl FindBy<ClientId> for Vec<Box<Account>> {
    fn find_by(&self, target: u16) -> Option<usize> {
        let mut index = 0;
        for account in self.iter() {
            if account.client == target {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}

impl TryAdd<&Transaction> for Vec<Box<Account>> {
    type Error = AccountError;

    fn try_add(&mut self, tx: &Transaction) -> Result<&Self, Self::Error> {
        match self.find_by(tx.client) {
            Some(index) => {
                self.get_mut(index).unwrap().try_add(tx)?;
            },
            None => {
                let mut account = Account::with_client_id(tx.client);
                account.try_add(tx)?;
                self.push(Box::new(account))
            }
        }
        Ok(self)
    }
}

impl TryAdd<Transaction> for Ledger {
    type Error = LedgerError;

    fn try_add(&mut self, tx: Transaction) -> Result<&Self, Self::Error> {
        match self.transactions.verify(Box::new(tx))? {
            Some(tx) => {
                self.accounts.try_add(&tx)?;
                self.transactions.push(tx);
                Ok(self)
            },
            None => {
                Ok(self)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn find_by_returns_index_of_account() {
        let accounts: Vec<Box<Account>> = vec![Box::new(Account::with_client_id(1))
                                          , Box::new(Account::with_client_id(2))
                                          , Box::new(Account::with_client_id(3))];

        assert_eq!(Some(2), accounts.find_by(3));
    }

    #[test]
    fn find_by_returns_none_when_account_does_not_exist() {
        let accounts: Vec<Box<Account>> = vec![Box::new(Account::with_client_id(1))
                                          , Box::new(Account::with_client_id(2))
                                          , Box::new(Account::with_client_id(3))];

        assert_eq!(None, accounts.find_by(4));
    }

}