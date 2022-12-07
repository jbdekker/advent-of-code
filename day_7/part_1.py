fn = "input"

max_size = 100000

sizes = {}
def walk(lines, cwd):
    if cwd not in sizes.keys():
        sizes[cwd] = 0

    while True:
        try:
            line = lines.pop(0)

            if not line or line == "$ cd ..":
                return sizes[cwd]

            elif line.startswith("$ cd"):
                sizes[cwd] += walk(lines, cwd + line.split(" ")[2] + "/")
                continue   
            
            elif line == "$ ls" or line.startswith("dir "):
                continue

            else:
                sizes[cwd] += int(line.split(" ")[0])
        except:
            return sizes[cwd]


with open(fn, "r") as f:
    lines = f.read().split("\n")

    walk(lines, "")

    for k, v in sizes.items():
        if k:
            print(f"{k} \t\t\t {v}")

    
    print(sum([v for _, v in sizes.items() if v <= max_size]))