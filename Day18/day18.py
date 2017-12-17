step = 337

def spin(step, ran):
    pos = 1
    state = [0,1]
    for i in range(2,ran):
        lon = len(state)
        pos = (pos + step + 1)%lon
        state.insert(pos, i)
    return state[pos+1]

print("Solution to part 1: ", spin(step, 2018))

# Part 2

def spinlong(step, ran):
    pos = 0
    val = 0
    for i in range(1,ran):
        #If the postion when you insert is the first one
        #The next element of 0 changes. Saw it using the
        #previous part
        pos = (pos + step+1)%i

        if pos == 0:
            val = i
            
    return val

print("Solution to part 2: ", spinlong(step, 50000001))
