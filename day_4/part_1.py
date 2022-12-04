from typing import List
from typing import Tuple

fp = "input"


def str_to_int_interval(interval: List[str]) -> List[int]:
    # ['2', '4']
    return list(map(int, interval))


def parse_pair(pair: List[str]) -> List[Tuple[int]]:
    # pair: ['2-4', '6-8']
    pair = [p.split("-") for p in pair]
    return list(map(str_to_int_interval, pair))


def to_section_pairs(lines: List[str]) -> List[Tuple[Tuple[int]]]:
    pairs = [l.split(",") for l in lines]
    pairs = list(map(parse_pair, pairs))
    return pairs


def left_contains_right(a: List[int], b: List[int]) -> bool:
    if a[0] <= b[0] and a[1] >= b[1]:
        return True
    return False


def one_contains_the_other(a: List[int], b: List[int]) -> bool:
    if left_contains_right(a, b) or left_contains_right(b, a):
        return True
    return False


def has_overlap(a: List[int], b: List[int]) -> bool:
    return b[1] >= a[0] and b[0] <= a[1]


with open(fp, "r") as f:
    lines = [l.strip("\n") for l in f.readlines()]

pairs = to_section_pairs(lines)
fully_contained = sum([one_contains_the_other(p[0], p[1]) for p in pairs])
has_overlap = sum([has_overlap(p[0], p[1]) for p in pairs])

print(f"{fully_contained=}")
print(f"{has_overlap=}")
