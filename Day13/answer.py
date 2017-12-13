#Part 1
security = []
with open("input.txt") as f:
    for line in f:
        line = line.replace(":", "").replace("\n","").split(" ")
        security.append((int(line[0]), int(line[1])))

def penalty(delay):
    penality = 0

    for level, size in security:
        positions = list(range(0, size - 1)) + list(range(size-1,0, -1))
        guard_pos = positions[(level)%len(positions)]

        if guard_pos == 0:
            penality += level * size

    return penality

print(penalty(0))
#Part 2



