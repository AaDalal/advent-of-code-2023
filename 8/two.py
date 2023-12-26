from functools import reduce
import math
import sys
import re
from collections import defaultdict


RE = re.compile(r"(?P<curr>.{3}) = \((?P<left>.{3}), (?P<right>.{3})\)")
directions = []
mapping = {}
currs = []

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
            currs.append(line_match.group("curr"))
        mapping[line_match.group("curr")] = (line_match.group("left"), line_match.group("right"))

print(mapping)
print(directions)

i = 0

counts = []
for curr in currs:
    count = 0
    while not curr.endswith("Z"):
        curr = mapping[curr][directions[i]]
        i = (i+1) % len(directions)
        count += 1
    counts.append(count)
print(counts)

# get the LCM of these cycles
# prime factorize -> get a multiset of numbers
def prime_factorize(num):
    factors = defaultdict(int)
    for i in range(2, num+1):
        while num % i == 0:
            factors[i] += 1
            num = num // i
    return factors

# union multiset: modifies ms1 in place
def union_multiset(this: defaultdict, other: defaultdict):
    for key in other.keys():
        this[key] = max(this[key], other[key])
    return this


factors = [prime_factorize(count) for count in counts]
print(factors)
lcm_factors = reduce(union_multiset, factors)
print(lcm_factors)

result = 1
for key, value in lcm_factors.items():
    for i in range(value):
        result *= key
print(result)

# alt approach
def euclid_gcd(a: int, b: int) -> int:
    if a < b:
        return euclid_gcd(a, b)
    while b != 0:
        a, b = b, a % b
    return a # last non-zero remainder, or original b if b divides a

def lcm(a: int, b: int) -> int:
    return a / euclid_gcd(a,b) * b

print(reduce(lcm, counts))
