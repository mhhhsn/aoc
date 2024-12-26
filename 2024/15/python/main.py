from fileinput import input
from itertools import takewhile

inp = map(str.strip, input())

grid = {
    complex(x, y): c
    for y, line in enumerate(takewhile(bool, inp))
    for x, c in enumerate(line)
}
grid_gold = {}
for p, c in grid.items():
    p += p.real
    match c:
        case "#" | ".":
            grid_gold[p] = grid_gold[p + 1] = c
        case "O":
            grid_gold[p] = "["
            grid_gold[p + 1] = "]"
        case "@":
            grid_gold[p] = "@"
            grid_gold[p + 1] = "."


moves = "".join(inp)

bot = next(p for p, c in grid.items() if c == "@")
bot_gold = bot + bot.real


def debug(grid):
    xmax, ymax = (max(p.real for p in grid.keys()), max(p.imag for p in grid.keys()))
    for y in range(int(ymax) + 1):
        for x in range(int(xmax) + 1):
            print(grid[complex(x, y)], end="")
        print()
    print()


def push(grid, x, v, modify=True):
    match grid[x], v:
        case ".", _:
            return True
        case "#", _:
            return False
        case "[", (1j | -1j) if push(grid, x + v, v, modify) and push(
            grid, x + v + 1, v, modify
        ):
            if modify:
                grid[x + v] = grid[x]
                grid[x + v + 1] = grid[x + 1]
                grid[x] = "."
                grid[x + 1] = "."
            return True
        case "[", (1j | -1j):
            return False
        # Push other one
        case "]", (1j | -1j):
            return push(grid, x - 1, v, modify)
        case _, _ if push(grid, x + v, v, modify):
            if modify:
                grid[x + v] = grid[x]
            return True


dirs = {">": 1, "v": 1j, "<": -1, "^": -1j}
for m in map(dirs.get, moves):
    if push(grid, bot, m):
        grid[bot] = "."
        bot += m
    if push(grid_gold, bot_gold, m, modify=False):
        push(grid_gold, bot_gold, m, modify=True)
        grid_gold[bot_gold] = "."
        bot_gold += m


silver = int(sum(p.real + 100 * p.imag for p, c in grid.items() if c == "O"))
gold = int(sum(p.real + 100 * p.imag for p, c in grid_gold.items() if c == "["))

print("silver:", silver)
print("gold:", gold)
