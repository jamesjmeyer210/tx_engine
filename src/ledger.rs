use csv::{Reader, DeserializeRecordsIntoIter};
use std::fs::File;
use crate::transaction::Transaction;

pub fn read_csv(buffer: &mut Reader<File>) -> () {
    for result in buffer.deserialize() {
        let tx: Transaction = result.unwrap();
        println!("{:?}", tx)
    }

    //.for_byte_line(|line| println!("{}", line))
}