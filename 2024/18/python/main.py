from bisect import bisect_left
from collections import deque
from fileinput import input

coords = [line.strip().split(",") for line in input()]
grid = {complex(int(x), int(y)): t for t, [x, y] in enumerate(coords)}


def solve(size, delay):
    q = deque([(0, 0)])
    seen = set()
    while q:
        curr, t = q.popleft()
        if curr in seen:
            continue
        if curr == complex(size, size):
            return t
        seen.add(curr)

        for d in [1, -1, 1j, -1j]:
            x = curr + d
            if not (0 <= x.real <= size and 0 <= x.imag <= size):
                continue
            if x in grid and grid[x] < delay:
                continue
            q.append((x, t + 1))
    return None


silver = solve(70, 1024)
# silver = solve(6, 12)  # test.txt

gold = ",".join(
    coords[
        bisect_left(
            list(range(len(grid))),
            True,
            key=lambda x: solve(70, x) is None,
        )
        - 1
    ]
)

print("silver:", silver)
print("gold:", gold)
