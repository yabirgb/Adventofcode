from itertools import count

inputA = 783
inputB = 325


def generator(n, r):
    while True:
        n = n * r % 2147483647
        if n % r == 0:
            yield n & 0xffff

genA = generator(inputA, 16807)
genB = generator(inputB, 48271)

counter = sum(next(genA) == next(genB) for _ in range(40000000))

print(counter)
