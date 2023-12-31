import math
import sys
import re
from functools import reduce

with open(sys.argv[1]) as f:
    items = f.readline().split(",")
    boxes = [dict() for _ in range(256)]

    for item in items:
        total = 0
        for c in item:
            if c == "\n":
                print("NEWLINE")
                continue
            total += ord(c)
            total *= 17
            total %= 256

        if "=" in item:
            key, focal_length = item.split("=")
            boxes[total][key] = int(focal_length)
        else:
            boxes[total].pop(item[:-1], None)

    print(boxes)

    result = 0
    for i, box in enumerate(boxes):
        for j,focal_length in enumerate(box.values()):
            result += (j + 1) * (i + 1) * focal_length

print(result)

