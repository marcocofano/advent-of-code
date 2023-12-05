from aoc.tools import timing
from aoc.solution import DailyPuzzleBase


def parse_scratchcard(line):
    game, scratchcard = line.split(': ')
    winning, numbers = scratchcard.split('|')
    winning = winning.strip().split(' ')
    numbers = numbers.strip().split(' ')
    inters = set(numbers).intersection(set(winning))
    inters.discard("")
    return len(inters)


def first_score_rule(function):
    def wraps(*args, **kwargs):
        return int(2**(function(*args, **kwargs) - 1))
    return wraps


class Solution(DailyPuzzleBase):
    def __init__(self, day: int):
        super().__init__(day)

    def part_one(self, input) -> int:
        return sum(map(first_score_rule(parse_scratchcard), input))

    def part_two(self, input) -> int:
        scratchcards = [0]*len(input)
        for r, line in enumerate(input):
            scratchcards[r] += 1
            score = parse_scratchcard(line)
            for i in range(score):
                if r + 1 + i < len(input):
                    scratchcards[r + 1 + i] += scratchcards[r]
        return sum(scratchcards)
