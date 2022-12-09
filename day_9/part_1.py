from collections import defaultdict


moves_dict = {
    "R": (1, 0),
    "U": (0, 1),
    "L": (-1, 0),
    "D": (0, -1),
}

s = (0, 0)
head, tail = s, s

visited = defaultdict(lambda: 0)
visited[s] += 1

with open("input", "r") as f:
    moves = [l.split(" ") for l in f.read().split("\n")] 

    for move in moves:
        m = moves_dict[move[0]]
        for i in range(int(move[1])):
            prev_head = head
            head = (head[0] + m[0], head[1] + m[1])

            if abs(tail[0] - head[0]) >= 2 or abs(tail[1] - head[1]) >= 2:
                tail = prev_head
                visited[tail] += 1

print(len(visited))