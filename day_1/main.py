fp = "./input"

with open(fp, "r") as f:
    lines = f.readlines()

counts = []
n = 0
for line in lines:
    try:
        n += int(line)
    except ValueError:
        counts.append(n)
        n = 0

res = sorted(counts, reverse=True)

print(f"Top elf: {res[0]} Calories")
print(f"Sum of top 3 elves: {sum(res[:3])} Calories")