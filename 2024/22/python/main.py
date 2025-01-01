from collections import Counter, deque
from fileinput import input
from itertools import islice

lines = [int(line.strip()) for line in input()]


def secrets(secret):
    x = secret
    while True:
        yield x
        x ^= x << 6
        x &= 0xFFFFFF
        x ^= x >> 5
        x &= 0xFFFFFF
        x ^= x << 11
        x &= 0xFFFFFF


def prices(it):
    for s in it:
        yield s % 10


def diff(it):
    prev = next(it)
    for curr in it:
        yield curr, curr - prev
        prev = curr


def to_seqs(it, n=4):
    head = map(lambda x: x[1], islice(it, n - 1))
    q = deque(head, maxlen=n)
    for price, diff in it:
        q.append(diff)
        yield price, tuple(q)


def seq_sells(it):
    sells = {}
    for price, diff in it:
        if diff in sells:
            continue
        sells[diff] = price
    return sells


best_seqs = Counter()
for x in lines:
    best_seqs += seq_sells(to_seqs(islice(diff(prices(secrets(x))), 2000)))

silver = sum(next(islice(secrets(x), 2000, None)) for x in lines)
gold = best_seqs.most_common(1)[0][1]

print("silver:", silver)
print("gold:", gold)
