#Part1

#First attempt
def csum2(sequence):
    suma = 0
    l = len(sequence)
    for pos, key in enumerate(sequence):
        if key == sequence[(pos+1)%l]:
            suma += int(key)

    return suma

#Second attempt
def csum(sequence):
    suma = 0
    for currentValue, nextValue in zip(sequence, sequence[1:]):
        if currentValue == nextValue:
            suma += int(currentValue)

    if sequence[0] == sequence[-1]:
        suma += int(sequence[0])

    return suma

#Part2
def csumHalfway(sequence):
    suma = 0
    half = len(sequence)//2
    for currentValue, nextValue in zip(sequence*2, sequence[half:]):
        if currentValue == nextValue:
            suma += 2*int(currentValue)

    return suma
