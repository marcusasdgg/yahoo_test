use serde::{Serialize, Deserialize};
type c = i32;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case,dead_code)]
struct QueryResponse {
	quoteType : QuoteType, 
	quoteSourceName : String ,
	currency : Currency, 
	marketState : MarketState ,
	regularMarketChangePercent : u64 , 
	regularMarketPrice : u64 ,
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
	postMarketChangePercent : f64 ,
	postMarketTime : u64 ,
	postMarketPrice : f64 ,
	postMarketChange : f64 ,
	regularMarketChange : f64 ,
	regularMarketTime : u64 ,
	regularMarketDayHigh : f64 ,
	regularMarketDayRange : String ,
	regularMarketDayLow : f64 ,
	regularMarketVolume : f64 ,
	regularMarketPreviousClose : f64 ,
	bid : f64 ,
	ask : f64 ,
	bidSize : f64 ,
	askSize : f64 ,
	fullExchangeName : String ,
	financialCurrency : Currency, 
	regularMarketOpen : u64, 
	averageDailyVolume3Month : u64 ,
	averageDailyVolume10Day : u64 ,
	fiftyTwoWeekLowChange : f64 ,
	fiftyTwoWeekLowChangePercent : f64 ,
	fiftyTwoWeekRange : String ,
	fiftyTwoWeekHighChange : f64 ,
	fiftyTwoWeekHighChangePercent : f64,
	fiftyTwoWeekLow : f64 ,
	fiftyTwoWeekHigh : f64 ,
	fiftyTwoWeekChangePercent : f64 ,
	earningsTimestamp : u64 ,
	earningsTimestampStart : u64 ,
	earningsTimestampEnd : u64 ,
	trailingAnnualDividendRate : f64 ,
	trailingPE : f64 ,
	trailingAnnualDividendYield : f64 ,
	epsTrailingTwelveMonths : f64 ,
	epsForward : f64 ,
	epsCurrentYear : f64 ,
	priceEpsCurrentYear : f64 ,
	sharesOutstanding : i64 ,
	bookValue : f64 ,
	fiftyDayAverage : f64 ,
	fiftyDayAverageChange : f64 ,
	fiftyDayAverageChangePercent : f64 ,
	twoHundredDayAverage : f64,
	twoHundredDayAverageChange : f64 ,
	twoHundredDayAverageChangePercent : f64 ,
	marketCap : f64 ,
	forwardPE : f64 ,
	priceToBook : f64 ,
	sourceInterval : f64 ,
	exchangeDataDelayedBy : f64 ,
	averageAnalystRating : String,
	displayName : String ,
	symbol : String ,
    json : String, 
	language : String,
	region : String,
	typeDisp : QuoteType,
	triggerable : bool,
	customPriceAlertConfidence : i32,
	messageBoardId : i32,
	isEarningsDateEstimate: i64,
	tradeable : bool,
	cryptoTradeable : bool,
	dividendDate : i128,
	dividendRate : i128,
	dividendYield : i64,
	underlyingSymbol : String,
	strike : f64,
	openInterest : f64,
	optionsType : OptionsType,
	underlyingShortName : String,
	expireDate : u64,
	expireIsoDate : u64,
	circulatingSupply : u128,
	lastMarket : String,
	volume24Hr : i128,
	volumeAllCurrencies : i128,
	fromCurrency : String,
	toCurrency : String,
	coinMarketCapLink : String,
	startDate : u64,
	coinImageUrl : String,
	logoUrl : String,
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
	CLOSED,
	REGULAR
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
enum QuoteType {
	EQUITY,
	OPTION,
	BOND,
	FUTURE
}

#[derive(Debug,Serialize, Deserialize)]
enum OptionsType
{
	CALL,
	PUT
}