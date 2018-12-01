memory = [0,2,7,0]

ind = "4	1	15	12	0	9	9	5	5	8	7	3	14	5	12	3"



def part1(string):
    memory = list(map(int, string.replace("\t", " ").split(" ")))
    states = set()

    elements = len(memory)
    steps = 0

    while tuple(memory) not in states:
        states.add(tuple(memory)) #save state
        amount = max(memory)
        redistribute = memory.index(amount)
        memory[redistribute] = 0
        
        for i in range(1,amount+1):
            memory[(redistribute + i)%elements] += 1
            
        steps += 1
        
    return steps

def part2(string):
    memory = list(map(int, string.replace("\t", " ").split(" ")))
    states = {} #Now the best structure is a dictionary

    elements = len(memory)
    steps = 0

    while tuple(memory) not in states:
        states[tuple(memory)] = steps #save state
        amount = max(memory)
        redistribute = memory.index(amount)
        memory[redistribute] = 0
        
        for i in range(1,amount+1):
            memory[(redistribute + i)%elements] += 1
            
        steps += 1
        
    return steps - states[tuple(memory)]
    

print(part1(ind))
print(part2(ind))
