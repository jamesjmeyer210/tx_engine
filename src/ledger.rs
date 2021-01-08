use csv::{Reader, DeserializeRecordsIntoIter};
use std::fs::File;
use std::convert::TryFrom;
use crate::model::{SerialTransaction, Transaction, ClientId};
use crate::model::Account;
use crate::{Add, Contains};

pub struct Ledger {
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
            accounts: Vec::new()
        })
    }
}

impl Contains<ClientId> for Ledger {
    fn contains(&self, target: ClientId) -> bool {
        let mut contains = false;
        self.accounts.iter().for_each(|a|{
            if a.client == target {
                contains = true;
            }
        });
        contains
    }
}

impl Add<Transaction> for Ledger {
    fn add(&mut self, n: Transaction) -> () {
        if !self.contains(n.client) {

        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn contains_returns_false_when_client_id_does_not_exit() {
        let ledger = Ledger {
            accounts: vec![Account {
                client: 1,
                available: 0.0,
                held: 0.0,
                total: 0.0,
                locked: false,
            }]
        };

        let result = ledger.contains(2);
        assert_eq!(false, result);
    }

    #[test]
    fn contains_returns_true_when_client_id_exits() {
        let ledger = Ledger {
            accounts: vec![Account {
                client: 2,
                available: 0.0,
                held: 0.0,
                total: 0.0,
                locked: false,
            }]
        };

        let result = ledger.contains(2);
        assert_eq!(true, result);
    }

}