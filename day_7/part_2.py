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

    
    total_disk_space = 70000000
    free_space_required = 30000000
    used_space = sizes["//"]

    deltas = [v for _, v in sizes.items() if used_space - v <= 70000000 - 30000000]
    print(min(deltas))

