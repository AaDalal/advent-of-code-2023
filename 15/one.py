import math
import sys
import re
from functools import reduce

with open(sys.argv[1]) as f:
    items = f.readline().split(",")
    result = 0
    for item in items:
        total = 0
        for c in item:
            if c == "\n":
                print("NEWLINE")
                continue
            total += ord(c)
            total *= 17
            total %= 256
        result += total

print(result)

