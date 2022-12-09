from collections import defaultdict


moves_dict = {
    "R": (1, 0),
    "U": (0, 1),
    "L": (-1, 0),
    "D": (0, -1),
}

n_knots = 10
s = (0, 0)
knots = [s for _ in range(n_knots)]

visited = defaultdict(lambda: 0)

sign = lambda v: 1 if v > 0 else (-1 if v < 0 else 0)

def show():
    def get_knot(x, y):
        for i, knot in enumerate(knots):
            if knot == (x, y):
                return i
        return None

    for y in range(5, -1, -1):
        for x in range(0, 6, 1):
            knot_id = get_knot(x, y)
            if knot_id is not None:
                if knot_id == 0:
                    print("H", end="")
                else:
                    print(knot_id, end="")
            else:
                print(".", end="")
        print("")
    print("")

with open("input", "r") as f:
    moves = [l.split(" ") for l in f.read().split("\n")] 

    for move in moves:
        m = moves_dict[move[0]]
        for i in range(int(move[1])):
            knots[0] = (knots[0][0] + m[0], knots[0][1] + m[1])

            for j in range(1, n_knots):
                dx = knots[j-1][0] - knots[j][0]
                dy = knots[j-1][1] - knots[j][1]
                if abs(dx) >= 2 or abs(dy) >= 2:
                    knots[j] = (
                        knots[j][0] + sign(dx), 
                        knots[j][1] + sign(dy)
                        )

            visited[knots[-1]] += 1
            
print(len(visited))