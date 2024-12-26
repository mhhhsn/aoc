from fileinput import input

lines = [int(line.strip()) for line in input()]


def evolve(x, n):
    for _ in range(n):
        x ^= x << 6
        x &= 0xFFFFFF
        x ^= x >> 5
        x &= 0xFFFFFF
        x ^= x << 11
        x &= 0xFFFFFF
    return x


silver = sum(evolve(x, 2000) for x in lines)
gold = 0

print("silver:", silver)
print("gold:", gold)
