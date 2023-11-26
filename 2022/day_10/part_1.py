m_d = {
    "R": (1, 0),
    "U": (0, 1),
    "L": (-1, 0),
    "D": (0, -1),
}

head, tail = (0, 0), (0, 0)

visited = set()

with open("input", "r") as f:
    moves = [l.split(" ") for l in f.read().split("\n")] 

    for move in moves:
        m = m_d[move[0]]
        for i in range(int(move[1])):
            prev_head = head
            head = (head[0] + m[0], head[1] + m[1])

            dx = tail[0] - head[0]
            dy = tail[1] - head[1]

            if abs(dx) >= 2 or abs(dy) >= 2:
                tail = prev_head
            
            visited.add(tail)

print(len(visited))