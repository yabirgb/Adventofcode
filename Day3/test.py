from day3 import alg

def test1():
    assert alg(1) == 0
    assert alg(12) == 3
    assert alg(23) == 2
    assert alg(1024) == 31
    print("All OK")

test1()
