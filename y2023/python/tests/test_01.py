from src.d01.solution import part_one, part_two, read_input


SOLUTION_1 = 142
SOLUTION_2 = 281

filepath_1 = '../inputs/test_d01.txt'
inputs_1 = read_input(filepath_1)

filepath_2 = '../inputs/test_d01_2.txt'
inputs_2 = read_input(filepath_2)


def test_part_1():
    assert SOLUTION_1 == part_one(inputs_1)


def test_part_2():
    assert SOLUTION_2 == part_two(inputs_2)
