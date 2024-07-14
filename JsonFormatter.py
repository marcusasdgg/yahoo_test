import os
import json

file = open('json.json','r').read()
filew = open('properties.txt', 'w')

ret = ""

data = json.loads(file)
for obj in data["quoteResponse"]["result"]:
    for (value) in obj:
        ret += f"{value}: \n"

filew.write(ret)
