use crate::state::{Wallet, Config, Token};
use std::fs::File;
use std::collections::HashMap;
use std::io::Write;
use serde_json;


fn save_config(config: &Config, filename: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(config)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn create_config_file(
    walletAddress: String, privateKey: String, lamports_w: u64, _tokens_vec: Vec<Token>, 
    transactionFee: f64, rpc_w: String, hashedPassword: String, accountName: String
) {

    let mut accounts = HashMap::new();

    let mut tokens_vec: Vec<Token> = Vec::new();

    tokens_vec.push( Token {
        name: "Bitcoin".to_string(),
        address: "btc_address".to_string(),
        amount: 500,
        decimals: 8,
    });

    let wallet = Wallet {
        address: walletAddress,
        private_key: privateKey,
        lamports: lamports_w,
        tokens: tokens_vec,
        transaction_fee: transactionFee,
        rpc: rpc_w,
    };

    accounts.insert(accountName.to_string(), wallet);

    let config = Config {
        account_password: hashedPassword,
        accounts_by_id: accounts,
    };

    save_config(&config, "config.json").expect("Failed to save config file");
}