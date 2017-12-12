class Equivalence:

    def __init__(self, integer):
        self.elements = set()
        self.elements.add(integer)

    def __iadd__(self, other):
        self.elements.update(other.objs())

    def add(self, integer):
        self.elements.add(integer)

    def represents(self, integer):
        return integer in self.elements

    def objs(self):
        return self.elements

    def __contains__(self, key):
        return self.represents(key)
    
    def __str__(self):
        return repr(self.elements)

    def __repr__(self):
        return __str__()

    def __len__(self):
        return len(self.elements)

    def __eq__(self, other):
        return self.represents(key)

eq = []

with open("input.txt") as f:


    for line in f:
        line = line.replace("\n","").replace(",", "").split(" ")
        key = int(line[0])
        related = list(map(int, line[2:]))
        target = None
        
        if key not in eq:
            eq.append(Equivalence(key))
            target = eq.index(key)
        else:
            target = eq.index(key)

        for rel in related:            
            if rel in eq:
                eq[target].add(rel)
            else:
                pos = eq.find(rel, target)
                eq[target] += eq[pos]
                del eq[pos]

for elem in eq:
    print(elem)

#zero = eq.index(0)
#print(len(eq[zero]))
