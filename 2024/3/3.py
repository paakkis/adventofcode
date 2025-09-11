import re


def main():
    total = 0
    with open(
        "C:/Users/Juuso Paakkunainen/Documents/projects/advent_of_code/3/input.txt", "r"
    ) as f:
        corrupted = f.read().strip()
        muls = re.findall(r"mul\((\d+),(\d+)\)", corrupted)
        for mul in muls:
            total += int(mul[0]) * int(mul[1])
    return total


if __name__ == "__main__":
    print(main())
