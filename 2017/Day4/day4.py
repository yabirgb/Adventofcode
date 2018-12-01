from itertools import groupby

def valid(line): 
    l = line.split(" ")
    d = groupby(sorted(l))

    if len (list(d)) == len(l):
        return True
    
    return False

def validP2(line):
    l = list(map(sorted, line.split(" ")))
    d = groupby(sorted(l))

    if len (list(d)) == len(l):
        return True
    
    return False

def read(tfile="input.txt", function=valid):

    number = 0
    with open(tfile, "r") as f:
        for line in f:
            line = line.replace("\n", "")
            if line != "" and function(line):
                number += 1

    return number



