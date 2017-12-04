from day4 import valid, validP2

def test1():
    assert valid("aa bb cc dd ee") == True
    assert valid("aa bb cc dd aa") == False
    assert valid("aa bb cc dd aaa") == True

    print("All ok")

def test2():
    assert validP2("abcde fghij") == True
    assert validP2("abcde xyz ecdab") == False
    assert validP2("a ab abc abd abf abj") == True
    assert validP2("iiii oiii ooii oooi oooo") == True
    assert validP2("oiii ioii iioi iiio") == False
    print("All OK")
    
test1()
test2()
