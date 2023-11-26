with open("input", "r") as f:
    line = f.read()

    n = 14
    for i in range(len(line)):
        if len(set(line[i:i+n])) == n:
            print(i+n)
            break