cycles = [20, 60, 100, 140, 180, 220]
v =  0
with open("input", "r") as f:
    instructions = [l.split(" ") for l in f.read().split("\n")] 

    x = 1
    cycle = 0
    for instruction in instructions:
        if instruction[0] == "noop":
            cycle += 1

            if cycle in cycles:
                v += cycle * x

        if instruction[0] == "addx":
            cycle += 2

            if cycle - 1 in cycles:
                v += (cycle-1) * x

            if cycle in cycles:
                v += cycle * x

            x += int(instruction[1])



print(v)