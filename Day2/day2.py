from itertools import product

def checksum(matrix):
    return sum(max(row)-min(row) for row in matrix)

def read(file="input.txt"):
    matrix = []
    with open(file) as f:
        for line in f:
            #We clean the line before getting the numbers
            matrix.append(map(int, line.replace("\t", " ").replace("\n", "").split(" ")))

    return matrix

def superfilter(row):
    #I make the product and check the pairs that dont contain 1,
    #are not from the diagonal and verify that one divides the other
    a = filter(lambda pair: pair[0] != pair[1] and all(map(lambda x: x != 1, pair)) and pair[0]%pair[1] == 0, product(row, row))
    
    numbers = list(a)
    #From the conditions of the problem the result exists and is unique
    return numbers[0][0]/numbers[0][1]

def checksumEven(matrix):
    return sum(superfilter(row) for row in matrix)
