import math
import sys
import re
from functools import reduce

def compute_next(sequence: list[int]) -> int:
    if all([elem == 0 for elem in sequence]):
        return 0 # base case

    # compute difference between elements
    differences = [sequence[i+1] - sequence[i] for i in range(len(sequence)-1)]
    return sequence[-1] + compute_next(differences)

result = 0
with open(sys.argv[1]) as f:
    for i, line in enumerate(f.readlines()):
        sequence = [int(piece) for piece in line.split(" ")]
        next_elem = compute_next(sequence[::-1])
        print(next_elem)
        result += next_elem

print(result)
