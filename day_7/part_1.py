fn = "example_input"


with open(fn, "r") as f:
    rows = f.read().split("\n")
    rows = [int(x) for x in rows]

print(rows)