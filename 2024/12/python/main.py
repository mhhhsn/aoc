from fileinput import input

grid = {
    complex(idx, idy): c
    for idy, line in enumerate(input())
    for idx, c in enumerate(line.strip())
}
xmax = int(max(x.real for x in grid.keys()))
ymax = int(max(x.imag for x in grid.keys()))

seen = set()

silver = 0

for x in range(xmax + 1):
    for y in range(ymax + 1):
        c = complex(x, y)
        if c in seen:
            continue

        # Flood fill
        char = grid[c]
        perimeter = area = 0
        queue = [c]
        while queue:
            curr = queue.pop()
            if curr in seen:
                continue
            seen.add(curr)

            for dir in [1, -1, 1j, -1j]:
                next_ = curr + dir
                if (
                    not (0 <= next_.real <= xmax and 0 <= next_.imag <= ymax)
                    or grid[next_] != char
                ):
                    perimeter += 1
                    continue

                queue.append(next_)
            area += 1
        silver += perimeter * area

gold = 0

print("silver:", silver)
print("gold:", gold)
