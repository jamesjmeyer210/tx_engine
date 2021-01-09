use std::env;
use std::io;
use std::fs::File;
use tx_engine::Ledger;
use std::convert::TryFrom;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file_name) => {
            let mut reader = csv::Reader::from_path(file_name)?;
            let ledger = Ledger::try_from(&mut reader).unwrap();
            ledger.display();
            Ok(())
        },
        _ => Err(io::Error::from(io::ErrorKind::InvalidInput))
    }
}
