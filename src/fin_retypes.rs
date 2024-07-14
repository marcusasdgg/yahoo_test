use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Deserialize, Debug)]
#[allow(dead_code, non_snake_case)]
pub struct StockStruct {
    pub language: String,
    pub region: String,
    pub typeDisp: String,
    pub triggerable: bool,
    pub customPriceAlertConfidence: String,
    pub currency: String,
    pub marketState: String,
    pub regularMarketChangePercent: f64,
    pub regularMarketPrice: f64,
    pub epsForward: Option<f32>,
    pub epsCurrentYear: Option<f32>,
    pub priceEpsCurrentYear: Option<f64>,
    pub sharesOutstanding: f64,
    pub bookValue: f64,
    pub fiftyDayAverage: f64,
    pub fiftyDayAverageChange: f64,
    pub fiftyDayAverageChangePercent: f64,
    pub twoHundredDayAverage: f64,
    pub twoHundredDayAverageChange: f64,
    pub twoHundredDayAverageChangePercent: f64,
    pub marketCap: i64,
    pub forwardPE: Option<f64>,
    pub priceToBook: f64,
    pub sourceInterval: f64,
    pub exchangeDataDelayedBy: i32,
    pub averageAnalystRating: Option<String>,
    pub tradeable: bool,
    pub cryptoTradeable: bool,
    pub exchange: String,
    pub shortName: String,
    pub longName: String,
    pub messageBoardId: String,
    pub exchangeTimezoneName: String,
    pub market: String,
    pub hasPrePostMarketData: bool,
    pub firstTradeDateMilliseconds: i64,
    pub priceHint: i8,
    pub postMarketChangePercent: Option<f64>,
    pub postMarketTime: Option<i64>,
    pub postMarketPrice: Option<f32>,
    pub postMarketChange: Option<f32>,
    pub regularMarketChange: f32,
    pub regularMarketTime: f32,
    pub regularMarketDayHigh: f32,
    pub regularMarketDayRange: String,
    pub regularMarketDayLow: f32,
    pub regularMarketVolume: i64,
    pub regularMarketPreviousClose: f64,
    pub bid: f32,
    pub ask: f32,
    pub bidSize: i32,
    pub askSize: i32,
    pub fullExchangeName: String,
    pub financialCurrency: String,
    pub regularMarketOpen: f64,
    pub averageDailyVolume3Month: i64,
    pub averageDailyVolume10Day: i64,
    pub fiftyTwoWeekLowChange: f64,
    pub fiftyTwoWeekLowChangePercent: f64,
    pub fiftyTwoWeekRange: String,
    pub fiftyTwoWeekHighChange: f64,
    pub fiftyTwoWeekHighChangePercent: f64,
    pub fiftyTwoWeekLow: f64,
    pub fiftyTwoWeekHigh: f64,
    pub fiftyTwoWeekChangePercent: f64,
    pub dividendDate: Option<i64>,
    pub earningsTimestamp: Option<i64>,
    pub earningsTimestampStart: Option<i64>,
    pub earningsTimestampEnd: Option<i64>,
    pub earningsCallTimestampStart: Option<i64>,
    pub earningsCallTimestampEnd: Option<i64>,
    pub isEarningsDateEstimate: bool,
    pub trailingAnnualDividendRate: f32,
    pub trailingPE: Option<f64>,
    pub dividendRate: Option<f32>,
    pub trailingAnnualDividendYield: f64,
    pub dividendYield: Option<f64>,
    pub epsTrailingTwelveMonths: f64,
    pub exchangeTimezoneShortName: String,
    pub gmtOffSetMilliseconds: i64,
    pub esgPopulated: bool,
    pub displayName: Option<String>,
    pub symbol: String,
    pub quoteSourceName: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct OptionStruct {
    pub language: String,
    pub region: String,
    pub typeDisp: String,
    pub quoteSourceName: Option<String>,
    pub triggerable: bool,
    pub customPriceAlertConfidence: String,
    pub currency: String,
    pub regularMarketChangePercent: f64,
    pub regularMarketPrice: f64,
    pub underlyingSymbol: String,
    pub exchange: String,
    pub shortName: String,
    pub longName: Option<String>,
    pub exchangeTimezoneName: String,
    pub exchangeTimezoneShortName: String,
    pub gmtOffSetMilliseconds: i64,
    pub market: String,
    pub esgPopulated: bool,
    pub strike: f32,
    pub openInterest: i32,
    pub optionsType: String,
    pub underlyingShortName: String,
    pub expireDate: i64,
    pub expireIsoDate: String,
    pub sourceInterval: i64,
    pub exchangeDataDelayedBy: i64,
    pub tradeable: bool,
    pub cryptoTradeable: bool,
    pub marketState: String,
    pub hasPrePostMarketData: bool,
    pub firstTradeDateMilliseconds: Option<i64>,
    pub priceHint: i32,
    pub regularMarketChange: f32,
    pub regularMarketTime: f32,
    pub regularMarketDayHigh: f32,
    pub regularMarketDayRange: String,
    pub regularMarketDayLow: f32,
    pub regularMarketVolume: Option<i64>,
    pub regularMarketPreviousClose: f64,
    pub bid: f64,
    pub ask: f64,
    pub fullExchangeName: String,
    pub regularMarketOpen: Option<f64>,
    pub fiftyTwoWeekLowChange: f64,
    pub fiftyTwoWeekLowChangePercent: f64,
    pub fiftyTwoWeekRange: String,
    pub fiftyTwoWeekHighChange: f64,
    pub fiftyTwoWeekHighChangePercent: f64,
    pub fiftyTwoWeekLow: f64,
    pub fiftyTwoWeekHigh: f64,
    pub symbol: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "quoteType")]
pub enum FinResult {
    EQUITY(StockStruct),
    OPTION(OptionStruct),
}

impl FinResult {
    pub fn new(data: &str) -> Result<Vec<Self>, String> {
        let json_obj: Value = serde_json::from_str(data).unwrap();
        let json_obj = json_obj.get("quoteResponse").unwrap();
        let pot_error = json_obj.get("error");

        if pot_error.is_none() {
            return Err(pot_error.unwrap().to_string());
        }

        let result = json_obj.get("result").unwrap().as_array().unwrap();

        let arr = result
            .iter()
            .map(|ob| {
                let run: FinResult = serde_json::from_value(ob.to_owned()).unwrap();
                return run;
            })
            .collect();

        return Ok(arr);
    }
}
