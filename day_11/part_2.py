import operator
from functools import reduce


ops = {
    "+": operator.add,
    "*": operator.mul,
}

with open("input", "r") as f:
    lines = [l.strip(" ") for l in f.read().split("\n")]

    monkey = {}
    m = 0
    for line in lines:
        if line.startswith("Monkey"):
            m = int(line[-2])
            monkey[m] = {}
            monkey[m]["inspections"] = 0
        
        if line.startswith("Starting items"):
            starting_items = list(map(int, line.replace("Starting items: ", "").split(", ")))
            monkey[m]["items"] = starting_items

        if line.startswith("Operation"):
            monkey[m]["ops"] = ops[line.split(" ")[-2]]
            monkey[m]["rh-value"] = line.split(" ")[-1]

        if line.startswith("Test"):
            monkey[m]["test"] = int(line.split(" ")[-1])

        if line.startswith("If true"):
            monkey[m]["true"] = int(line.split(" ")[-1])

        if line.startswith("If false"):
            monkey[m]["false"] = int(line.split(" ")[-1])

    x = reduce(operator.mul, [v["test"] for _, v in monkey.items()])
    n_rounds = 10000
    for _ in range(n_rounds):
        for k, v in monkey.items():
            try:
                while old:=v["items"].pop(0):
                    if v["rh-value"] == "old":
                        rh_value = old
                    else:
                        rh_value = int(v["rh-value"])
                    new = v["ops"](old, rh_value)

                    new = new % x

                    if new % v["test"] == 0:
                        monkey[v["true"]]["items"].append(new)
                    else:
                        monkey[v["false"]]["items"].append(new)

                    v["inspections"] += 1

            except IndexError:
                pass

print(operator.mul(*sorted([v["inspections"] for k, v in monkey.items()])[-2:]))