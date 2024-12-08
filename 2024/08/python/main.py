from collections import defaultdict
from fileinput import input
from itertools import product

antennas = defaultdict(set)
x_max, y_max = 0, 0

for idy, line in enumerate(input()):
    y_max = max(y_max, idy)
    for idx, c in enumerate(line.strip()):
        x_max = max(x_max, idx)
        if c == ".":
            continue

        antennas[c].add(complex(idx, idy))

nodes = set()
for vs in antennas.values():
    for v1, v2 in product(vs, vs):
        if v1 == v2:
            continue
        vec = v2 - v1
        nodes.add(v1 + 2 * vec)
        nodes.add(v2 - 2 * vec)

nodes = {n for n in nodes if 0 <= n.real <= x_max and 0 <= n.imag <= y_max}

gold_nodes = set()
for vs in antennas.values():
    for v1, v2 in product(vs, vs):
        if v1 == v2:
            continue
        vec = v2 - v1
        start = v1
        while 0 <= start.real <= x_max and 0 <= start.imag <= y_max:
            gold_nodes.add(start)
            start += vec

        start = v2
        while 0 <= start.real <= x_max and 0 <= start.imag <= y_max:
            gold_nodes.add(start)
            start -= vec


silver = len(nodes)
gold = len(gold_nodes)

print("silver:", silver)
print("gold:", gold)
