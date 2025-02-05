mod state;
mod wallet;
mod password;
pub mod utils;

fn main() {
    println!("Hello, world!");

    wallet::create_new_wallet();
    password::set_password("123456".to_string());
}