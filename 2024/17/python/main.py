from fileinput import input
from itertools import takewhile

inp = map(str.strip, input())
regs = {r[1][0]: int(r[2]) for r in (line.split() for line in takewhile(bool, inp))}
prog = [int(o) for o in next(inp).split()[1].split(",")]
out = []

ir = 0
combo = {
    0: lambda: 0,
    1: lambda: 1,
    2: lambda: 2,
    3: lambda: 3,
    4: lambda: regs["A"],
    5: lambda: regs["B"],
    6: lambda: regs["C"],
    7: None,
}


def _adv(op):
    regs["A"] = regs["A"] >> combo[op]()


def _bxl(op):
    regs["B"] ^= op


def _bst(op):
    regs["B"] = combo[op]() & 0b111


def _jnz(op):
    global ir
    if regs["A"]:
        ir = op
        ir -= 2  # Undo IR


def _bxc(_):
    regs["B"] ^= regs["C"]


def _out(op):
    out.append(combo[op]() & 0b111)


def _bdv(op):
    regs["B"] = regs["A"] >> combo[op]()


def _cdv(op):
    regs["C"] = regs["A"] >> combo[op]()


instrs = {
    0: _adv,
    1: _bxl,
    2: _bst,
    3: _jnz,
    4: _bxc,
    5: _out,
    6: _bdv,
    7: _cdv,
}

while ir <= len(prog) - 1:
    inst, op = prog[ir], prog[ir + 1]
    instrs[inst](op)
    ir += 2

silver = ",".join(map(str, out))
gold = 0

print("silver:", silver)
print("gold:", gold)
