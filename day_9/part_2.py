x = 1
cycle = 0

def sprite(x):
    return [x-1, x, x+1]


def do_cycle():
    global cycle

    if cycle % 40 == 0:
        print()

    if cycle % 40 in sprite(x):
        print("#", end="")
    else:
        print(" ", end="")

    cycle += 1


with open("input", "r") as f:
    instructions = [l.split(" ") for l in f.read().split("\n")] 

    for instruction in instructions:
        if instruction[0] == "noop":
            do_cycle()

        if instruction[0] == "addx":
            do_cycle()
            do_cycle()

            x += int(instruction[1])

print("\n\n")