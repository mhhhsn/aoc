import re
from fileinput import input
from itertools import batched

lines = [line.strip() for line in input()]


re_button = r"Button [A|B]: X\+(\d+), Y\+(\d+)"
re_prize = r"Prize: X=(\d+), Y=(\d+)"


def solve(machine, gold):
    m = re.fullmatch(
        r"Button A: X\+(\d+), Y\+(\d+)Button B: X\+(\d+), Y\+(\d+)Prize: X=(\d+), Y=(\d+)",
        "".join(machine),
    )
    # 2x2 matrix solve
    ax, ay, bx, by, px, py = map(int, m.groups())
    if gold:
        px += 10000000000000
        py += 10000000000000
    det = ax * by - ay * bx

    A = (by * px - bx * py) / det
    B = (-ay * px + ax * py) / det

    # Check solutions are ints
    if abs(int(A) - A) < 1e-7 and abs(int(B) - B) < 1e-7:
        return 3 * int(A) + int(B)
    else:
        return 0


silver = sum(solve(b, False) for b in batched(lines, 4))
gold = sum(solve(b, True) for b in batched(lines, 4))

print("silver:", silver)
print("gold:", gold)
