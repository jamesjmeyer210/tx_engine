mod model;
mod ledger;

trait TryAdd<T> {
    type Error;

    fn try_add(&mut self, _: T) -> Result<&Self,Self::Error>;
}

trait Contains<T> {
    fn contains(&self, target: T) -> bool;
}

trait FindBy<T> {
    fn find_by(&self, target: T) -> Option<usize>;
}

pub type Ledger = ledger::Ledger;