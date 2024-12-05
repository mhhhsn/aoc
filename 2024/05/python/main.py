from fileinput import input
from functools import cmp_to_key

silver = 0
gold = 0

lines = [line.strip() for line in input()]
split = lines.index("")

ordering, updates = lines[:split], lines[split + 1 :]


def cmp(x, y):
    if f"{y}|{x}" in ordering:
        return 1
    return -1


for update in updates:
    raw = update.split(",")
    fixed = list(sorted(raw, key=cmp_to_key(cmp)))

    mid = int(fixed[len(fixed) // 2])
    if fixed == raw:
        silver += mid
    else:
        gold += mid

print("silver:", silver)
print("gold:", gold)
