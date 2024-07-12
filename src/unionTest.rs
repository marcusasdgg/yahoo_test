use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Deserialize, Debug)]
#[allow(dead_code, non_snake_case)]
pub struct StockStruct {
    language: String,
    region: String,
    typeDisp: String,
    triggerable: bool,
    customPriceAlertConfidence: String,
    currency: String,
    marketState: String,
    regularMarketChangePercent: f64,
    regularMarketPrice: f64,
    epsForward: f32,
    epsCurrentYear: f32,
    priceEpsCurrentYear: f64,
    sharesOutstanding: f64,
    bookValue: f64,
    fiftyDayAverage: f64,
    fiftyDayAverageChange: f64,
    fiftyDayAverageChangePercent: f64,
    twoHundredDayAverage: f64,
    twoHundredDayAverageChange: f64,
    twoHundredDayAverageChangePercent: f64,
    marketCap: i64,
    forwardPE: f64,
    priceToBook: f64,
    sourceInterval: f64,
    exchangeDataDelayedBy: i32,
    averageAnalystRating: String,
    tradeable: bool,
    cryptoTradeable: bool,
    exchange: String,
    shortName: String,
    longName: String,
    messageBoardId: String,
    exchangeTimezoneName: String,
    market: String,
    hasPrePostMarketData: bool,
    firstTradeDateMilliseconds: i64,
    priceHint: i8,
    postMarketChangePercent: f64,
    postMarketTime: i64,
    postMarketPrice: f32,
    postMarketChange: f32,
    regularMarketChange: f32,
    regularMarketTime: f32,
    regularMarketDayHigh: f32,
    regularMarketDayRange: String,
    regularMarketDayLow: f32,
    regularMarketVolume: i64,
    regularMarketPreviousClose: f64,
    bid: f32,
    ask: f32,
    bidSize: i32,
    askSize: i32,
    fullExchangeName: String,
    financialCurrency: String,
    regularMarketOpen: f64,
    averageDailyVolume3Month: i64,
    averageDailyVolume10Day: i64,
    fiftyTwoWeekLowChange: f64,
    fiftyTwoWeekLowChangePercent: f64,
    fiftyTwoWeekRange: String,
    fiftyTwoWeekHighChange: f64,
    fiftyTwoWeekHighChangePercent: f64,
    fiftyTwoWeekLow: f64,
    fiftyTwoWeekHigh: f64,
    fiftyTwoWeekChangePercent: f64,
    dividendDate: Option<i64>,
    earningsTimestamp: i64,
    earningsTimestampStart: Option<i64>,
    earningsTimestampEnd: Option<i64>,
    earningsCallTimestampStart: Option<i64>,
    earningsCallTimestampEnd: Option<i64>,
    isEarningsDateEstimate: bool,
    trailingAnnualDividendRate: f32,
    trailingPE: f64,
    dividendRate: Option<f32>,
    trailingAnnualDividendYield: f64,
    dividendYield: Option<f64>,
    epsTrailingTwelveMonths: f64,
    exchangeTimezoneShortName: String,
    gmtOffSetMilliseconds: i64,
    esgPopulated: bool,
    displayName: String,
    symbol: String,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, dead_code)]
pub struct OptionStruct {
    language: String,
    region: String,
    typeDisp: String,
    quoteSourceName: Option<String>,
    triggerable: bool,
    customPriceAlertConfidence: String,
    currency: String,
    regularMarketChangePercent: f64,
    regularMarketPrice: f64,
    underlyingSymbol: String,
    exchange: String,
    shortName: String,
    longName: Option<String>,
    exchangeTimezoneName: String,
    exchangeTimezoneShortName: String,
    gmtOffSetMilliseconds: i64,
    market: String,
    esgPopulated: bool,
    strike: f32,
    openInterest: i32,
    optionsType: String,
    underlyingShortName: String,
    expireDate: i64,
    expireIsoDate: String,
    sourceInterval: i64,
    exchangeDataDelayedBy: i64,
    tradeable: bool,
    cryptoTradeable: bool,
    marketState: String,
    hasPrePostMarketData: bool,
    firstTradeDateMilliseconds: Option<i64>,
    priceHint: i32,
    regularMarketChange: f32,
    regularMarketTime: f32,
    regularMarketDayHigh: f32,
    regularMarketDayRange: String,
    regularMarketDayLow: f32,
    regularMarketVolume: Option<i64>,
    regularMarketPreviousClose: f64,
    bid: f64,
    ask: f64,
    fullExchangeName: String,
    regularMarketOpen: Option<f64>,
    fiftyTwoWeekLowChange: f64,
    fiftyTwoWeekLowChangePercent: f64,
    fiftyTwoWeekRange: String,
    fiftyTwoWeekHighChange: f64,
    fiftyTwoWeekHighChangePercent: f64,
    fiftyTwoWeekLow: f64,
    fiftyTwoWeekHigh: f64,
    symbol: String,
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
