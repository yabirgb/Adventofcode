pairs = {}

with open("input2.txt") as f:
    for line in f:
        line = line.replace("\n","").replace(",", "").split(" ")
        pairs[int(line[0])] = set(map(int, line[2:]))

def search(num):
    equiv = set([num])
    start = set([num])

    while start:

        upgrade = set()
        for elem in start:
            upgrade.update(pairs[elem])

        start = upgrade - equiv
        equiv.update(upgrade)

    return equiv
            
def search_all():
    allkeys = set(pairs)
    amount = 0
    while allkeys:
        amount += 1
        group_last = search(allkeys.pop())
        allkeys -=group_last

    return amount
print(len(search(0)))
print(search_all())
