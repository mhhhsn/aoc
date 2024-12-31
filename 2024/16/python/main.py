from fileinput import input

grid = {
    complex(idx, idy): c
    for idy, line in enumerate(input())
    for idx, c in enumerate(line.strip())
}
start = next(p for p, c in grid.items() if c == "S")
end = next(p for p, c in grid.items() if c == "E")

q = [(0, (start, 1))]
seen = set()
while q:
    curr = min(q, key=lambda x: x[0])
    d, (pos, vel) = curr
    q.remove(curr)
    if (pos, vel) in seen:
        continue
    if pos == end:
        silver = d
        break
    seen.add((pos, vel))

    if grid[pos + vel] != "#":
        q.append((d + 1, (pos + vel, vel)))
    q.append((d + 1000, (pos, vel * 1j)))
    q.append((d + 1000, (pos, vel * -1j)))

print("silver:", silver)
