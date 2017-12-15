from itertools import count

inputA = 783
inputB = 325


def generator(n, r):
    while True:
        n = n * r % 2147483647
        yield n & 0xffff

def generator2(n, r, k):
    while True:
        n = n * r % 2147483647
        if not n % k:
            yield n & 0xffff

genA = generator(inputA, 16807)
genB = generator(inputB, 48271)

#counter = sum(next(genA) == next(genB) for _ in range(40000000))

gen2A = generator2(inputA, 16807,4)
gen2B = generator2(inputB, 48271,8)

counter = sum(next(gen2A) == next(gen2B) for _ in range(5000000))
print(counter)
