mod ledger;
mod model;

trait TryAdd<T> {
    type Error;

    fn try_add(&mut self, _: T) -> Result<&Self, Self::Error>;
}

trait Verify<T> {
    type Error;

    fn verify(&self, _: T) -> Result<Option<T>, Self::Error>;
}

trait FindBy<T> {
    fn find_by(&self, target: T) -> Option<usize>;
}

pub type Ledger = ledger::Ledger;
