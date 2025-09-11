def main():
    num_safe = 0
    with open(
        "C:/Users/Juuso Paakkunainen/Documents/projects/advent_of_code/2/input.txt", "r"
    ) as f:
        for line in f:
            report = [int(x) for x in line.strip().split()]
            diffs = [report[i + 1] - report[i] for i in range(len(report) - 1)]
            increasing = all(1 <= diff <= 3 for diff in diffs)
            decreasing = all(-3 <= diff <= -1 for diff in diffs)
            if increasing or decreasing:
                num_safe += 1
    return num_safe


if __name__ == "__main__":
    print(main())
