use reqwest::header::{HeaderValue, Iter, USER_AGENT};
use tokio::main;
use reqwest::*;
use std::time::{self, Duration};
use std::time::Instant;
use std::string;
use reqwest::header::COOKIE;

mod supp;
use supp::YAHOOCONNECT;

#[tokio::main]
async fn main() -> Result<()>
{   
    //first things first define a url
    let client = YAHOOCONNECT::new().await?;
    let s = client.get_ticker("AAPL").await?;
    println!("{:?}",s);
    Ok(())
}

//add some fake headers to fake some basic info like browser.