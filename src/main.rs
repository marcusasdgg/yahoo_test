use reqwest::header::COOKIE;
use reqwest::header::USER_AGENT;
use reqwest::*;
use std::string;
use tokio::main;

mod supp;

#[tokio::main]
async fn main() -> Result<()> {
    //first things first define a url.
    let multi_client = reqwest::Client::new();
    let response = multi_client.get("https://fc.yahoo.com");
    let s = response.send().await?;
    let mut i = 1;
    let mut cookie_str = String::new();
    for sr in s.cookies() {
        println!("{i}th cokies are {:?}", sr);
        i += 1;
        cookie_str = format!("{}={}", sr.name(), sr.value());
    }

    println!("cookie is {:?}", cookie_str);
    let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
    let crumb_response = multi_client
        .get("https://query2.finance.yahoo.com/v1/test/getcrumb")
        .header(COOKIE, cookie_str.clone())
        .header(USER_AGENT, user_agent)
        .send()
        .await?
        .text()
        .await?;
    let mut final_get = String::new();
    final_get = format!(
        "{}{}",
        "https://query2.finance.yahoo.com/v7/finance/quote?symbols=TSLA&crumb=", crumb_response
    );
    let get_tsla = multi_client
        .get(final_get)
        .header(COOKIE, cookie_str)
        .header(USER_AGENT, user_agent)
        .send()
        .await?
        .text()
        .await?;

    println!("tesla is rawr {get_tsla}");

    Ok(())
}

//add some fake headers to fake some basic info like browser.
