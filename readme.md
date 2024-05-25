This is a package meant to simplify the grabbing of data off a given ticker. Written using the Reqwest library. It provides a
struct object called YahooConnect which is a thread safe object which stores the cookie and "crumb" needed to access the Yahoo website and
grab the data off it. the get_ticker method with argument &str will return a string containing all information from the ticker name. If it fails
first, it will try to 