use super::ClientId;
use crate::TryAdd;
use crate::model::Transaction;
use crate::model::TxType;

pub struct Account {
    pub client: ClientId,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

pub type AccountError = &'static str;

static INSUFFICIENT_FUNDS: AccountError = "Insufficient funds";

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

    fn deposit(&mut self, amount: f32) -> Result<&Self,AccountError> {
        self.available += amount;
        self.total += amount;
        Ok(self)
    }

    fn withdraw(&mut self, amount: f32) -> Result<&Self,AccountError> {
        if self.available < amount {
            Err("Insufficient funds")
        }
        else {
            self.total -= amount;
            self.available -= amount;
            Ok(self)
        }
    }

    fn dispute(&mut self, amount: f32) -> Result<&Self,AccountError> {
        self.available -= amount;
        self.held += amount;
        Ok(self)
    }

    fn resolve(&mut self, amount: f32) -> Result<&Self,AccountError> {
        self.available += amount;
        self.held -= amount;
        Ok(self)
    }

    fn chargeback(&mut self, amount: f32) -> Result<&Self,AccountError> {
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
            TxType::Chargeback => self.resolve(tx.amount),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn try_add_returns_ok_on_deposit() -> Result<(),AccountError> {
        let mut account = Account::new();

        let tx = Transaction {
            tx_type: TxType::Deposit,
            client: 0,
            tx: 0,
            amount: 2.3
        };

        let result = account.try_add(&tx);
        assert!(result.is_ok());
        let result = result?;
        assert_eq!(2.3, result.available);
        assert_eq!(2.3, result.total);
        Ok(())
    }

    #[test]
    fn withdraw_returns_ok_on_sufficient_funds() -> Result<(),AccountError> {
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
        result.map_err(|o|{
            assert_eq!(INSUFFICIENT_FUNDS, o);
        });
    }
}