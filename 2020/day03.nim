import strutils, sequtils, strformat

proc countTrees(map: seq[string], down, right :int):int = 
    var trees = 0
    var row  = 0
    var col = 0

    while row < len(map):
        if map[row][col] == '#':
            trees += 1
        
        row += down
        col = (col + right) mod len(map[0])

    return trees

proc partTwo(map: seq[string], paths: seq[(int, int)]): int =
    result = 1
    for s in paths:
        result *= countTrees(map, s[1], s[0]) 

let map = toSeq("inputs/day3.txt".lines)

var result_p1 = countTrees(map, 1, 3)
var result_p2 = partTwo(map, @[(1,1), (3,1), (5,1), (7,1), (1,2)])
echo(&"part 1 result: {result_p1}")
echo(&"part 2 result: {result_p2}")