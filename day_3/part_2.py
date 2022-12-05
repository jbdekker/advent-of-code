import functools
import itertools
from typing import Tuple
from typing import List
from typing import Set

fp = "input"


def split_list(a: List[str]) -> Tuple[set, set]:
    half = len(a) // 2
    return set(a[:half]), set(a[half:])


def priority(v: str) -> int:
    i = ord(v)
    return i - 96 if i >= 96 else i - 38


def find_common(groups: List[List[str]]) -> List[Set[str]]:
    return [functools.reduce(lambda a, b: set(a) & set(b), grp) for grp in groups]


with open(fp, "r") as f:
    lines = [l.strip("\n") for l in f.readlines()]


badges = find_common([lines[i : i + 3] for i in range(0, len(lines), 3)])
priorities = map(priority, itertools.chain(*badges))

print(sum(priorities))
