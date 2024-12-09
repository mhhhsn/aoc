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


expand_blocks = []
for a, b, x in sorted(blocks):
    for _ in range(b - a):
        expand_blocks.append(x)


def debug(xs):
    for a, b, x in sorted(xs):
        if x == -1:
            x = "."
        print(str(x) * (b - a), end="")
    print()


# Silver
tail = len(expand_blocks) - 1
fill = expand_blocks.index(-1)
while fill < tail:
    expand_blocks[fill] = expand_blocks[tail]
    expand_blocks[tail] = -1
    while expand_blocks[tail] == -1:
        tail -= 1
    while expand_blocks[fill] != -1:
        fill += 1

silver = sum(idx * n for idx, n in enumerate(expand_blocks) if n != -1)

print(blocks)

gold = 0

print("silver:", silver)
print("gold:", gold)
