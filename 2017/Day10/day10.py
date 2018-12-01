from itertools import chain
from functools import reduce
from operator import xor

num = [0,1,2,3,4]
length = "3,4,1,5"

length = "192,69,168,160,78,1,166,28,0,83,198,2,254,255,41,12"

def part1(numbers, length):

    num = list(numbers)
    skip = 0
    pos = 0

    length = list(map(int, length.split(",")))
    lst = len(num)
    
    for leng in length:

        for i in range(leng):
            num[(pos+i)%lst], num[(pos-i+leng-1)%lst] = num[(pos+leng-i-1)%lst], num[(pos+i)%lst]
        
        pos += leng + skip
        skip += 1


    return num[0]*num[1]

def dense(array):
    return reduce(xor, array)

def part2(numbers, string):
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
        result += '{:02x}'.format(i)

    return result

#print(part1(range(256), length))
#print(part2(range(256), ""))
print(part2(range(256), length)) 
