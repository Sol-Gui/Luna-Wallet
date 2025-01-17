use solana_sdk::{signature::{Keypair}, pubkey::Pubkey};
use bs58;

pub fn create_new_wallet() {
    let keypair = Keypair::new();
    let pubkey: Pubkey = keypair.pubkey();
    let full_keypair_bytes = keypair.to_bytes();

    let full_private_key = bs58::encode(&full_keypair_bytes).into_string();

    println!("Your new public key is: {}", pubkey);
    println!("Your new private key is: {}", full_private_key);
}