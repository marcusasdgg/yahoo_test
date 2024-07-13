use yahoo_tick_grabber::{fin_retypes::FinResult, YAHOOCONNECT};
#[tokio::main]
async fn main() -> Result<(), ()> {
    let s = YAHOOCONNECT::new().await.unwrap();
    let guts = s
        .get_ticker("NVDA,TSLA240719C00050000,AAPL,RTX,nwm.AX,CXO.ax")
        .await
        .unwrap();

    for b in guts {
        match b {
            FinResult::EQUITY(_) => println!("equity found"),
            FinResult::OPTION(_) => println!("option found"),
        }
    }

    Ok(())
}
