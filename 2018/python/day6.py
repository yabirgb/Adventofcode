from collections import defaultdict

data = ""

with open("inputs/day6.txt", "r") as f:
    data = f.read()

    
points = set()
max_x, max_y = 0,0
for x in data.split("\n"):
    if x:
        a = map(int, x.replace(" ","").split(","))
        a = tuple(a)
        max_x = max(max_x, a[0])
        max_y = max(max_y, a[1])
        points.add(a)

def part1():
    # Assign an id to points
    ids = {point: i for point, i in enumerate(points)}
    regions = defaultdict(int)
    inf = set()
    for i in range(max_x+1):
        for j in range(max_y+1):

            dists = min([(abs(x-i) + abs(y-j), pk) for pk, (x,y) in ids.items()], key= lambda t: t[0])
            #print(dists)
            regions[dists[1]] += 1

            if i == 0 or j == 0 or i == max_x or j == max_y:
                inf.add(dists[1])

    #print(regions)
    return max(size for ids, size in regions.items() if ids not in inf)

#print(f"Solution to part 1: {part1()}")

def part2():

    limit = 10000
    region = 0
    for i in range(max_x+1):
        for j in range(max_y+1):
            region += int(sum(abs(x - i) + abs(y - j) for x, y in points) < limit)

    return region

print(f"Solution to part 2: {part2()}")
