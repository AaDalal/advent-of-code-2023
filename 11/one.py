import math
import sys
import re
from functools import reduce

# 1. iterate through the board
# 2. find the rows and columns with no galaxies + get the positions of all the galaxies
rows = [] # whether rows are empty
galaxies = [] # (row, column)
with open(sys.argv[1]) as f:
    for i, line in enumerate(f.readlines()):
        if i == 0:
            columns = [True] * (len(line) - 1) # whether columns are empty
        row_isempty = True
        for j,c in enumerate(line[:-1]): # exclude \n
            if c == "#":
                row_isempty = False
                galaxies.append((i, j))
                columns[j] = False
        rows.append(row_isempty)
MULTIPLE = int(sys.argv[2]) - 1

print(rows)
print(columns)
print(galaxies)

# 3. for each row/column in the initial board, calculate the number of empty rows/columns before that row/column (cum sum!)
def cum_sum(l: list) -> list:
    sum = 0
    out = []
    for item in l:
        sum += item
        out.append(sum)
    return out

cum_rows = cum_sum([MULTIPLE * isempty for isempty in rows])
cum_cols = cum_sum([MULTIPLE * isempty for isempty in columns])

# 4. compute the real positions of all the galaxies
real_galaxies = [(r + cum_rows[r], c + cum_cols[c]) for (r, c) in galaxies]

# 5. for each pair of galaxies, compute the distance (can just use manhattan distance)
# 6. return the total result
def manhattan(coord: (int, int), other_coord: (int, int)) -> int:
    return abs(coord[0] - other_coord[0]) + abs(coord[1] - other_coord[1])

result = 0
for i, gal in enumerate(real_galaxies):
    for other_gal in real_galaxies[i+1:]:
        result += manhattan(gal, other_gal)
print(result)
