from pathlib import Path


INPUTS_PATH = Path('../../inputs')


def open_file(filepath):
    try:
        with open(filepath, 'r') as file:
            return [line.strip() for line in file]
    except FileNotFoundError:
        print(f'File {filepath} not present')
        return []


class InputReader:
    @staticmethod
    def get_input(day):
        filepath = INPUTS_PATH / f"d{day:02}.txt"
        return open_file(filepath)

    def get_test_input(day, part):
        part_path = f"_{part}" if part else ""
        filepath = INPUTS_PATH / f"test_d{day:02}{part_path}.txt"
        return open_file(filepath)

    def get_test_result(day, part):
        filepath = INPUTS_PATH / "test_results.csv"
        results = open_file(filepath)
        for line in results:
            result_line = line.split(', ')
            if result_line[0] == f'{day:02}':
                return int(result_line[part])
