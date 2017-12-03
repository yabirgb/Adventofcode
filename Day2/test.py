from day2 import checksum, superfilter, checksumEven

matrix = [[5,1,9,5],[7,5,3],[2,4,6,8]]
matrix2 = [[5,9,2,8], [9,4,7,3], [3,8,6,5]]

def test():
    assert(checksum(matrix)) == 18
    print("Correct")

def test2():
    assert(checksumEven(matrix2)) == 9
    print("Correct")
    
test()
test2()
