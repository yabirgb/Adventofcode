from day1 import csum, csumHalfway

def part1():
    assert csum("1122") == 3
    assert csum("1111") == 4
    assert csum("1234") == 0
    assert csum("91212129") == 9

    print("All correct")

def part2():
    assert csumHalfway("1212") == 6
    assert csumHalfway("1221") == 0
    assert csumHalfway("123425") == 4
    assert csumHalfway("123123") == 12
    assert csumHalfway("12131415") == 4
    

    print("All correct")

part1()
part2()
