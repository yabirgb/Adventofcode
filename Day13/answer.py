#Part 1
security = []
with open("input.txt") as f:
    for line in f:
        line = line.replace(":", "").replace("\n","").split(" ")
        security.append((int(line[0]), int(line[1])))

def penalty():
    penality = 0

    for level, size in security:
        positions = list(range(0, size - 1)) + list(range(size-1,0, -1))
        guard_pos = positions[level%len(positions)]

        if guard_pos == 0:
            penality += level * size

    return penality

print(penalty())


#Part 2

#Cache the list so we don't generete repeated data each time
positionsD = {}
for level, size in security:
    if size not in positionsD:
        positions = list(range(0, size - 1)) + list(range(size-1,0, -1))
        positionsD[size] = positions

def penaltyDelay(delay):
    for level, size in security:
        positions = positionsD[size]
        guard_pos = positions[(delay+level)%((size-1)*2)]

        if guard_pos == 0:
            return False

    return True

ok = False
seconds = 0
while not ok:
    if penaltyDelay(seconds) == True:
        print(seconds)
        ok = True
    seconds += 1

