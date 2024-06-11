use serde::{Serialize, Deserialize};
use chrono::{DateTime, NaiveDateTime};
use serde::Deserializer;
use serde::de::Error;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case,dead_code)]
pub struct QueryResponse {
    quoteResponse: QuoteResponse, // nested object
}
#[derive(Deserialize, Debug)]
#[allow(non_snake_case,dead_code)]
pub struct QuoteResponse {
	result : Vec<TradeResult>,
	error : Option<String>,
}
#[derive(Deserialize, Debug)]
#[allow(non_snake_case,dead_code)]
pub struct TradeResult {
	quoteType: String, 
	quoteSourceName: String ,
	currency: Currency , 
	marketState : MarketState ,
	regularMarketChangePercent : f64 , 
	regularMarketPrice : f64 ,
	exchange : String ,
	shortName : String ,
	longName : String ,
	exchangeTimezoneName : String ,
	exchangeTimezoneShortName : String ,
	gmtOffSetMilliseconds : f64 ,
	market : String ,
	esgPopulated : bool ,
	hasPrePostMarketData : bool ,
	firstTradeDateMilliseconds : f64 ,
	priceHint : i32 ,
	postMarketChangePercent : Option<f64> ,
	postMarketTime : Option<u64> ,
	postMarketPrice : Option<f64> ,
	postMarketChange : Option<f64> ,
	regularMarketChange : f64 ,
	regularMarketTime : u64 ,
	regularMarketDayHigh : f64 ,
	regularMarketDayRange : String ,
	regularMarketDayLow : f64 ,
	regularMarketVolume : f64 ,
	regularMarketPreviousClose : f64 ,
	bid : Option<f64> ,
	ask : f64 ,
	bidSize : Option<f64> ,
	askSize : Option<f64> ,
	fullExchangeName : String ,
	financialCurrency : Option<Currency>, 
	regularMarketOpen : f64, 
	averageDailyVolume3Month : Option<u64> ,
	averageDailyVolume10Day : Option<u64> ,
	fiftyTwoWeekLowChange : f64 ,
	fiftyTwoWeekLowChangePercent : f64 ,
	fiftyTwoWeekRange : Option<String> ,
	fiftyTwoWeekHighChange : f64 ,
	fiftyTwoWeekHighChangePercent : f64,
	fiftyTwoWeekLow : f64 ,
	fiftyTwoWeekHigh : f64 ,
	fiftyTwoWeekChangePercent : Option<f64> ,
	earningsTimestamp : Option<u64> ,
	earningsTimestampStart : Option<u64> ,
	earningsTimestampEnd : Option<u64> ,
	trailingAnnualDividendRate : Option<f64> ,
	trailingPE : Option<f64> ,
	trailingAnnualDividendYield : Option<f64> ,
	epsTrailingTwelveMonths : Option<f64> ,
	epsForward : Option<f64> ,
	epsCurrentYear : Option<f64> ,
	priceEpsCurrentYear : Option<f64> ,
	sharesOutstanding : Option<i64> ,
	bookValue : Option<f64> ,
	fiftyDayAverage : Option<f64> ,
	fiftyDayAverageChange : Option<f64> ,
	fiftyDayAverageChangePercent : Option<f64> ,
	twoHundredDayAverage : Option<f64>,
	twoHundredDayAverageChange : Option<f64> ,
	twoHundredDayAverageChangePercent : Option<f64> ,
	marketCap : Option<f64> ,
	forwardPE : Option<f64> ,
	priceToBook : Option<f64> ,
	sourceInterval : Option<f64> ,
	exchangeDataDelayedBy : f64 ,
	averageAnalystRating : Option<String>,
	displayName : Option<String> ,
	symbol : Option<String> ,
	language : String ,
	region : String ,
	typeDisp : String ,
	triggerable : bool ,
	customPriceAlertConfidence : String ,
	messageBoardId : Option<String> ,
	isEarningsDateEstimate : Option<bool> ,
	tradeable : bool ,
	cryptoTradeable : bool,
	dividendDate : Option<i128> ,
	dividendRate : Option<i128> ,
	dividendYield : Option<i64> ,
	underlyingSymbol : Option<String> ,
	strike : Option<f64> ,
	openInterest : Option<f64> ,
	optionsType : Option<OptionsType> ,
	underlyingShortName : Option<String> ,
	expireDate : Option<i64> ,
	#[serde(deserialize_with = "deserialize_date")]
	expireIsoDate : Timestamp ,
	circulatingSupply : Option<u128> ,
	lastMarket : Option<String> ,
	volume24Hr : Option<i128> , 
	volumeAllCurrencies : Option<i128> ,
	fromCurrency : Option<String> ,
	toCurrency : Option<String>,
	coinMarketCapLink : Option<String> ,
	startDate : Option<u64> ,
	coinImageUrl : Option<String> ,
	logoUrl : Option<String> ,
}









#[derive(Debug)]
struct Timestamp {
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
enum Currency {
	USD,
	AUD,
	HKD,
	CNY,
	SGD
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
enum MarketState {
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
enum OptionsType
{
	Call,  
	Put
}



fn deserialize_date<'de, D>(deserializer: D) -> Result<Timestamp, D::Error> //turn all other 'normal timestamps into a Timestamp type'
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
	Ok(timestamp)
}

//for a newer release make this more space efficeint, i.e different quote types 
//get different structs.