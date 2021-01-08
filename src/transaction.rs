use serde::{Deserialize, Deserializer, de};
use serde::de::Visitor;
use std::fmt;
use std::convert::TryFrom;

#[derive(Debug, Deserialize)]
pub struct SerialTransaction {
    #[serde(rename = "type")]
    tx_type: String,
    client: u16,
    tx: u32,
    amount: f32,
}

pub struct Transaction {
    tx_type: TxType,
    client: u16,
    tx: u32,
    amount: f32,
}

impl TryFrom<SerialTransaction> for Transaction {
    type Error = ();

    fn try_from(value: SerialTransaction) -> Result<Self, Self::Error> {
        let tx_type = TxType::try_from(&*value.tx_type)?;

        Ok(Transaction {
            tx_type,
            client: value.client,
            tx: value.tx,
            amount: value.amount,
        })
    }
}

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