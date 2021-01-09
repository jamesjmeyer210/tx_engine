use std::env;
use std::io;
use tx_engine::Ledger;
use std::convert::TryFrom;
use csv::Trim;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(file_name) => {
            let mut reader = csv::ReaderBuilder::new()
                .delimiter(b',')
                .trim(Trim::All)
                .flexible(true)
                .from_path(file_name)?;

            let mut ledger = Ledger::try_from(&mut reader).unwrap();
            ledger.display().unwrap();
            Ok(())
        },
        _ => Err(io::Error::from(io::ErrorKind::InvalidInput))
    }
}
