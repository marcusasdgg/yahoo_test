use reqwest::header::COOKIE;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use reqwest::Result;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

pub struct YAHOOCONNECT {
    multiclient: reqwest::Client,
    cookie: Arc<RwLock<String>>,
    crumb: Arc<RwLock<String>>,
    cookie_url: Arc<String>,
    crumb_url: Arc<String>,
    user_agent: Arc<String>
}

impl YAHOOCONNECT {
    pub async fn new() -> Result<YAHOOCONNECT> {
        let multiclient = reqwest::Client::new();
        let cookie = Arc::new(RwLock::new(String::new()));
        let crumb = Arc::new(RwLock::new(String::new()));
        let cookie_url = Arc::new(String::from("https://fc.yahoo.com"));
        let crumb_url = Arc::new(String::from("https://query2.finance.yahoo.com/v7/finance/quote?symbols="));
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

    async fn update_crumb_n_cookie(&self) -> Result<()>  {
        // get cookie first from link.
        let response = self.multiclient.get("https://fc.yahoo.com").send().await?;
        let mut cookie_str = String::new();
        for sr in response.cookies()
        {
            cookie_str = format!("{}={}", sr.name(), sr.value());
        }

        {
            let mut re = self.cookie.write().unwrap();
            *re = cookie_str.clone();
        }

        let user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
        let crumb_response = self.multiclient
        .get("https://query2.finance.yahoo.com/v1/test/getcrumb")
        .header(COOKIE, cookie_str)
        .header(USER_AGENT, user_agent)
        .send()
        .await?.text().await?;

        {
            let mut re = self.crumb.write().unwrap();
            *re = crumb_response;
        }

        Ok(())
    } //if update doesnt work, return error with each step.

    pub async fn get_ticker(&self, name: &str) -> std::result::Result<String, String> {
        let ticker_info = self.get_tic_internal(name).await.unwrap();
        if !ticker_info.contains(name)
        {
            self.update_crumb_n_cookie().await.unwrap();
            let ticker_info = self.get_tic_internal(name).await.unwrap();
            if !ticker_info.contains(name)
            {
                return Err("Urls probably got fixed".to_string());
            } else {
                return Ok(ticker_info);
            }
        }

        return Ok(ticker_info);
        }
        
        //test //we read the error, if it is
    
    async fn get_tic_internal(&self,name: &str) -> Result<String>
    {
        let mut final_get = String::new();
        final_get = format!("{}{}&crumb={}",self.crumb_url.as_str(),name,self.crumb.read().unwrap().as_str());
        let ticker_info = self.multiclient.get(final_get)
        .header(COOKIE, self.cookie.read().unwrap().as_str())
        .header(USER_AGENT, self.user_agent.as_str())
        .send()
        .await?
        .text()
        .await?;
        
        return Ok(ticker_info);
    }                                                            //crumb/cookie related udpate
                                                                     //that.
}
