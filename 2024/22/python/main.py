from fileinput import input
from itertools import islice

lines = [int(line.strip()) for line in input()]


def prices(secret):
    x = secret
    while True:
        yield x
        x ^= x << 6
        x &= 0xFFFFFF
        x ^= x >> 5
        x &= 0xFFFFFF
        x ^= x << 11
        x &= 0xFFFFFF


silver = sum(next(islice(prices(x), 2000, None)) for x in lines)
gold = 0

print("silver:", silver)
print("gold:", gold)
