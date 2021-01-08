use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SerialTransaction {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: f32,
}