extern crate dotenv;
use crate::state::Wallet;

pub fn get_env_data() -> Wallet {
    // Load environment variables from a .env file
    dotenv::from_filename("config.env").ok().expect("Falha ao carregar o arquivo config.env");

    Wallet {
        id: dotenv::var("ID").unwrap().parse::<u8>().unwrap(),
        balance: dotenv::var("BALANCE").unwrap().parse::<f64>().unwrap(),
        address: dotenv::var("ADDRESS").unwrap(),
        rpc: dotenv::var("WALLET_RPC_URL").unwrap(),
        transaction_fee: dotenv::var("TRANSACTION_FEE").unwrap().parse::<f64>().unwrap(),
    }
}
