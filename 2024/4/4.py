

@staticmethod
def create_grid(letters: list, ncol: int, nrow: int):
    assert len(letters) == ncol * nrow, "Invalid number of letters."
    grid = []
    for i in range(nrow):
        grid.append(letters[i * ncol : (i + 1) * ncol])
    return grid


def main():
    with open(
        "C:/Users/Juuso Paakkunainen/Documents/projects/advent_of_code/4/input.txt", "r"
    ) as f:
        input = f.read().strip()
        nrow = len(input.split("\n"))
        ncol = len(input.split("\n")[0])
        grid = create_grid(input.replace("\n", ""), ncol, nrow)
        print(grid)


if __name__ == "__main__":
    print(main())
