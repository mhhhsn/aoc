from fileinput import input

obstacles = set()
seen = set()

xmax, ymax = 0, 0
for idy, line in enumerate(input()):
    ymax = max(ymax, idy)
    for idx, c in enumerate(line.strip()):
        xmax = max(xmax, idx)
        match c:
            case "^":
                pos = complex(idx, idy)
                dir = 0 - 1j
            case "#":
                obstacles.add(complex(idx, idy))


def is_loop(pos, obst):
    seen_ = set()
    dir = 0 - 1j
    while 0 <= pos.real <= xmax and 0 <= pos.imag <= ymax:
        if (pos, dir) in seen_:
            return True
        seen_.add((pos, dir))
        if pos + dir in obstacles or pos + dir == obst:
            # Rotate cw
            dir *= 1j
            continue
        pos += dir
    return False


gold = 0
for idx in range(xmax + 1):
    for idy in range(ymax + 1):
        if complex(idx, idy) == pos:
            continue
        else:
            gold += is_loop(pos, complex(idx, idy))


while 0 <= pos.real <= xmax and 0 <= pos.imag <= ymax:
    seen.add(pos)
    if pos + dir in obstacles:
        # Rotate cw
        dir *= 1j
        continue
    pos += dir


silver = len(seen)

print("silver:", silver)
print("gold:", gold)
