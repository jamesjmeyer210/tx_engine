mod model;
mod ledger;

trait Add<T> {
    fn add(&mut self, n: T) -> ();
}

trait TryAdd<T> {
    type Error;

    fn try_add(&mut self, _: T) -> Result<&Self,Self::Error>;
}

trait Contains<T> {
    fn contains(&self, target: T) -> bool;
}

pub type Ledger = ledger::Ledger;