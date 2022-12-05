with open("input", "r") as f:
    pairs = f.read().split("\n")

    n_fully_contained = 0
    n_has_overlap = 0
    for pair in pairs:
        pair = pair.split(",")

        a = [int(x) for x in pair[0].split("-")]
        b = [int(x) for x in pair[1].split("-")]

        if a[0] <= b[0] and a[1] >= b[1] or b[0] <= a[0] and b[1] >= a[1]:
            n_fully_contained += 1

        if b[1] >= a[0] and b[0] <= a[1]:
            n_has_overlap += 1
        
print(f"{n_fully_contained=}")  # part 1
print(f"{n_has_overlap=}")      # part 2
