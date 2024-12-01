from collections import Counter
from fileinput import input

L, R = zip(*[[int(n) for n in s.split()] for s in input()])

silver = sum(abs(left - right) for left, right in zip(sorted(L), sorted(R)))

L, R = Counter(L), Counter(R)
gold = sum(n * L[n] * R[n] for n in list(L & R))

print("silver:", silver)
print("gold:", gold)
