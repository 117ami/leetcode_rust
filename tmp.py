

from bisect import bisect_left, bisect_right 

a = [1,2,3,3,4,5,8]
print(bisect_left(a, 3))
print(bisect_right(a, 3))