use std::env;
use std::io;
use std::fs::File;
use tx_engine::ledger;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file_name) => {
            let mut reader = csv::Reader::from_path(file_name)?;
            ledger::read_csv(&mut reader);
            Ok(())
        },
        _ => Err(io::Error::from(io::ErrorKind::InvalidInput))
    }
}
