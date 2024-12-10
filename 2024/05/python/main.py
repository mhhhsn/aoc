from fileinput import input
from functools import cmp_to_key
from itertools import takewhile

inp = map(str.strip, input())
ordering = set(takewhile(bool, inp))
us = list(inp)


def cmp(x, y):
    return (f"{y}|{x}" in ordering) * 2 - 1  # hehe


silver = 0
gold = 0

for u in us:
    pre = u.split(",")
    post = list(sorted(pre, key=cmp_to_key(cmp)))

    mid = int(post[len(post) // 2])
    if pre == post:
        silver += mid
    else:
        gold += mid

print("silver:", silver)
print("gold:", gold)
