import math
import sys
import re

RE = re.compile(r"(?P<curr>.{3}) = \((?P<left>.{3}), (?P<right>.{3})\)")
directions = []
mapping = {}
starts = []

with open(sys.argv[1]) as f:
    for i, line in enumerate(f.readlines()):
        if i == 0:
            for c in line:
                if c == "R":
                    directions.append(1)
                elif c == "L":
                    directions.append(0)
            continue
        if i == 1:
            continue

        # all other lines are a mapping
        line_match = RE.match(line)
        if line_match.group("curr").endswith("A"):
            starts.append(line_match.group("curr"))
        mapping[line_match.group("curr")] = (line_match.group("left"), line_match.group("right"))

print(directions)
print(mapping)
print(starts)

currs = starts
i = 0;
count = 0;
n_zs = 0
while not all([curr.endswith("Z") for curr in currs]):
    if (new_zs := sum([curr.endswith("Z") for curr in currs])) > n_zs:
        print(new_zs)
        n_zs = new_zs
    currs = [mapping[curr][directions[i]] for curr in currs]
    i = (i+1) % len(directions)
    count += 1
    if not count % 100000:
        print(count)
        print(currs)
print(count)

