from src.d02.solution import part_one, part_two, read_input


SOLUTION_1 = 8
SOLUTION_2 = 2286

filepath_1 = '../inputs/test_d02.txt'

def test_part_1():
    assert SOLUTION_1 == part_one(filepath_1)


def test_part_2():
    assert SOLUTION_2 == part_two(filepath_1)
