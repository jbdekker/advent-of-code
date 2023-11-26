import operator
from tqdm import tqdm
from typing import List
from typing import Tuple

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


def grid_value(grid: List[List[int]], node: Node) -> int:
    return grid[node.position[0]][node.position[1]]


def get_neighbours(grid: List[List[int]], parent: Node) -> List[Node]:
    neighbours = []
    for action in [(0, -1), (0, 1), (-1, 0), (1, 0)]:
        neighbour_position = (parent.position[0] + action[0], parent.position[1] + action[1])

        if not 0 <= neighbour_position[0] < len(grid):
            continue

        if not 0 <= neighbour_position[1] < len(grid[0]):
            continue
        
        neighbour = Node(parent=parent, position=neighbour_position)

        neighbours.append(neighbour)
    return neighbours


def get_children(grid: List[List[int]], parent: Node, end_node: Node) -> List[Node]:
    children = []
    for child in get_neighbours(grid, parent):
        
        try: 
            h_parent = grid[parent.position[0]][parent.position[1]]
            h_child = grid[child.position[0]][child.position[1]]

            if h_parent - h_child > 1:
                continue

            child.g = parent.g + 1
            child.h = 0
            child.f = child.g + child.h

            children.append(child)

        except IndexError:
            pass  # off-grid
        
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
        ff = [x.f for x in open_list]
        idx = ff.index(min(ff))

        # Pop current off open list, add to closed list
        current_node = open_list.pop(idx)
        closed_list.append(current_node)

        # Found the goal
        if grid_value(grid, current_node) == grid_value(grid, end_node):
            path = []
            current = current_node
            while current is not None:
                path.append(current.position)
                current = current.parent
            return path[::-1] # Return reversed path

        # Generate children
        children = get_children(grid, current_node, end_node)
        for child in children:
            if child in closed_list:
                continue

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

    path = astar(grid, end=S, start=E)

print(E)
print(S)
print(path)
print(len(path)-1)