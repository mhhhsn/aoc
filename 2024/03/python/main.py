import re
from fileinput import input

s = "\n".join(input())

silver = sum(int(x) * int(y) for x, y in re.findall(r"mul\((\d{1,3}),(\d{1,3})\)", s))

gold = 0
active = True
for g in re.findall(r"(?:mul\((\d{1,3}),(\d{1,3})\))|(don't\(\))|(do\(\))", s):
    match g, active:
        case (x, y, "", ""), True:
            gold += int(x) * int(y)
        case (_, _, "don't()", _), _:
            active = False
        case (_, _, _, "do()"), _:
            active = True


print("silver:", silver)
print("gold:", gold)
