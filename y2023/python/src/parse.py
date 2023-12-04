def parse_input(filename: str):
    with open(filename, 'r') as file:
        for line in file:
            yield line.strip()
