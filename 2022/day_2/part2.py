fp = "input"

shape_value = {
    "A": 1,  # rock
    "B": 2,  # paper
    "C": 3,  # scissors
}

to_win = {  
    "A": "B",
    "B": "C",
    "C": "A",
}

to_lose = {  
    "A": "C",
    "B": "A",
    "C": "B",
}


with open(fp, "r") as f:
    rounds =  [line.strip("\n").split(" ") for line in f.readlines()]

score = 0
for r in rounds:
    if r[1] == "X":
        choice = to_lose[r[0]]

    if r[1] == "Y":
        choice = r[0]
        score += 3

    if r[1] == "Z":
        choice = to_win[r[0]]
        score += 6

    score += shape_value[choice]

print(f"{score=}")