from fileinput import input

reports = [[int(level) for level in report.split()] for report in input()]


def incr(xs):
    return all(x < y and y - x <= 3 for x, y in zip(xs, xs[1:]))


def safe(xs):
    return incr(xs) or incr(xs[::-1])


def drops(xs):
    return (xs[:idx] + xs[idx + 1 :] for idx, _ in enumerate(xs))


silver = sum(safe(rep) for rep in reports)
gold = sum(any(safe(mod) for mod in drops(rep)) for rep in reports)

print("silver:", silver)
print("gold:", gold)
