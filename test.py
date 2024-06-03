import json
stru = open("src/response.rs","r+")
responsefile = stru.read()
startdex = responsefile.find("pub struct Result {")
enddex = responsefile.find("//list enums below yasss queen")
responsestruct = responsefile[startdex:enddex].strip()
stru.close()

d = responsestruct

d = d.removesuffix("}")

e = ""

for r in d.strip(","):
    e += r.removeprefix("\t").removesuffix(",\n")

d = d.removeprefix("pub struct Result {\n")

while (1):
	# temp = ""
	# len = input("enter index: ")
	# if len == "":
	# 	break
	# else:
	# 	len = int(len)
	# if len == 0:
	# 	break
	# toggle = 0
	# jason = '''{"quoteResponse":{"result":[{"language":"en-US","region":"US","quoteType":"EQUITY","typeDisp":"Equity","quoteSourceName":"Nasdaq Real Time Price","triggerable":true,"customPriceAlertConfidence":"HIGH","currency":"USD","marketState":"PREPRE","regularMarketChangePercent":-0.38592046,"regularMarketPrice":178.1,"exchange":"NMS","shortName":"Tesla, Inc.","longName":"Tesla, Inc.","messageBoardId":"finmb_27444752","exchangeTimezoneName":"America/New_York","exchangeTimezoneShortName":"EDT","gmtOffSetMilliseconds":-14400000,"market":"us_market","esgPopulated":false,"hasPrePostMarketData":true,"firstTradeDateMilliseconds":1277818200000,"priceHint":2,"postMarketChangePercent":-0.314464,"postMarketTime":1717199998,"postMarketPrice":177.52,"postMarketChange":-0.559998,"regularMarketChange":-0.6899872,"regularMarketTime":1717185603,"regularMarketDayHigh":180.32,"regularMarketDayRange":"173.8201 - 180.32","regularMarketDayLow":173.8201,"regularMarketVolume":67314602,"regularMarketPreviousClose":178.79,"bid":177.95,"ask":178.44,"bidSize":3,"askSize":4,"fullExchangeName":"NasdaqGS","financialCurrency":"USD","regularMarketOpen":178.42,"averageDailyVolume3Month":94657485,"averageDailyVolume10Day":73972190,"fiftyTwoWeekLowChange":39.300003,"fiftyTwoWeekLowChangePercent":0.28314123,"fiftyTwoWeekRange":"138.8 - 299.29","fiftyTwoWeekHighChange":-121.19,"fiftyTwoWeekHighChangePercent":-0.404925,"fiftyTwoWeekLow":138.8,"fiftyTwoWeekHigh":299.29,"fiftyTwoWeekChangePercent":-18.165524,"earningsTimestamp":1713907800,"earningsTimestampStart":1721213940,"earningsTimestampEnd":1721649600,"isEarningsDateEstimate":true,"trailingAnnualDividendRate":0.0,"trailingPE":45.433674,"trailingAnnualDividendYield":0.0,"epsTrailingTwelveMonths":3.92,"epsForward":3.31,"epsCurrentYear":2.53,"priceEpsCurrentYear":70.39526,"sharesOutstanding":3189199872,"bookValue":20.188,"fiftyDayAverage":171.9572,"fiftyDayAverageChange":6.142807,"fiftyDayAverageChangePercent":0.035722885,"twoHundredDayAverage":212.0957,"twoHundredDayAverageChange":-33.995697,"twoHundredDayAverageChangePercent":-0.1602847,"marketCap":567996514304,"forwardPE":53.80665,"priceToBook":8.822073,"sourceInterval":15,"exchangeDataDelayedBy":0,"averageAnalystRating":"2.8 - Hold","tradeable":false,"cryptoTradeable":false,"displayName":"Tesla","symbol":"TSLA"}],"error":null}}'''
	# for i in range(0,len):
	# 	j = len - i
	# 	if (jason[j] == "," ):
	# 		if toggle == 1:
	# 			break
	# 		else:
	# 			toggle = 1
	# 	if toggle == 1:
	# 		temp += jason[j]
			

	# temp = reversed(temp)
	# for i in temp:
	# 	print(i,end="")

	optioner = input("name of property: ")
	if optioner == "":
		break
	strtype = ""
	finalstring = ""
	copy = ""
	for line in d.split("\n"):
		if line.find(optioner) != -1:
			copy = line
			toggle1 = 0
			toggle2 = 0
			for ch in line:
				if ch == ":":
					toggle1 = 1
					continue
				if toggle1 == 1:
					if ch == " " and toggle2 == 0:
						toggle2 = 1
						continue
					if ch == " " and toggle2 != 0:
						toggle1 = 0
						continue
					strtype += ch
				optioned = "Option<" + strtype + ">"
				finalstring = line.replace(strtype, optioned)



	d = "pub struct Result {\n" + d.replace(copy, finalstring) + "}\n"


responsefile = responsefile.replace(responsestruct, d)

stru = open("src/response.rs","w")
stru.write(responsefile)

               

# data = json.loads(s)

# # Access the 'quoteResponse' field
# quote_response = data.get("quoteResponse", {})

# # Access the 'result' list within 'quoteResponse'
# results = quote_response.get("result", [])

# # Iterate over each item in the 'result' list
# res = []
# for result in results:
#     for key, value in result.items():
#         res.append(key)
        

# stru = []
# for line in structDef:
#     field = ""
#     for i in line:
#         if i == ":":
#             break
#         field += i
#     field = field.strip()
#     if field == "":
#         continue
#     stru.append(field)

# i = 0
# extrafields = ""
# for r in res:
#     if r not in stru:
#         extrafields += f"\t{r} : c,\n"

# print(f"{i} fields are missing")

# final_struct = d.removesuffix("}")
# final_struct += extrafields
# final_struct = "struct QueryResponse {\n" + final_struct + "}"
# print(final_struct)
# pt = open("return.txt","a")
# pt.write(final_struct)