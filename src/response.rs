use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use serde::Deserializer;
use serde::de::Error;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case,dead_code)]
pub struct QueryResponse {
    pub quoteResponse: QuoteResponse, // nested object
    pub quoteResponse: QuoteResponse, // nested object
}
#[derive(Deserialize, Debug)]
#[allow(non_snake_case,dead_code)]
pub struct QuoteResponse {
	pub result : Vec<TradeResult>,
	pub error : Option<String>,
	pub result : Vec<TradeResult>,
	pub error : Option<String>,
}
#[derive(Deserialize, Debug)]
#[allow(non_snake_case,dead_code)]
pub struct TradeResult {
    pub quoteType: String, 
    pub quoteSourceName: String,
    pub currency: Currency, 
    pub marketState: MarketState,
    pub regularMarketChangePercent: f64, 
    pub regularMarketPrice: f64,
    pub exchange: String,
    pub shortName: String,
    pub longName: String,
    pub exchangeTimezoneName: String,
    pub exchangeTimezoneShortName: String,
    pub gmtOffSetMilliseconds: f64,
    pub market: String,
    pub esgPopulated: bool,
    pub hasPrePostMarketData: bool,
    pub firstTradeDateMilliseconds: f64,
    pub priceHint: i32,
    pub postMarketChangePercent: Option<f64>,
    pub postMarketTime: Option<u64>,
    pub postMarketPrice: Option<f64>,
    pub postMarketChange: Option<f64>,
    pub regularMarketChange: f64,
    pub regularMarketTime: u64,
    pub regularMarketDayHigh: f64,
    pub regularMarketDayRange: String,
    pub regularMarketDayLow: f64,
    pub regularMarketVolume: f64,
    pub regularMarketPreviousClose: f64,
    pub bid: Option<f64>,
    pub ask: f64,
    pub bidSize: Option<f64>,
    pub askSize: Option<f64>,
    pub fullExchangeName: String,
    pub financialCurrency: Option<Currency>,
    pub regularMarketOpen: f64,
    pub averageDailyVolume3Month: Option<u64>,
    pub averageDailyVolume10Day: Option<u64>,
    pub fiftyTwoWeekLowChange: f64,
    pub fiftyTwoWeekLowChangePercent: f64,
    pub fiftyTwoWeekRange: Option<String>,
    pub fiftyTwoWeekHighChange: f64,
    pub fiftyTwoWeekHighChangePercent: f64,
    pub fiftyTwoWeekLow: f64,
    pub fiftyTwoWeekHigh: f64,
    pub fiftyTwoWeekChangePercent: Option<f64>,
    pub earningsTimestamp: Option<u64>,
    pub earningsTimestampStart: Option<u64>,
    pub earningsTimestampEnd: Option<u64>,
    pub trailingAnnualDividendRate: Option<f64>,
    pub trailingPE: Option<f64>,
    pub trailingAnnualDividendYield: Option<f64>,
    pub epsTrailingTwelveMonths: Option<f64>,
    pub epsForward: Option<f64>,
    pub epsCurrentYear: Option<f64>,
    pub priceEpsCurrentYear: Option<f64>,
    pub sharesOutstanding: Option<i64>,
    pub bookValue: Option<f64>,
    pub fiftyDayAverage: Option<f64>,
    pub fiftyDayAverageChange: Option<f64>,
    pub fiftyDayAverageChangePercent: Option<f64>,
    pub twoHundredDayAverage: Option<f64>,
    pub twoHundredDayAverageChange: Option<f64>,
    pub twoHundredDayAverageChangePercent: Option<f64>,
    pub marketCap: Option<f64>,
    pub forwardPE: Option<f64>,
    pub priceToBook: Option<f64>,
    pub sourceInterval: Option<f64>,
    pub exchangeDataDelayedBy: f64,
    pub averageAnalystRating: Option<String>,
    pub displayName: Option<String>,
    pub symbol: Option<String>,
    pub language: String,
    pub region: String,
    pub typeDisp: String,
    pub triggerable: bool,
    pub customPriceAlertConfidence: String,
    pub messageBoardId: Option<String>,
    pub isEarningsDateEstimate: Option<bool>,
    pub tradeable: bool,
    pub cryptoTradeable: bool,
    pub dividendDate: Option<i128>,
    pub dividendRate: Option<i128>,
    pub dividendYield: Option<i64>,
    pub underlyingSymbol: Option<String>,
    pub strike: Option<f64>,
    pub openInterest: Option<f64>,
    pub optionsType: Option<OptionsType>,
    pub underlyingShortName: Option<String>,
    pub expireDate: Option<i64>,
    #[serde(deserialize_with = "deserialize_date")]
    pub expireIsoDate: Option<Timestamp>,
    pub circulatingSupply: Option<u128>,
    pub lastMarket: Option<String>,
    pub volume24Hr: Option<i128>,
    pub volumeAllCurrencies: Option<i128>,
    pub fromCurrency: Option<String>,
    pub toCurrency: Option<String>,
    pub coinMarketCapLink: Option<String>,
    pub startDate: Option<u64>,
    pub coinImageUrl: Option<String>,
    pub logoUrl: Option<String>,
}










#[derive(Debug)]

pub struct Timestamp {
	second : u8,
	minute : u8,
	hour : u8,
	day : u8,
	month : u8,
	year : u16,
	unixstamp : u64
}




//list enums below yasss queen
#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub enum Currency {
	USD,
	AUD,
	HKD,
	CNY,
	SGD
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub enum MarketState {
	PREPRE,
	POSTPOST,
	PRE,
	CLOSED,
	REGULAR
}

// #[derive(Serialize, Deserialize, Debug)]
// #[allow(dead_code)]
// enum QuoteType {
// 	Equity,
// 	Option,
// 	Bond,
// 	Future
// }

#[derive(Debug,Serialize, Deserialize)]
pub enum OptionsType{
	Call,  
	Put
}



fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<Timestamp>, D::Error> //turn all other 'normal timestamps into a Timestamp type'
where
    D: Deserializer<'de>,
{
	let timestamp = String::deserialize(deserializer).unwrap();
	if timestamp.len() != 20
	{
		return Err(D::Error::custom("Invalid timestamp length"));
	}
	let year = timestamp[0..4].parse::<u16>().map_err(D::Error::custom)?;
    let month = timestamp[5..7].parse::<u8>().map_err(D::Error::custom)?;
    let day = timestamp[8..10].parse::<u8>().map_err(D::Error::custom)?;
    let hour = timestamp[11..13].parse::<u8>().map_err(D::Error::custom)?;
    let minute = timestamp[14..16].parse::<u8>().map_err(D::Error::custom)?;
    let second = timestamp[17..19].parse::<u8>().map_err(D::Error::custom)?;
	
	let date = chrono::NaiveDate::from_ymd_opt(year.into(), month.into(), day.into())
	.ok_or_else(|| D::Error::custom("Invalid date"))?;
	let time = chrono::NaiveTime::from_hms_opt(hour.into(), minute.into(), second.into())
	.ok_or_else(|| D::Error::custom("Invalid time"))?;

	let datetime = NaiveDateTime::new(date,time);

	let unixstamp: u64 = datetime.and_utc().timestamp().try_into().unwrap();
	let timestamp = Timestamp {second,minute,hour,day,month,year,unixstamp};
	Ok(Some(timestamp))
}

//for a newer release make this more space efficeint, i.e different quote types 
//get different structs.