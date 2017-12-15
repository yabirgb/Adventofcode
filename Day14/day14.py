from itertools import chain
from functools import reduce
from operator import xor

def dense(array):
    return reduce(xor, array)

def KnotHash(numbers, string):
    sufix = [17,31,73,47,23]
    mess = map(ord, string)
    length = list(chain(mess, sufix))

    num = list(numbers)
    lst = len(num)
    
    skip = 0
    pos = 0

    for _ in range(64):
        for leng in length:
            reverse = []
            
            for i in range(leng):
                reverse.append(num[(i+pos)%lst])

            for i in range(leng):
                num[(i+pos)%lst] = reverse[len(reverse) - i -1]

            pos += leng + skip 
            skip += 1

    chunks = [num[i:i + 16] for i in range(0, len(num), 16)]
    conversion = map(dense, chunks)
    result = ""

    for i in conversion:
        result += '{:02b}'.format(i)

    return result

#Part1
count = 0
for i in range(0,128):
    stra = "amgozmfv-{}".format(i)
    a = KnotHash(range(256),stra)
    count += a.count("1")

print(count)
