import operator
from tqdm import tqdm
from typing import List

def value(v):
    return ord(v)


class Node():
    """A node class for A* Pathfinding"""

    def __init__(self, parent=None, position=None):
        self.parent = parent
        self.position = position

        self.g = 0
        self.h = 0
        self.f = 0

    def __eq__(self, other):
        return self.position == other.position


def get_neighbours(grid: List[List[int]], parent: Node, end: Node) -> List[Node]:
    children = []
    for action in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
        
        try: 
            child_position = (parent.position[0] + action[0], parent.position[1] + action[1])

            h_parent = grid[parent.position[0]][parent.position[1]]
            h_child = grid[child_position[0]][child_position[1]]

            if h_child - h_parent > 1:
                continue

            child = Node(parent=parent, position=child_position)

            child.g = parent.g + 1
            child.h = ((child.position[0] - end.position[0]) ** 2) + ((child.position[1] - end.position[1]) ** 2)
            child.f = child.g + child.h

            children.append(child)

        except IndexError:
            pass
        
    return children


def astar(grid, start, end):
    """Returns a list of tuples as a path from the given start to the given end in the given maze"""

    # Create start and end node
    start_node = Node(None, start)
    start_node.g = start_node.h = start_node.f = 0
    end_node = Node(None, end)
    end_node.g = end_node.h = end_node.f = 0

    # Initialize both open and closed list
    open_list = []
    closed_list = []

    # Add the start node
    open_list.append(start_node)

    # Loop until you find the end
    while open_list:

        # Get the current node
        # min_f = open_set[0].f
        # current_index = 0
        ff = [x.f for x in open_list]
        idx = ff.index(min(ff))

        # Pop current off open list, add to closed list
        current_node = open_list.pop(idx)
        closed_list.append(current_node)

        # Found the goal
        if current_node == end_node:
            path = []
            current = current_node
            while current is not None:
                path.append(current.position)
                current = current.parent
            return path[::-1] # Return reversed path

        # Generate children
        children = []
        for action in [(0, -1), (0, 1), (-1, 0), (1, 0)]: # Adjacent squares
            try:
                # Get node position
                child = (current_node.position[0] + action[0], current_node.position[1] + action[1])

                # can we walk there?
                h1 = grid[current_node.position[0]][current_node.position[1]]
                h2 = grid[child[0]][child[1]]
                if abs(h1 - h2) > 1:
                    continue

                children.append(Node(current_node, child))



            except IndexError:
                pass  # out of the grid

        # Loop through children
        for child in children:

            # Child is on the closed list
            if child in closed_list:
                continue

            # Create the f, g, and h values
            child.g = current_node.g + 1
            child.h = ((child.position[0] - end_node.position[0]) ** 2) + ((child.position[1] - end_node.position[1]) ** 2)
            child.f = child.g + child.h

            if child in open_list:
                open_node = [n for n in open_list if n == child][0]
                if open_node.g > child.g:
                    open_node.g = child.g
                    open_node.f = child.f
                    open_node.parent = child.parent
            else:
                open_list.append(child)


grid = []
with open("input", "r") as f:
    grid = [[ord(c) for c in line] for line in f.read().split("\n")]

    h = len(grid)
    w = len(grid[0])

    E = [(i, j) for i in range(h) for j in range(w) if grid[i][j] == 69][0]
    S = [(i, j) for i in range(h) for j in range(w) if grid[i][j] == 83][0]

    grid[E[0]][E[1]] = ord("z")
    grid[S[0]][S[1]] = ord("a")

    path = astar(grid, S, E)

print(E)
print(S)
print(path)
print(len(path)-1)