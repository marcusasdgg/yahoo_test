mod unionTest;
use yahoo_tick_grabber::YAHOOCONNECT;
mod response;

#[tokio::main]
async fn main() -> Result<(),()> {
    let s = YAHOOCONNECT::new().await.unwrap();
    let guts = s.get_ticker("AAPL,TSLA,TSLA240712C00075000").await.unwrap();
    println!("{}", guts);
    Ok(())
}