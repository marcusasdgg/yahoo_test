
use yahoo_tick_grabber::YAHOOCONNECT;
mod response;
use response::{QueryResponse};

#[tokio::main]
async fn main() -> Result<(),()> {
    let s = YAHOOCONNECT::new().await.unwrap();
    let guts = s.get_ticker("TSLA").await.unwrap();

    println!("{:?}",guts);

    Ok(())
}