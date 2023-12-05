from aoc.tools import timing
from aoc.parse import InputReader

PARTS = {
    1: 'one',
    2: 'two'
}


class DailyPuzzleBase:
    def __init__(self, day: int):
        self.day = day
        self.input = InputReader.get_input(day)

    def get_input(self):
        return InputReader.get_input(self.day)

    def get_test_input(self, part: int):
        return InputReader.get_test_input(self.day, part)

    def get_test_result(self, part: int):
        return InputReader.get_test_result(self.day, part)

    def test(self, part: int) -> bool:
        test_input = self.get_test_input(part)
        test_result = self.get_test_result(part)
        function_part = getattr(self, f"part_{PARTS[part]}")
        return (function_part(test_input) == test_result, function_part(test_input))

    def test_all(self) -> list[bool]:
        return [self.run_test(part=i) for i in [1, 2]]

    def solve_all(self) -> list[bool]:
        return [self.solve(part=i) for i in [1, 2]]

    def solve(self, part):
        func = getattr(self, f"part_{PARTS[part]}")
        result = func(self.input)
        return result

    @timing
    def part_one(self, input: list[list]) -> int:
        pass

    @timing
    def part_two(self, input: list[list]) -> int:
        pass
