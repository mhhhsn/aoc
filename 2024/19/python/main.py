from fileinput import input
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


def drops(design, start=0):
    curr = trie
    for n, s in enumerate(design[start:], start=start + 1):
        if s not in curr:
            return
        term, curr = curr[s]
        if term:
            yield n


def match(design):
    q = list(drops(design))
    seen = set()
    while q:
        n = q.pop()
        if n == len(design):
            return True
        if n in seen:
            continue
        seen.add(n)
        for d in drops(design, n):
            q.append(d)
    return False


silver = sum(map(match, designs))
gold = 0

print("silver:", silver)
print("gold:", gold)
