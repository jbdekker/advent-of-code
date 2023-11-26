m_d = {
    "R": (1, 0),
    "U": (0, 1),
    "L": (-1, 0),
    "D": (0, -1),
}

n = 10
knots = [(0, 0) for _ in range(n)]

visited = set()

sign = lambda v: 1 if v > 0 else (-1 if v < 0 else 0)

with open("input", "r") as f:
    moves = [l.split(" ") for l in f.read().split("\n")]

    for move in moves:
        m = m_d[move[0]]
        for i in range(int(move[1])):
            knots[0] = (knots[0][0] + m[0], knots[0][1] + m[1])

            for j in range(1, n):
                dx = knots[j - 1][0] - knots[j][0]
                dy = knots[j - 1][1] - knots[j][1]
                if abs(dx) >= 2 or abs(dy) >= 2:
                    knots[j] = (knots[j][0] + sign(dx), knots[j][1] + sign(dy))

            visited.add(knots[-1])

print(len(visited))
