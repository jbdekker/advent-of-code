from typing import Tuple
from typing import List

fp = "input"


def split_list(a: List[str]) -> Tuple[set, set]:
    half = len(a) // 2
    return set(a[:half]), set(a[half:])


def priority(v: str) -> int:
    i = ord(v)
    return i - 96 if i >= 96 else i - 38
    

with open(fp, "r") as f:
    lines =  [l.strip("\n") for l in f.readlines()]

rucksacks = [split_list(list(l)) for l in lines]
misplaced_items = [r[0] & r[1] for r in rucksacks]
priorities = [priority(v) for i in misplaced_items for v in i]

print(sum(priorities))