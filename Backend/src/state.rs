#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub lamports: f64,
    pub fee: f64,
    pub signature: String,
    pub status: String,
    pub block_height: i32,
    pub block_hash: String,
    pub amount: f64,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct Wallet {
    pub id: u8,
    pub balance: f64,
    pub address: String,
    //pub transactions: Vec<Transaction>,
    pub rpc: String,
    pub transaction_fee: f64,
}