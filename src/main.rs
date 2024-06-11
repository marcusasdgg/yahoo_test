
use yahoo_tick_grabber::YAHOOCONNECT;
mod response;
use response::{QueryResponse};

#[tokio::main]
async fn main() -> Result<(),()> {
    let s = YAHOOCONNECT::new().await.unwrap();
    let guts: String = s.get_ticker("TSLA240607C00075000").await.unwrap();
    let strc : QueryResponse = serde_json::from_str(&guts).unwrap();
    println!("{:?}",strc);

    Ok(())
}