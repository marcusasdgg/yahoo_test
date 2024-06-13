This is a package meant to simplify the grabbing of data off a given ticker. Written using the Reqwest library. It provides a
struct object called YahooConnect which is a thread safe object which stores the cookie and "crumb" needed to access the Yahoo website and
grab the data off it. the get_ticker method with argument &str will return a struct containing all fields of the JSON that yahoo provides.
the implementation of the struct is an array of JSONS which each refer to a single stock request, allowing you to combine multiple stock requests
into a single YAHOOCONNECT request by separating with a Comma, i.e AAPL,TSLA.