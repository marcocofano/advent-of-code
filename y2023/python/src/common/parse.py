def read_input(filename: str):
    with open(filename, 'r') as file:
        for line in file:
            yield line.strip()


def read_complete_input(filename: str):
    with open(filename, 'r') as file:
        return [line.strip() for line in file]
