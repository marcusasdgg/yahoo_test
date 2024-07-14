import os

filer = open('properties.txt', 'r')
ret = ""
for line in filer:
    ret += line.rstrip() + " String,\n"

filew = open('propertiesString.txt','w').write(ret)
