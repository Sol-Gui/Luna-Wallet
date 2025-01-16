extern crate dotenv;

use dotenv;
use State::Wallet;

let Wallet = State::Wallet {
    id: "1".to_string(),
    balance: 100.0,
    address: "0x1234567890".to_string(),
    transactions: vec![],
}

pub fn get_current_balance(String Wallet::address) -> usize { 
    
}