fn = "input"


trees = set()
max_scenic_score = 0
with open(fn, "r") as f:
    rows = f.read().split("\n")
    rows = [[int(x) for x in row] for row in rows if row]

    height = len(rows)
    width = len(rows[0])
    max_span = max(height, width)

    for i in range(height):
        for j in range(width):
            
            point = rows[i][j]
            view_distance = [0, 0, 0, 0]

            # look up
            for n in range(1, i+1):
                try:
                    view_distance[0] = n

                    if rows[i-n][j] >= point:
                        break
                except:
                    break

            # look left
            for n in range(1, j+1):
                try:
                    view_distance[1] = n

                    if rows[i][j-n] >= point:
                        break
                except:
                    break

            # look down
            for n in range(1, max_span - i):
                try:

                    view_distance[2] = n

                    if rows[i+n][j] >= point:
                        break
                except:
                    break

            # look right
            for n in range(1, max_span - j):
                try:
                    view_distance[3] = n

                    if rows[i][j+n] >= point:
                        break
                except:
                    break

            scenic_score = view_distance[0] * view_distance[1] * view_distance[2] * view_distance[3]
            max_scenic_score = max(max_scenic_score, scenic_score)


print(f"{max_scenic_score=}")