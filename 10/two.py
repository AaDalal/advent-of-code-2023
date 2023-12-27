import math
import sys
import re
from functools import reduce
from collections import defaultdict

# basically need to the find the length of the loop
# start at the starting position and traverse up/down/left/right

char_to_dirs = { "|": ["u", "d"], "-": ["l", "r"], "F": ["d", "r"], "J": ["l", "u"], "L": ["u", "r"], "7": ["l", "d"], "S": ["u", "d", "l", "r"] }

mirror = {"u": "d", "l": "r", "r": "l", "d": "u"}

def dir_to_location(dir: str, curr_loc: (int, int)) -> (int, int):
    r, c = curr_loc
    if dir == "u":
        return r-1, c
    elif dir == "d":
        return r+1, c
    elif dir == "l":
        return r, c-1
    elif dir == "r":
        return r, c+1

result = 0
board = []
start = None
with open(sys.argv[1]) as f:
    for i, line in enumerate(f.readlines()):
        board.append(line[:-1]) # exclude \n
        for j in range(len(line)):
            if line[j] == "S":
                start = (i,j)

print(board)
print(start)

# re-create the board for debug purposes!
debug = [ ["."] * len(board[0]) ] * len(board)

loop = defaultdict(list)
stack = [(start, None, None)] # (location, direction, prev_location)
while len(stack) > 0:
    curr, prev_dir, prev_location = stack.pop()

    if prev_location != None:
        loop[prev_location[0]].append(prev_location[1])
        debug[prev_location[0]][prev_location[1]] = "O"

    
    for dir in char_to_dirs[curr_char]:
        if dir == prev_dir:
            continue
        next = dir_to_location(dir, curr)
        if next[0] < 0 or next[0] >= len(board) or next[1] < 0 or next[1] >= len(board[0]):
            continue
        next_char = board[next[0]][next[1]]
        if next_char == "S":
            break
        if mirror[dir] not in char_to_dirs[next_char]:
            continue
        
        stack.append((dir_to_location(dir, curr), mirror[dir], curr))
    else:
        pass
    break

        




min_row = min(loop.keys())
max_row = max(loop.keys())
result = 0
for r in range(min_row, max_row + 1):
    loop[r].sort()
    for i in range(len(loop[r]) - 1):
        result += (loop[r][i+1] - loop[r][i] - 1)

print(result)
for row in debug:
    print("".join(row))
