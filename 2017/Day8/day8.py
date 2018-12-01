registers = {}

with open("input.txt") as f:
    max_val = 0
    for line in f:
        line = line.split(" ")
        reg = line[0]
        reg2 = line[4]
        
        if reg not in registers.keys():
            registers[reg] = 0

        if reg2 not in registers.keys():
            registers[reg2] = 0

        line[4] = "registers['{}']".format(reg2)
        condition = eval(" ".join(line[4:]))

        if condition:

            if line[1] == "inc":
                registers[reg] += int(line[2])
            else:
                registers[reg] -= int(line[2])

        if max(registers.values()) > max_val:
            max_val = max(registers.values())

    print("Max value at the end of execution: {}".format(max(registers.values())))
    print("Max value reached: {}".format(max_val))
