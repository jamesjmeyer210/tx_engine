use csv::{Reader, DeserializeRecordsIntoIter};
use std::fs::File;
use crate::model::SerialTransaction;
use crate::model::Client;

pub fn read_csv(buffer: &mut Reader<File>) -> () {
    for result in buffer.deserialize() {
        let tx: SerialTransaction = result.unwrap();
        println!("{:?}", tx)
    }
}

struct Ledger {
    clients: Vec<Client>,
}