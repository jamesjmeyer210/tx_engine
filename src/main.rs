use std::env;
use std::io;
use tx_engine::Ledger;
use std::convert::TryFrom;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file_name) => {
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(b',')
                .flexible(true)
                .from_path(file_name)?;

            let ledger = Ledger::try_from(&mut reader).unwrap();
            Ok(ledger.display())
        },
        _ => Err(io::Error::from(io::ErrorKind::InvalidInput))
    }
}
