use reqwest::header::COOKIE;
use reqwest::header::USER_AGENT;

use std::borrow::Borrow;

use reqwest::Result;

use std::sync::Arc;
use tokio::sync::RwLock;

pub mod fin_retypes;
use fin_retypes::FinResult;

pub struct YAHOOCONNECT {
    multiclient: reqwest::Client,
    cookie: Arc<RwLock<String>>,
    crumb: Arc<RwLock<String>>,
    cookie_url: Arc<String>,
    crumb_url: Arc<String>,
    user_agent: Arc<String>,
}

impl YAHOOCONNECT {
    pub async fn new() -> Result<YAHOOCONNECT> {
        let multiclient = reqwest::Client::new();
        let cookie = Arc::new(RwLock::new(String::new()));
        let crumb = Arc::new(RwLock::new(String::new()));
        let cookie_url = Arc::new(String::from("https://fc.yahoo.com"));
        let crumb_url = Arc::new(String::from(
            "https://query2.finance.yahoo.com/v7/finance/quote?symbols=",
        ));
        let user_agent = Arc::new(String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));
        let te = YAHOOCONNECT {
            multiclient,
            cookie,
            crumb,
            cookie_url,
            crumb_url,
            user_agent,
        };
        YAHOOCONNECT::update_crumb_n_cookie(te.borrow()).await?;
        return Ok(te);
    }

    async fn update_crumb_n_cookie(&self) -> Result<()> {
        // get cookie first from link.
        let response = self
            .multiclient
            .get(self.cookie_url.as_str())
            .send()
            .await?;
        let mut cookie_str = String::new();
        for sr in response.cookies() {
            cookie_str = format!("{}={}", sr.name(), sr.value());
        }

        {
            let mut re = self.cookie.write().await;
            *re = cookie_str.clone();
        }

        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
        let crumb_response = self
            .multiclient
            .get("https://query2.finance.yahoo.com/v1/test/getcrumb")
            .header(COOKIE, cookie_str)
            .header(USER_AGENT, user_agent)
            .send()
            .await?
            .text()
            .await?;

        {
            let mut re = self.crumb.write().await;
            *re = crumb_response;
        }

        Ok(())
    } //if update doesnt work, return error with each step.

    pub async fn get_ticker(&self, lame: &str) -> std::result::Result<Vec<FinResult>, String> {
        println!("getting ticker \"{}\"", lame);
        let query = self.get_tic_internal(lame).await.unwrap();
        if query.contains("quoteResponse\":{\"") {
            return FinResult::new(&query);
        }
        if query.contains("Invalid Cookie") || query.contains("Invalid Crumb") {
            self.update_crumb_n_cookie().await.unwrap();
            let queryretry = FinResult::new(&self.get_tic_internal(lame).await.unwrap())?;
            return Ok(queryretry);
        } else {
            return Err("Error Searching for a ticker".to_string());
        }
    }

    //test //we read the error, if it is
    // revamp the entire function according to iterator

    //add error checking in this function.
    async fn get_tic_internal(&self, name: &str) -> Result<String> {
        let final_get = format!(
            "{}{}&crumb={}",
            self.crumb_url.as_str(),
            name,
            self.crumb.read().await.as_str()
        );
        let ticker_info = self
            .multiclient
            .get(final_get)
            .header(COOKIE, self.cookie.read().await.as_str())
            .header(USER_AGENT, self.user_agent.as_str())
            .send()
            .await?
            .text()
            .await?;

        return Ok(ticker_info);
    } //crumb/cookie related udpate
      //that.
}

//error checking added, need to update version.

//botched error checking, minor change needed to correctly return error when no stock found with ticker.

//add serialization and ability to switch between exchange
