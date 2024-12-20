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


def cheat(x, y, d):
    dist = int(abs(x.real - y.real) + abs(x.imag - y.imag))
    if dist > d:
        return False
    saved = times[y] - times[x] - dist
    return saved >= 100


silver = sum(cheat(x, y, 2) for x in times.keys() for y in times.keys())
gold = sum(cheat(x, y, 20) for x in times.keys() for y in times.keys())


print("silver:", silver)
print("gold:", gold)
