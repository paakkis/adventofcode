def main():
    left = []
    right = []
    with open(
        "C:/Users/Juuso Paakkunainen/Documents/projects/advent_of_code/1/input.txt", "r"
    ) as f:
        for line in f:
            parts = line.strip().split()
            left.append(parts[0])
            right.append(parts[1])
    left.sort()
    right.sort()
    tot_dist = 0
    for i in range(len(left)):
        dist = abs(int(left[i]) - int(right[i]))
        tot_dist += dist
    return tot_dist


if __name__ == "__main__":
    print(main())
