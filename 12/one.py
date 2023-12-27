import math
import sys
import re
from functools import reduce

with open(sys.argv[1]) as f:
    for i, line in enumerate(f.readlines()):
        # split the into condition record and counts
        record, counts_str = line.split(" ")
        counts = [int(count) for count in counts_str.split(",")]


