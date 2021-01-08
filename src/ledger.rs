use csv::{Reader, DeserializeRecordsIntoIter};
use std::fs::File;
use std::convert::TryFrom;
use crate::model::{SerialTransaction, Transaction};
use crate::model::Account;

pub struct Ledger {
    clients: Vec<Account>,
}

impl TryFrom<&mut Reader<File>> for Ledger {
    type Error = ();

    fn try_from(buffer: &mut Reader<File>) -> Result<Self, Self::Error> {
        for result in buffer.deserialize() {
            let tx: SerialTransaction = result.unwrap();
            let tx = Transaction::try_from(tx)?;
        }

        Ok(Ledger {
            clients: Vec::new()
        })
    }
}