import os
import json
str1 = open('json.json','r').read()
str2 = open('json1.json','r').read()

data = json.loads(str1)["quoteResponse"]["result"][0]
data2 = json.loads(str2)["quoteResponse"]["result"][0]

print("for exclusive in data")
for property in data:
    if property not in data2:
        print(property)

print("for exclusive in data2")
for property in data2:
    if property not in data:
        print(property)
