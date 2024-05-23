use reqwest::header::{HeaderValue, Iter};
use tokio::main;
use reqwest::*;
use std::time::{self, Duration};
use std::time::Instant;
use std::string;
use reqwest::header::COOKIE;
#[tokio::main]
async fn main() -> Result<()>
{   
    //first things first define a url.
    let multi_client = reqwest::Client::new();
    let response = multi_client.get("https://fc.yahoo.com");
    let s = response.send().await?;
    let t : Vec<_> = s.headers().into_iter().take(s.headers().len()).collect();
    let mut s = String::new();
    for lines in t {
       if lines.0 == "set-cookie"
       {
            let str = lines.1.to_str().unwrap();
            for i in str.chars()
            {
                if i == ';'{
                    break;
                }
                s.push(i);
            }
       }
    }


    //got the cookie yay.
    println!("cookie found is {s}");

    std::thread::sleep(Duration::from_secs(2));
    let mut get_crumb = multi_client.get("https://query2.finance.yahoo.com/v1/test/getcrumb");
    get_crumb = get_crumb.header(COOKIE,s);
    let response2 = get_crumb.send().await?.text().await?;
    println!("response is {:?}",response2);

    Ok(())
}

//add some fake headers to fake some basic info like browser.