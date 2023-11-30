# from pathlib import Path
# from typing import list

def read_next_calories(filepath):
    with open(filepath, 'r') as file:
        group = []
        for line in file:
            line = line.strip()
            if line == '':
                if group:  # Yield only if the group is not empty
                    yield group
                    group = []  # Reset the group for the next set of numbers
            else:
                group.append(int(line))

        # Yield the last group if the file doesn't end with a newline
        if group:
            yield group


def part1(filepath: str) -> int:
    inputs = [group for group in read_next_calories(filepath)]
    calories = list(map(sum, inputs))
    return max(calories)


def part2(filepath: str) -> int:
    inputs = [group for group in read_next_calories(filepath)]
    calories = list(map(sum, inputs))
    calories.sort()
    return sum(calories[-3:])


def main():
    filepath = '../../inputs/d01.txt'
    print(f"Part 1 result: {part1(filepath)}")
    print(f"Part 2 result: {part2(filepath)}")


if __name__ == "__main__":
    main()
