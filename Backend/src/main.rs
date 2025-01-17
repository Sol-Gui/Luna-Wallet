mod utils;
mod state;
mod wallet;

fn main() {
    println!("Hello, world!");
    let x = utils::get_env_data();
    println!("Value of X: {:?}", x);

    wallet::create_new_wallet();
}
