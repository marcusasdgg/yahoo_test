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

# temp = ""
# lens = input("enter index: ")
# if lens == "":
# 	exit()
# else:
# 	lens = int(lens)
# if len == 0:
# 	exit()
# toggle = 0
# jason = '''{"quoteResponse":{"result":[{"language":"en-US","region":"US","quoteType":"OPTION","typeDisp":"Option","quoteSourceName":"Delayed Quote","triggerable":false,"customPriceAlertConfidence":"NONE","currency":"USD","marketState":"REGULAR","regularMarketChangePercent":5.7505054,"regularMarketPrice":104.27,"underlyingSymbol":"TSLA","exchange":"OPR","shortName":"TSLA Jun 2024 75.000 call","longName":"TSLA Jun 2024 75.000 call","exchangeTimezoneName":"America/New_York","exchangeTimezoneShortName":"EDT","gmtOffSetMilliseconds":-14400000,"market":"us24_market","esgPopulated":false,"hasPrePostMarketData":false,"firstTradeDateMilliseconds":1716350400000,"priceHint":2,"regularMarketChange":5.669998,"regularMarketTime":1717685760,"regularMarketDayHigh":104.27,"regularMarketDayRange":"104.27 - 104.27","regularMarketDayLow":104.27,"regularMarketVolume":45,"regularMarketPreviousClose":98.6,"bid":101.85,"ask":103.4,"fullExchangeName":"OPR","regularMarketOpen":104.27,"fiftyTwoWeekLowChange":0.0,"fiftyTwoWeekLowChangePercent":0.0,"fiftyTwoWeekRange":"104.27 - 104.27","fiftyTwoWeekHighChange":0.0,"fiftyTwoWeekHighChangePercent":0.0,"fiftyTwoWeekLow":104.27,"fiftyTwoWeekHigh":104.27,"strike":75.0,"openInterest":64,"optionsType":"Call","underlyingShortName":"Tesla, Inc.","expireDate":1717718400,"expireIsoDate":"2024-06-07T00:00:00Z","sourceInterval":15,"exchangeDataDelayedBy":20,"tradeable":false,"cryptoTradeable":false,"symbol":"TSLA240607C00075000"}],"error":null}}'''
# print(len(jason))
# for i in range(0,lens):
# 	j = lens - i
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
strtype = ""
finalstring = ""
copy = ""
for line in d.split("\n"):
	if line.find(optioner + " ") != -1:
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


d = d.removeprefix("pub struct Result {\n").removesuffix("}")
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

							
							
