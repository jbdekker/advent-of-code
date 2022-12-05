from typing import List
from typing import Tuple

fp = "input"


def str_to_int_interval(interval: List[str]) -> List[int]:
    return list(map(int, interval))


def parse_pair(pair: List[str]) -> List[Tuple[int]]:
    return list(map(str_to_int_interval, [p.split("-") for p in pair]))


def to_section_pairs(lines: List[str]) -> List[Tuple[Tuple[int]]]:
    return list(map(parse_pair, [l.split(",") for l in lines]))


def left_contains_right(a: List[int], b: List[int]) -> bool:
    return a[0] <= b[0] and a[1] >= b[1]


def one_contains_the_other(a: List[int], b: List[int]) -> bool:
    return left_contains_right(a, b) or left_contains_right(b, a):


def has_overlap(a: List[int], b: List[int]) -> bool:
    return b[1] >= a[0] and b[0] <= a[1]


with open(fp, "r") as f:
    lines = [l.strip("\n") for l in f.readlines()]

n_fully_contained = sum([one_contains_the_other(*p) for p in to_section_pairs(lines)])
n_has_overlap = sum([has_overlap(*p) for p in pairs])

print(f"{n_fully_contained=}")
print(f"{n_has_overlap=}")
