from fileinput import input
from functools import cache
from itertools import takewhile

inp = map(str.strip, input())
towels = [t.strip() for t in next(takewhile(bool, inp)).split(",")]
designs = [d for d in inp if d]

# Construct trie
trie = {}
for towel in towels:
    prev = None
    curr = trie
    for t in towel:
        term, succ = curr.setdefault(t, (False, {}))
        prev = curr
        curr = succ
    prev[towel[-1]] = (True, curr)


@cache
def steps(design, n=0):
    if n == len(design):
        return 1
    ways = 0
    curr = trie
    for d, s in enumerate(design[n:], start=1):
        if s not in curr:
            return ways
        term, curr = curr[s]
        if term:
            ways += steps(design, n + d)
    return ways


silver = sum(map(bool, map(steps, designs)))
gold = sum(map(steps, designs))

print("silver:", silver)
print("gold:", gold)
