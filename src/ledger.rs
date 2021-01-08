use csv::{Reader};
use std::fs::File;
use std::convert::TryFrom;
use crate::model::{SerialTransaction, Transaction, TxType, ClientId, TxId};
use crate::model::Account;
use crate::{Contains, FindBy, TryAdd};

pub struct Ledger {
    transactions: Vec<Transaction>,
    accounts: Vec<Account>,
}

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

impl FindBy<TxId> for Vec<Transaction> {
    fn find_by(&self, target: u32) -> Option<u32> {
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

impl Contains<ClientId> for Vec<Account> {
    fn contains(&self, target: ClientId) -> bool {
        self.find_by(target).is_some()
    }
}

impl FindBy<ClientId> for Vec<Account> {
    fn find_by(&self, target: u16) -> Option<u32> {
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

impl TryAdd<Transaction> for Ledger {
    type Error = ();

    fn try_add(&mut self, tx: Transaction) -> Result<&Self, Self::Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn find_by_returns_index_of_account() {
        let accounts: Vec<Account> = vec![Account::with_client_id(1)
                                          , Account::with_client_id(2)
                                          , Account::with_client_id(3)];

        assert_eq!(Some(2), accounts.find_by(3));
    }

    #[test]
    fn find_by_returns_none_when_account_does_not_exist() {
        let accounts: Vec<Account> = vec![Account::with_client_id(1)
                                          , Account::with_client_id(2)
                                          , Account::with_client_id(3)];

        assert_eq!(None, accounts.find_by(4));
    }

    #[test]
    fn contains_returns_false_when_client_id_does_not_exit() {
        let accounts: Vec<Account> = vec![Account::with_client_id(1)];

        let result = accounts.contains(2);
        assert_eq!(false, result);
    }

    #[test]
    fn contains_returns_true_when_client_id_exits() {
        let accounts: Vec<Account> = vec![Account::with_client_id(2)];

        let result = accounts.contains(2);
        assert_eq!(true, result);
    }

}