use super::SerialTransaction;
use std::convert::TryFrom;

pub enum TxType {
    Deposit,
    Withdraw,
    Dispute,
    Resolve,
    Chargeback,
}

impl TryFrom<&str> for TxType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "deposit" => Ok(TxType::Deposit),
            "withdraw" | "withdrawal" => Ok(TxType::Withdraw),
            "dispute" => Ok(TxType::Dispute),
            "resolve" => Ok(TxType::Resolve),
            "chargeback" => Ok(TxType::Chargeback),
            _ => Err(()),
        }
    }
}

pub type ClientId = u16;
pub type TxId = u32;

pub struct Transaction {
    pub tx_type: TxType,
    pub client: ClientId,
    pub tx: TxId,
    pub amount: f32,
}

impl TryFrom<SerialTransaction> for Transaction {
    type Error = ();

    fn try_from(value: SerialTransaction) -> Result<Self, Self::Error> {
        let tx_type = TxType::try_from(&*value.tx_type)?;

        if 0.0 > value.amount {
            Err(())
        } else {
            Ok(Transaction {
                tx_type,
                client: value.client,
                tx: value.tx,
                amount: value.amount,
            })
        }
    }
}