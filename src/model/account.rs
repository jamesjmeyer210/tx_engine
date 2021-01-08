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

impl Account {
    pub fn new() -> Account {
        Account {
            client: 0,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}

impl TryAdd<&Transaction> for Account {
    type Error = ();

    fn try_add(&mut self, tx: &Transaction) -> Result<&Self, Self::Error> {
        match tx.tx_type {
            TxType::Deposit => {
                self.total += tx.amount;
                Ok(self)
            },
            _ => Ok(self),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn try_add_returns_ok_on_deposit() {
        let mut account = Account::new();

        let tx = Transaction {
            tx_type: TxType::Deposit,
            client: 0,
            tx: 0,
            amount: 2.3
        };

        let result = account.try_add(&tx);
        assert!(result.is_ok());
        assert_eq!(2.3, result.unwrap().total);
    }
}