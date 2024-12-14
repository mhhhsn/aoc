import re
from fileinput import input
from itertools import groupby
from math import prod

XMAX, YMAX = 11, 7  # for test
XMAX, YMAX = 101, 103  # for aoc

robots = [
    re.fullmatch(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)", line.strip()).groups()
    for line in input()
]
robots = [
    (complex(int(px), int(py)), complex(int(vx), int(vy))) for px, py, vx, vy in robots
]


def move(p, v, s):
    e = p + v * s
    return complex(e.real % XMAX, e.imag % YMAX)


def quad(x):
    return (x.real < XMAX // 2, x.imag < YMAX // 2)


def is_quad(x):
    return x.real != XMAX // 2 and x.imag != YMAX // 2


def draw(s):
    print(f"Seconds passed: {s}")
    moved = {move(p, v, s) for p, v in robots}
    for x in range(XMAX):
        for y in range(YMAX):
            print(
                "#" if complex(x, y) in moved else " ",
                end="",
            )
        print()
    print("-" * XMAX)


moved = groupby(
    sorted(
        filter(is_quad, (move(p, v, 100) for p, v in robots)),
        key=quad,
    ),
    quad,
)


silver = prod(sum(1 for _ in k) for _, k in moved)
gold = 0

print("silver:", silver)
print("gold:", gold)

s = 0
while s < 10_000:
    draw(s)
    s += 1
