from fileinput import input

disk = list(input())[0].strip()

id_ = -1
blocks = []
tail = 0
file = True
for c in disk:
    blocks.append([tail, tail + int(c), (id_ := id_ + 1) if file else -1])
    tail += int(c)
    file = not file


def expand(bs):
    return [x for a, b, x in sorted(bs) for _ in range(b - a)]


def debug(bs):
    print("".join(str(b) if b != -1 else "." for b in expand(bs)))


# Silver
expanded = expand(blocks)
tail = len(expanded) - 1
fill = expanded.index(-1)
while fill < tail:
    expanded[fill] = expanded[tail]
    expanded[tail] = -1
    while expanded[tail] == -1:
        tail -= 1
    while expanded[fill] != -1:
        fill += 1
silver = sum(idx * n for idx, n in enumerate(expanded) if n != -1)


# Gold
for move in blocks[::-1]:
    print(move)
    ma, mb, mx = move
    if mx == -1:
        continue
    mlen = mb - ma
    for tidx, to in enumerate(blocks):
        ta, tb, tx = to
        if ta >= ma:
            break
        if tx != -1:
            continue
        if (tlen := tb - ta) < mlen:
            continue
        move[2] = -1
        to[2] = mx
        to[1] = ta + mlen
        blocks.insert(tidx + 1, [ta + mlen, tb, -1])
        break
gold = sum(idx * n for idx, n in enumerate(expand(blocks)) if n != -1)

print("silver:", silver)
print("gold:", gold)
