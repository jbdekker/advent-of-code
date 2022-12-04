fp = "./day_2/input"

shape_value = {
    "X": 1,  # rock
    "Y": 2,  # paper
    "Z": 3,  # scissors
}

beats = {
    "X": "C",
    "Y": "A",
    "Z": "B",
}

with open(fp, "r") as f:
    rounds =  [line.strip("\n").split(" ") for line in f.readlines()]

score = 0
for r in rounds:
    score += shape_value[r[1]]

    if chr(ord(r[1]) - 23) == r[0]:
        score += 3

    if r[0] == beats[r[1]]:
        score += 6

print(f"{score=}")