use super::ClientId;
use crate::model::Transaction;
use crate::model::TxType;
use crate::TryAdd;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Account {
    pub client: ClientId,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

#[derive(Debug, PartialEq)]
pub enum AccountError {
    InsufficientFunds,
}

impl Account {
    pub fn new() -> Self {
        Account {
            client: 0,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn with_client_id(id: ClientId) -> Self {
        let mut account = Account::new();
        account.client = id;
        account
    }

    fn deposit(&mut self, amount: f64) -> Result<&Self, AccountError> {
        self.available += amount;
        self.total += amount;
        Ok(self)
    }

    fn withdraw(&mut self, amount: f64) -> Result<&Self, AccountError> {
        if self.available < amount {
            Err(AccountError::InsufficientFunds)
        } else {
            self.total -= amount;
            self.available -= amount;
            Ok(self)
        }
    }

    fn dispute(&mut self, amount: f64) -> Result<&Self, AccountError> {
        self.available -= amount;
        self.held += amount;
        Ok(self)
    }

    fn resolve(&mut self, amount: f64) -> Result<&Self, AccountError> {
        self.available += amount;
        self.held -= amount;
        Ok(self)
    }

    fn chargeback(&mut self, amount: f64) -> Result<&Self, AccountError> {
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
        Ok(self)
    }
}

impl TryAdd<&Transaction> for Account {
    type Error = AccountError;

    fn try_add(&mut self, tx: &Transaction) -> Result<&Self, Self::Error> {
        match tx.tx_type {
            TxType::Deposit => self.deposit(tx.amount),
            TxType::Withdraw => self.withdraw(tx.amount),
            TxType::Dispute => self.dispute(tx.amount),
            TxType::Resolve => self.resolve(tx.amount),
            TxType::Chargeback => self.chargeback(tx.amount),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn account_derives_debug() -> () {
        println!("{:?}", Account::with_client_id(77));
    }

    #[test]
    fn try_add_returns_ok_on_deposit() -> Result<(), AccountError> {
        let mut account = Account::new();

        let tx = Transaction {
            tx_type: TxType::Deposit,
            client: 0,
            tx: 0,
            amount: 2.3,
        };

        let result = account.try_add(&tx);
        assert!(result.is_ok());
        let result = result?;
        assert_eq!(2.3, result.available);
        assert_eq!(2.3, result.total);
        Ok(())
    }

    #[test]
    fn withdraw_returns_ok_on_sufficient_funds() -> Result<(), AccountError> {
        let mut account = Account::new();
        account.total = 99.0001;
        account.available = 99.0001;

        let result = account.withdraw(33.0001);
        assert!(result.is_ok());
        let result = result?;
        assert_eq!(66.0, result.available);
        assert_eq!(66.0, result.total);
        Ok(())
    }

    #[test]
    fn withdraw_returns_error_when_available_funds_are_insufficient() {
        let mut account = Account::new();
        account.total = 99.0001;
        account.available = 99.0001;

        let result = account.withdraw(99.0002);
        assert!(result.is_err());
        result
            .map_err(|o| {
                assert_eq!(AccountError::InsufficientFunds, o);
            })
            .unwrap_err()
    }

    #[test]
    fn dispute_decreases_available_and_increases_held() -> Result<(), AccountError> {
        let mut account = Account::new();
        account.available = 42.0024;
        account.held = 24.0042;

        let result = account.dispute(10.0)?;
        assert_eq!(32.0024, result.available);
        assert_eq!(34.0042, result.held);
        Ok(())
    }

    #[test]
    fn resolve_increase_available_and_decreases_held() -> Result<(), AccountError> {
        let mut account = Account::new();
        account.available = 42.0024;
        account.held = 24.0042;

        let result = account.resolve(10.0)?;
        assert_eq!(52.0024, result.available);
        assert_eq!(14.0042, result.held);
        Ok(())
    }

    #[test]
    fn chargeback_decreases_total_and_held_and_locks_account() -> Result<(), AccountError> {
        let mut account = Account::new();
        account.total = 23.5711;
        account.held = 3.35;

        let result = account.chargeback(2.0)?;
        assert_eq!(21.5711, result.total);
        assert_eq!(1.35, result.held);
        assert_eq!(true, result.locked);
        Ok(())
    }
}
