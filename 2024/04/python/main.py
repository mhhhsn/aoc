import sys

with open(sys.argv[1], "r") as f:
    grid = [line.strip() for line in f]

min_x = 0
min_y = 0
max_x = len(grid)
max_y = len(grid[0])


def count_xmas(sx, sy):
    total = 0
    for dx in [-1, 0, 1]:
        for dy in [-1, 0, 1]:
            for n, char in enumerate("XMAS"):
                x = sx + n * dx
                y = sy + n * dy
                if not (min_x <= x < max_x and min_y <= y < max_y):
                    break
                if grid[x][y] != char:
                    break
            else:
                total += 1
    return total


def count_x_mas(sx, sy):
    if sx == min_x or sx == max_x - 1 or sy == min_y or sy == max_y - 1:
        return 0
    if grid[sx][sy] != "A":
        return 0
    mmss = "".join(
        (
            grid[sx - 1][sy - 1],
            grid[sx - 1][sy + 1],
            grid[sx + 1][sy + 1],
            grid[sx + 1][sy - 1],
        ),
    )
    return mmss in {"MMSS", "MSSM", "SSMM", "SMMS"}


silver = sum(count_xmas(x, y) for x in range(max_x) for y in range(max_y))
gold = sum(count_x_mas(x, y) for x in range(max_x) for y in range(max_y))
print("silver:", silver)
print("gold:", gold)
