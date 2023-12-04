import re
from common.tools import timing
from common.parse import read_input


NUMBERS = [
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9'
]
NUMBERS_WORDS = [
    'one',
    'two',
    'three',
    'four',
    'five',
    'six',
    'seven',
    'eight',
    'nine'
]

PATTERN = f"(\\d|{'|'.join(NUMBERS_WORDS)})"
PATTERN_REV = f"(\\d|{'|'.join([nw[::-1] for nw in NUMBERS_WORDS])})"
MATCHES_DICT = {key: value for key, value in zip(NUMBERS_WORDS, NUMBERS)}


def find_integer(string):
    for c in string:
        if c.isdigit():
            return int(c)


def parse_calibration_line(line):
    # numbers = [c for c in line if c in NUMBERS]
    # find first integer
    first = find_integer(line)
    last = find_integer(line[::-1])
    return first*10 + last


def parse_calibration_line_improved(line):
    start_match = re.search(PATTERN, line).group(1)
    end_match = re.search(PATTERN_REV, line[::-1]).group(1)
    end_match = end_match[::-1]
    first = int(MATCHES_DICT.get(start_match, start_match))
    last = int(MATCHES_DICT.get(end_match, end_match))
    return first*10 + last


@timing
def part_one(filepath: str) -> int:
    return sum([parse_calibration_line(line)
                for line in read_input(filepath=filepath)])


@timing
def part_two(filepath: str) -> int:
    return sum([parse_calibration_line_improved(line)
                for line in read_input(filepath=filepath)])
