from enum import Enum
import ast


class Decision(Enum):
    RIGHT_ORDER = 0
    WRONG_ORDER = 1
    UNDECIDED = 2


def evaluate(a, b) -> bool:
    for x, y in zip(a, b):
        if type(x) != type(y):
            if isinstance(x, int) and isinstance(y, list):
                x = [x]
            elif isinstance(x, list) and isinstance(y, int):
                y = [y]
            else:
                raise ValueError("!!!")

        if isinstance(x, int) and isinstance(y, int):
            """
            If both values are integers, the lower integer should come first. 
            If the left integer is lower than the right integer, the inputs 
            are in the right order. If the left integer is higher than the 
            right integer, the inputs are not in the right order. Otherwise, 
            the inputs are the same integer; continue checking the next part 
            of the input.
            """

            if x < y:
                return Decision.RIGHT_ORDER
            elif x > y:
                print(f"Wrong order because: {x} > {y}")
                print(f"\tInputs: \n\t\tLeft:\t{a}\n\t\tright: \t{b}")
                return Decision.WRONG_ORDER

        elif isinstance(x, list) and isinstance(y, list):
            """
            Compare the first value of each list, then the second value, and so 
            on. If the left list runs out of items first, the inputs are in the 
            right order. If the right list runs out of items first, the inputs 
            are not in the right order. If the lists are the same length and no 
            comparison makes a decision about the order, continue checking the 
            next part of the input.
            """
            res = evaluate(x, y)
            if res != Decision.UNDECIDED:
                return res
        else:
            raise ValueError("!!!")

    if len(a) > len(b):
        print(f"Wrong order because: length {a} > length {b}")
        print(f"\tInputs: \n\t\tLeft:\t{a}\n\t\tright: \t{b}")
        return Decision.WRONG_ORDER  # right runs out of items first
    elif len(a) < len(b):
        return Decision.RIGHT_ORDER
    
    return Decision.UNDECIDED


with open("input", "r") as f:
    lines = f.read().split("\n")

    data = []
    for line in lines:
        if line.strip(" ") != "":
            data.append(ast.literal_eval(line))

    flags = {}
    for idx in range(0, len(data), 2):
        print("")
        flags[1 + idx // 2] = evaluate(*data[idx : idx + 2])

    print(sum([k for k, v in flags.items() if v == Decision.RIGHT_ORDER]))

# print(flags)
