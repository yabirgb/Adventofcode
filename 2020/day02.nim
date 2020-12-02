
import strutils, strformat

proc valid_password(min, max:int, ch :char,  pass: string): bool =
    #[
    - min appearences
    - max appearences
    - ch character to consider
    - password
    ]#

    let counted = count(pass, ch)
    return not (counted < min or counted > max)


proc part_one(path: string): int =
    var line: string

    var f: File
    discard open(f, path)

    var valids = 0

    while readLine(f, line):
        var tmp = line.splitWhitespace()
        let numbers = split(tmp[0], '-')
        let min = parseInt(numbers[0])
        let max = parseInt(numbers[1])
        let ch = tmp[1][0]
        let pass = tmp[2]

        if valid_password(min, max, ch, pass):
            valids += 1

    return valids

proc part_two(path: string): int =
    var line: string

    var f: File
    discard open(f, path)

    var valids = 0

    while readLine(f, line):
        var tmp = line.splitWhitespace()
        let numbers = split(tmp[0], '-')
        # pos are 1 based
        let p1 = parseInt(numbers[0]) - 1
        let p2 = parseInt(numbers[1]) - 1
        let ch = tmp[1][0]
        let pass = tmp[2]

        if pass[p1] == ch xor pass[p2] == ch:
            valids += 1

    return valids

var result_p1 = part_one("inputs/day2.txt")
var result_p2 = part_two("inputs/day2.txt")
echo(&"part 1 result: {result_p1}")
echo(&"part 2 result: {result_p2}")
#echo(&"part 2 result: {result_p2}")