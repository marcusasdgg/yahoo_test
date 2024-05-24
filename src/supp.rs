use reqwest::header::COOKIE;
use reqwest::header::USER_AGENT;
use reqwest::*;
use std::borrow::BorrowMut;
use std::result;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

struct YAHOOCONNECT {
    multiclient: reqwest::Client,
    cookie: RwLock<String>,
    crumb: RwLock<String>,
    cookie_url: Arc<String>,
    crumb_url: Arc<String>,
}

impl YAHOOCONNECT {
    fn new() -> std::result::Result<YAHOOCONNECT, &str> {
        let multiclient = reqwest::Client::new();
        let cookie = RwLock::new(String::new());
        let crumb = RwLock::new(String::new());
        let cookie_url = String::from_str("hi");
        let crumb_url = String::from_str("hi");

        let mut te = YAHOOCONNECT {
            multiclient,
            cookie,
            crumb,
            cookie_url,
            crumb_urlm,
        };
        YAHOOCONNECT::update_crumb_n_cookie(te.borrow_mut());
        return Arc::new(te);
    }

    fn update_crumb_n_cookie(&mut self) -> std::Result<(), &str> {
        Ok(())
    } //if update doesnt work, return error with each step.

    fn get_ticker(&mut self, name: &str) -> std::Result<(), &str> {} //we read the error, if it is
                                                                     //crumb/cookie related udpate
                                                                     //that.
}
