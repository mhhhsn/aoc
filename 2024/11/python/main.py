from fileinput import input
from functools import cache
from math import floor, log

stones = [int(x) for x in input().readline().split()]


@cache
def blink(s, n):
    if n == 0:
        return 1

    if s == 0:
        return blink(1, n - 1)

    digits = floor(log(s, 10) + 1e-6) + 1
    if digits % 2 == 0:
        left, right = divmod(s, 10 ** (digits // 2))
        return blink(left, n - 1) + blink(right, n - 1)

    return blink(s * 2024, n - 1)


silver = sum(blink(s, 25) for s in stones)
gold = sum(blink(s, 75) for s in stones)

print("silver:", silver)
print("gold:", gold)
