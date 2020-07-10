#!/usr/bin/env python

file1 = open('output.txt', 'r') 
#lines = file1.readlines() 
  

tags = ['compressor']
compressors = set()

c_scores = {}

count = 0
tag = ""
# Strips the newline character 
for line in file1.readlines(): 
    line = line .strip()
    end_len = len("compressions by name")
    if line.endswith("compressions by name"):
        tag = line[2:-end_len].strip()
        if not tag in tags:
            tags.append(tag)
        #print("Line{}: {}".format(count, line.strip()))
    if ":" in line:
        #print("{}:{}".format(tag, line))
        compressor = line.split(":")[0].strip()        
        compressors.add(compressor)
        if not compressor in c_scores.keys():
            c_scores[compressor] = []
        c_scores[compressor].append(line.split(":")[2].strip())


print(",".join(tags))
#print(compressors)
for k in sorted(c_scores.keys()):
    print("{},{}".format(k, ",".join(c_scores[k])))

file1.close()

print("\n\n\n")

file1 = open('output.txt', 'r') 
#lines = file1.readlines() 
  

tags = ['compressor']
compressors = set()

c_scores = {}

count = 0
tag = ""
# Strips the newline character 
for line in file1.readlines(): 
    line = line .strip()
    end_len = len("compressions by name")
    if line.endswith("compressions by name"):
        tag = line[2:-end_len].strip()
        if not tag in tags:
            tags.append(tag)
        #print("Line{}: {}".format(count, line.strip()))
    if ":" in line:
        #print("{}:{}".format(tag, line))
        compressor = line.split(":")[0].strip()        
        compressors.add(compressor)
        if not compressor in c_scores.keys():
            c_scores[compressor] = []
        c_scores[compressor].append(line.split(":")[2].strip())


print(",".join(tags))
print(compressors)
for k in sorted(c_scores.keys()):
    print("{},{}".format(k, ",".join(c_scores[k])))

file1.close()

