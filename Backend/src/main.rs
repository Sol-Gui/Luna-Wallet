mod utils;
mod state;

fn main() {
    println!("Hello, world!");
    let x = utils::get_env_data();
    println!("Value of X: {:?}", x);
}
