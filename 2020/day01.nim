import tables
import macros
import algorithm
import system
import strformat, strutils, sequtils

var numbers = @[1721,979,366,299,675,1456]

proc twosumprod(numbers: seq[int]): int =
    var diffs = initTable[int, int]()
    for num in numbers:
        if diffs.hasKey(num):
            return num*diffs[num]
        diffs[2020-num]=num

proc threesumprod(numbers: seq[int], target:int): int =
    var nums = sorted(numbers, system.cmp[int])    
    for i, n in nums:
        var l = i+1
        var r = len(nums)-1
        while l<r:
            var sum:int = n + nums[l] + nums[r]
            if sum == target:
                return n * nums[l] * nums[r]
            elif sum < target:
                l += 1
            else:
                r -= 1
    

assert(twosumprod(numbers) == 514579)
assert(threesumprod(numbers, 2020) == 241861950)

var result_p1 = twosumprod(toSeq("inputs/day1.txt".lines).map(parseInt))
var result_p2 = threesumprod(toSeq("inputs/day1.txt".lines).map(parseInt), 2020)
echo(&"part 1 result: {result_p1}")
echo(&"part 2 result: {result_p2}")