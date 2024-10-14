
use ethers::prelude::*;
use mev_arbitrage_bot::utils::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::new().await;
    println!("[STARTING]");

    let mut stream = config.wss.subscribe_blocks().await?;
    while let Some(block) = stream.next().await {
       println!("[BLOCK NUMBER] - {:?}", block.number.unwrap_or_default());
    }
    Ok(())
}
