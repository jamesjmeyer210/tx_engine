use csv::{Reader};
use std::fs::File;
use std::convert::TryFrom;
use crate::model::{SerialTransaction, Transaction, TxType, ClientId, TxId};
use crate::model::{Account, AccountError};
use crate::{Contains, FindBy, TryAdd};

pub struct Ledger {
    transactions: Vec<Box<Transaction>>,
    accounts: Vec<Box<Account>>,
}

pub type LedgerError = &'static str;

impl TryFrom<&mut Reader<File>> for Ledger {
    type Error = ();

    fn try_from(buffer: &mut Reader<File>) -> Result<Self, Self::Error> {
        for result in buffer.deserialize() {
            let tx: SerialTransaction = result.unwrap();
            let tx = Transaction::try_from(tx)?;
        }

        Ok(Ledger {
            transactions: Vec::new(),
            accounts: Vec::new(),
        })
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

impl Contains<ClientId> for Vec<Box<Account>> {
    fn contains(&self, target: ClientId) -> bool {
        self.find_by(target).is_some()
    }
}

impl TryAdd<Transaction> for Vec<Box<Transaction>> {
    type Error = ();

    fn try_add(&mut self, tx: Transaction) -> Result<&Self, Self::Error> {
        match tx.tx_type {
            TxType::Dispute | TxType::Deposit | TxType::Chargeback => {
                match self.find_by(tx.tx) {
                    Some(_) => Ok(self),
                    None => Err(()),
                }
            },
            _ => {
                self.push(Box::new(tx));
                Ok(self)
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
                let mut account: &mut Account = self.get_mut(index).unwrap();
                account.try_add(tx)?;
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
        match tx.tx_type {
            TxType::Dispute | TxType::Deposit | TxType::Chargeback => {
                let index = self.transactions.find_by(tx.tx);

                Ok(self)
            },
            _ => {


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

    #[test]
    fn contains_returns_false_when_client_id_does_not_exit() {
        let accounts: Vec<Box<Account>> = vec![Box::new(Account::with_client_id(1))];

        let result = accounts.contains(2);
        assert_eq!(false, result);
    }

    #[test]
    fn contains_returns_true_when_client_id_exits() {
        let accounts: Vec<Box<Account>> = vec![Box::new(Account::with_client_id(2))];

        let result = accounts.contains(2);
        assert_eq!(true, result);
    }

}