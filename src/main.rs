use ethers::providers::{Provider, Http};
use ethers::{utils, prelude::*};
use ethers_core::rand::thread_rng;

async fn get_accounts() {
    let provider = Provider::<Http>::try_from("https://goerli.infura.io/v3/16087e2d4b5247d589382c9038b12f12").unwrap();
    let accounts = provider.get_accounts().await.unwrap();

    println!("{:?}", accounts)
}

#[tokio::main]
async fn main() {
    get_accounts().await;
}
