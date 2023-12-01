from d01.solution import part_one, part_two, read_input


def main():
    filepath = '../../inputs/d01.txt'
    inputs = read_input(filepath)
    print(f"Part 1 result: {part_one(inputs)}")
    print(f"Part 2 result: {part_two(inputs)}") # 55291


if __name__ == "__main__":
    main()
