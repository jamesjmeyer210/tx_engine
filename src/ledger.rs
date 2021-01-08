use csv::{Reader, DeserializeRecordsIntoIter};
use std::fs::File;
use crate::transaction::SerialTransaction;

pub fn read_csv(buffer: &mut Reader<File>) -> () {
    for result in buffer.deserialize() {
        let tx: SerialTransaction = result.unwrap();
        println!("{:?}", tx)
    }

    //.for_byte_line(|line| println!("{}", line))
}