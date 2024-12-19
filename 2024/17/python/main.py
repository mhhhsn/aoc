from fileinput import input
from itertools import takewhile, zip_longest

inp = map(str.strip, input())
regs = [int(r[2]) for r in (line.split() for line in takewhile(bool, inp))]
prog = [int(o) for o in next(inp).split()[1].split(",")]
out = []


def run(prog, ra, rb, rc):
    out = []
    ir = 0
    while ir < len(prog):
        instr, op = prog[ir], prog[ir + 1]
        combo = {4: ra, 5: rb, 6: rc}
        cop = combo.get(op, op)
        match instr:
            case 0:
                ra = ra >> cop
            case 1:
                rb ^= op
            case 2:
                rb = cop & 0b111
            case 3 if ra:
                ir = op - 2
            case 4:
                rb ^= rc
            case 5:
                yield cop & 0b111
            case 6:
                rb = ra >> cop
            case 7:
                rc = ra >> cop
        ir += 2
    return out


def match(prog, ra):
    return (
        got == want
        for got, want in zip_longest(
            reversed(list(run(prog, ra, 0, 0))),
            reversed(prog),
        )
    )


def find_a(prog):
    q = list(range(8))
    while True:
        curr = q.pop(0)
        if all(match(prog, curr)):
            return curr

        best = sum(match(prog, curr))
        for n in range(8):
            ra = (curr << 3) + n
            if sum(match(prog, ra)) > best:
                q.append(ra)


silver = ",".join(map(str, run(prog, *regs)))
gold = find_a(prog)

print("silver:", silver)
print("gold:", gold)
