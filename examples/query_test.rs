#![allow(unused)]

use ethers::prelude::*;
use mev_arbitrage_bot::{utils::*, dex_factory::*, address_book::*};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::new().await;
    println!("[STARTING]");

    let factory_addresses = vec![
        UNISWAP_FACTORY,
        SUSHISWAP_FACTORY,
    ]
    .into_iter()
    .map(|address| {
        address
            .parse::<Address>()
            .expect("parse factory address failed")
    })
    .collect::<Vec<Address>>();

    let flash_query_address = QUERY_CONTRACT.parse::<Address>().unwrap();
    let flash_query_contract = UniQuery::new(flash_query_address, config.http.clone());
    let grouped_pairs =
        get_markets_by_token(factory_addresses, &flash_query_contract, config.http.clone()).await;

    Ok(())
}