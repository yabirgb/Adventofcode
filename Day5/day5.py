def part1(lst):
   pos = 0
   step = 0
   nex = 0

   #Create a copy
   path = list(lst)

   while(pos < len(lst)):
       nex = path[pos]
       path[pos] += 1
       pos += nex
       step += 1       
       
   return step

def part2(lst):
   pos = 0
   step = 0
   nex = 0

   #Create a copy
   path = list(lst)

   while(pos < len(lst)):
       nex = path[pos]
       if nex >=  3:
           path[pos] -= 1
       else:
           path[pos] += 1
           
       pos += nex
       
       step += 1
   return step
    
       

def read():

    with open("input.txt") as f:
        lst = []
        for line in f:
            lst.append(int(line.replace("\n", "")))
        return lst

    return []


print(part1([0,3,0,1,-3]))
print(part1(read()))
print(part2([0,3,0,1,-3]))
print(part2(read()))
