from common.tools import timing
import math

VALIDATION = {
    'red': 12,
    'green': 13,
    'blue': 14
}


def read_input(filepath: str) -> list:
    with open(filepath, 'r') as file:
        for line in file:
            yield line.strip()


def parse_line(line):
    game_number, rounds = line.split(': ')
    rounds = rounds.replace(';', ',').split(', ')
    for round in rounds:
        round_parsed = round.split(' ')
        if VALIDATION[round_parsed[1]] < int(round_parsed[0]):
            return 0
    return int(game_number.split(' ')[1])


def parse_line_2(line):
    colours_min = [0, 0, 0]
    rounds = line.split(': ')[1].replace(';', ',').split(', ')
    for round in rounds:
        round = round.split(' ')
        c = round[1]
        n = int(round[0])
        match c:
            case 'red':
                colours_min[0] = max(colours_min[0], n)
            case 'green':
                colours_min[1] = max(colours_min[1], n)
            case 'blue':
                colours_min[2] = max(colours_min[2], n)
    return math.prod(colours_min)


@timing
def part_one(filepath: str) -> int:
    return sum([parse_line(line) for line in read_input(filepath)])


@timing
def part_two(filepath: str) -> int:
    return sum([parse_line_2(line) for line in read_input(filepath)])
