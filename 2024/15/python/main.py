from fileinput import input
from itertools import takewhile

inp = map(str.strip, input())

grid = {
    complex(x, y): c
    for y, line in enumerate(takewhile(bool, inp))
    for x, c in enumerate(line)
}
moves = "".join(inp)

bot = next(p for p, c in grid.items() if c == "@")


def push(x, v):
    match grid[x]:
        case ".":
            return True
        case "#":
            return False
        case _ if push(x + v, v):
            grid[x + v] = grid[x]
            return True


dirs = {">": 1, "v": 1j, "<": -1, "^": -1j}
for m in map(dirs.get, moves):
    if push(bot, m):
        grid[bot] = "."
        bot += m


silver = int(sum(p.real + 100 * p.imag for p, c in grid.items() if c == "O"))
gold = 0

print("silver:", silver)
print("gold:", gold)
