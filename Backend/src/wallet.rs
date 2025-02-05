use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
use bs58;
use crate::utils::{create_config_file};

pub fn create_new_wallet() {
    let keypair = Keypair::new();
    let pubkey: Pubkey = keypair.try_pubkey().unwrap();
    let full_keypair_bytes = keypair.to_bytes();

    let full_private_key = bs58::encode(&full_keypair_bytes).into_string();

    create_config_file(pubkey.to_string(), full_private_key.to_string().clone(), 0, Vec::new(), 0.0, "http://localhost:8899".to_string(), "123456".to_string(), "Main Account".to_string());

    println!("Your new public key is: {}", pubkey);
    println!("Your new private key is: {}", full_private_key);
}