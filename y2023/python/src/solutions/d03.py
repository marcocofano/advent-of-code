from aoc.solution import DailyPuzzleBase
from aoc.tools import timing
import re
import math
import string
from collections import defaultdict

SYMBOLS = set(string.punctuation) - set(".")


def check(start_pos, end_pos, row):
    return any(item in SYMBOLS for item in row[start_pos:end_pos + 1])


def check_star(start_pos, end_pos, row):
    return [
        start_pos + i for i, item in enumerate(row[start_pos:end_pos + 1]) if item == '*'
    ]


class Solution(DailyPuzzleBase):
    def __init__(self, day):
        super().__init__(day)

    @timing
    def part_one(self, input) -> int:
        tot_rows = len(input)
        tot_cols = len(input[0])
        result = 0
        for r, row in enumerate(input):
            row_str = ''.join(row)
            numbers = re.finditer(r"\d+", row_str)
            for number in numbers:
                col_start = max(0, number.start() - 1)
                col_end = min(number.end(), tot_cols)
                for check_row in [-1, 0, 1]:
                    if 0 < (r + check_row) < tot_rows:
                        if check(
                            start_pos=col_start,
                            end_pos=col_end,
                            row=input[r + check_row]
                        ):
                            result += int(number.group())
        return result

    @timing
    def part_two(self, input) -> int:
        gears = defaultdict(list)
        tot_rows = len(input)
        tot_cols = len(input[0])
        for r, row in enumerate(input):
            row_str = ''.join(row)
            numbers = re.finditer(r"\d+", row_str)
            for number in numbers:
                col_start = max(0, number.start() - 1)
                col_end = min(number.end(), tot_cols)
                for check_row in [-1, 0, 1]:
                    if 0 < (r + check_row) < tot_rows:
                        star_cols = check_star(
                            start_pos=col_start,
                            end_pos=col_end,
                            row=input[check_row + r]
                        )
                        if star_cols:
                            for star_col in star_cols:
                                gears[(check_row + r, star_col)].append(
                                    int(number.group()))
        return sum([math.prod(value) for _, value in gears.items() if len(value) == 2])
