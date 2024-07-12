mod unionTest;
use unionTest::FinResult;
use yahoo_tick_grabber::YAHOOCONNECT;
#[tokio::main]
async fn main() -> Result<(), ()> {
    let s = YAHOOCONNECT::new().await.unwrap();
    let guts = s.get_ticker("CXO.ax").await.unwrap();
    println!("{}", guts);
    let t = FinResult::new(&guts);
    for b in t.unwrap() {
        println!("{:?}", b);
    }

    Ok(())
}
