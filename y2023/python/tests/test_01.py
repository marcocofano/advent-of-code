from src.d01.solution import part_one, part_two, read_input


SOLUTION_1 = 142
SOLUTION_2 = 281

filepath_1 = '../inputs/test_d01.txt'

filepath_2 = '../inputs/test_d01_2.txt'


def test_part_1():
    assert SOLUTION_1 == part_one(filepath_1)


def test_part_2():
    assert SOLUTION_2 == part_two(filepath_2)
