from fileinput import input

grid = [[int(c) for c in line.strip()] for line in input()]

starts = {
    (idx, idy) for idy, line in enumerate(grid) for idx, n in enumerate(line) if n == 0
}

# print(*grid, sep="\n")
# print(starts)


def score_trailhead(start, gold):
    score = 0
    q = [start]
    seen = set()
    while q:
        x, y = q.pop()
        seen.add((x, y))

        h = grid[y][x]
        if h == 9:
            score += 1
            continue
        for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nx, ny = x + dx, y + dy
            if ((nx, ny) in seen and not gold) or not (
                0 <= nx < len(grid) and 0 <= ny < len(grid[0])
            ):
                continue

            if grid[ny][nx] == h + 1:
                q.append((nx, ny))
    return score


silver = sum(score_trailhead(start, False) for start in starts)
gold = sum(score_trailhead(start, True) for start in starts)

print("silver:", silver)
print("gold:", gold)
