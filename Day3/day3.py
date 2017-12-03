from math import sqrt, ceil

#I noticed a pattern with the spiral where in the botton right
#we find the square of the odd numbers.
#Using this as reference we can compute the steps needed.
#Even we can know if they are up/left/right/down

def alg(number):
    root = int(ceil(sqrt(number)))

    if number == 1:
        return 0

    if root % 2 == 0:
        root += 1

    evenPos = (root-1)//2

    d = number - (root -2)**2
    side = d % (root -1)

    return evenPos + abs(side - evenPos)
