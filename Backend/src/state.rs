use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Token {
    pub name: String,
    pub address: String,
    pub amount: u64,
    pub decimals: u8,
}


#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Wallet {
    pub address: String,
    pub private_key: String,
    pub lamports: u64,
    pub tokens: Vec<Token>,
    pub transaction_fee: f64,
    pub rpc: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub account_password: String,
    pub accounts_by_id: HashMap<String, Wallet>,
}