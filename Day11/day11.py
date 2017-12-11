x = 0
y = 0
maximum = 0

with open("input.txt") as f:

    l = f.read().split(",")

    for door in l:

        if door == "n":
            x += 1
        elif door == "ne":
            x += 0.5
            y += 0.5
        elif door == "se":
            x -= 0.5
            y += 0.5
        elif door == "s":
            y -= 1
        elif door == "sw":
            x -= 0.5
            y -= 0.5
        elif door == "nw":
            x += 0.5
            y -= 0.5

        if maximum < abs(abs(x) - abs(y)):
            maximum = abs(abs(x) - abs(y))

print(abs(abs(x) - abs(y)))
print(maximum)
