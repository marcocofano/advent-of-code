
from solution import part1, part2


SOLUTION_1 = 24000
SOLUTION_2 = 45000

filepath = '../../inputs/test_d01.txt'


def test_part_1():
    assert SOLUTION_1 == part1(filepath)


def test_part_2():
    assert SOLUTION_2 == part2(filepath)
