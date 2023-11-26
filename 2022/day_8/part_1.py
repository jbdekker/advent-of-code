fn = "input"


trees = set()

with open(fn, "r") as f:
    rows = f.read().split("\n")
    rows = [[int(x) for x in row] for row in rows if row]

    height = len(rows)
    width = len(rows[0])

    for i in range(1, height-1):
        for j in range(1, width-1):
            # look right
            if max(rows[i][:j]) < rows[i][j]:
                trees.add((i, j))
            
            # look left
            if max(rows[i][j+1:]) < rows[i][j]:
                trees.add((i, j))
            
            # look up
            if max([x[j] for x in rows[:i]]) < rows[i][j]:
                trees.add((i, j))
            
            # look down
            if max([x[j] for x in rows[i+1:]]) < rows[i][j]:
                trees.add((i, j))

n_edge_trees = 2 * height + 2 * width - 4

print(n_edge_trees + len(trees))