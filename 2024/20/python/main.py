from fileinput import input

grid = {
    complex(idx, idy): c
    for idy, line in enumerate(input())
    for idx, c in enumerate(line.strip())
}
start = next(p for p, c in grid.items() if c == "S")
end = next(p for p, c in grid.items() if c == "E")

times = {}
q = [(start, 0)]
while q:
    curr, t = q.pop()
    if curr in times:
        continue
    times[curr] = t
    for d in [1, -1, 1j, -1j]:
        x = curr + d
        if x not in grid or grid[x] == "#":
            continue
        q.append((x, t + 1))

cheats = []
for wall in (p for p, c in grid.items() if c == "#"):
    for d in [1, -1, 1j, -1j]:
        if (x := wall - d) not in grid or grid[x] == "#":
            continue
        if (y := wall + d) not in grid or grid[y] == "#":
            continue
        if (saved := times[y] - times[x] - 2) <= 0:
            continue
        cheats.append(saved)

silver = sum(1 for c in cheats if c >= 100)
gold = 0

print("silver:", silver)
print("gold:", gold)
