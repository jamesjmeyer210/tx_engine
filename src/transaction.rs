use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    tx_type: String,
    client: u16,
    tx: u32,
    amount: f32,
}