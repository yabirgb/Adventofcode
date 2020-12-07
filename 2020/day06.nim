import strutils, sequtils, strformat, sets


proc parse_data(filepath: string):(int,int) =
    #var data: seq[HashSet] = @[]
    var total:int = 0
    var total_group: int = 0
    for group in (readFile filepath).strip.split("\n\n"):
        #echo(&"{group}\n")
        var chars: set[char]
        var all: set[char] = {'a'..'z'}
        for person in group.splitLines:
            var person_set:set[char]
            for c in person:
                chars.incl c
                person_set.incl c
            all = all*person_set
        total = total + chars.card
        total_group = total_group + all.card
    return (total, total_group)
        #set(group)

echo(&"""{parse_data("inputs/day6.txt")}""")
#var result_p1 = countTrees(map, 1, 3)
#var result_p2 = partTwo(map, @[(1,1), (3,1), (5,1), (7,1), (1,2)])
#echo(&"part 1 result: {result_p1}")
#echo(&"part 2 result: {result_p2}")