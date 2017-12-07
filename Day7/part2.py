tree = {}
weights = {}

with open("input.txt") as f:
    for line in f:
        line = line.replace("(", "").replace(")", "").replace(",", "").replace("\n","").split(" ")
        parent = line[0]
        weight = int(line[1])

        if "->" in line:
            siblings = line[3:]
            tree[parent] = siblings

        weights[parent] = weight
            


def balance(node):
    children = tree[node]
    branches = []
    for child in children:
        if child in tree:
            computedW = balance(child)
            cW = sum(computedW) + weights[child]
        else:
            cW = weights[child]
        branches.append(cW)   
    if len(set(branches)) != 1:
        print ('Wrong weight {} weighting {}.'.format(tree[node],                                                                  branches))
    return branches 

balance("hlhomy")
#balance("tknk")
