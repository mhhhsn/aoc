from fileinput import input

lines = [line.strip() for line in input()]


def solve(w, acc, rest, gold=False):
    if not rest:
        return w == acc
    if w < acc:
        return False
    head, *rest = rest
    return (
        solve(w, acc + head, rest, gold)
        or solve(w, acc * head, rest, gold)
        or (gold and solve(w, int(str(acc) + str(head)), rest, gold))
    )


silver = 0
gold = 0

for line in lines:
    want, nums = line.split(":")
    want = int(want)
    nums = [int(n) for n in nums.split()]
    head, *rest = nums

    if solve(want, head, rest):
        silver += want
    if solve(want, head, rest, True):
        gold += want


print("silver:", silver)
print("gold:", gold)
