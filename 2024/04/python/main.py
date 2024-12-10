from fileinput import input

grid = {
    complex(idx, idy): c
    for idy, line in enumerate(input())
    for idx, c in enumerate(line.strip())
}
dirs = {complex(dx, dy) for dx in [-1, 0, 1] for dy in [-1, 0, 1]}


def xmas(x, v):
    return all(grid.get(x + n * v) == c for n, c in enumerate("XMAS"))


def x_mas(x, v):
    return (
        grid.get(x) == "A"
        and grid.get(x + v) == grid.get(x + 1j * v) == "M"
        and grid.get(x - v) == grid.get(x - 1j * v) == "S"
    )


silver = sum(xmas(x, v) for x in grid.keys() for v in dirs)
gold = sum(x_mas(x, v) for x in grid.keys() for v in dirs if abs(v) > 1)

print("silver:", silver)
print("gold:", gold)
