import math
import sys
import re
from functools import reduce

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

def loop_length(start: (int, int), board: list[list[str]]) -> int:
    stack = [(start, None, 0)]
    while len(stack) > 0:
        curr, prev_dir, length = stack.pop()
        if curr[0] < 0 or curr[0] >= len(board) or curr[1] < 0 or curr[1] >= len(board[0]):
            continue
        curr_char = board[curr[0]][curr[1]]
        if prev_dir is not None and prev_dir not in char_to_dirs[curr_char]:
            continue
        
        print(curr_char)

        if curr_char == "S" and prev_dir is not None:
            return length # base case

        
        for dir in char_to_dirs[curr_char]:
            if dir == prev_dir:
                continue
            stack.append((dir_to_location(dir, curr), mirror[dir], 1 + length))
    return None

result = 0
board = []
start = None
with open(sys.argv[1]) as f:
    for i, line in enumerate(f.readlines()):
        board.append([c for c in line[:-1]]) # exclude \n
        for j in range(len(line)):
            if line[j] == "S":
                start = (i,j)

print(board)
print(start)

length = loop_length(start, board)
print(length)
print((length + 1) // 2)
