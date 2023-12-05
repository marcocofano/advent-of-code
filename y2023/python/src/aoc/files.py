import urllib.request
import urllib.parse
import os
import shutil
# import sys
# import json
import datetime
from time import sleep
from pathlib import Path


BASEPATH = "."
INPUTS_FOLDER = "../../inputs"


def download_puzzle_input(year, day):
    headers = {}
    headers["Referer"] = f"https://adventofcode.com/{year}/day/{day}"
    headers["Cookie"] = f"session={get_session()}"

    url = f"https://adventofcode.com/{year}/day/{day}/input"
    method = "GET"

    req = urllib.request.Request(url, method=method, headers=headers)

    with urllib.request.urlopen(req) as response:
        content = response.read().decode("utf-8")

    return content


def add_day(day):
    solution = os.path.realpath(f"{BASEPATH}/solutions/d{day:02}.py")
    solution_path = Path(solution)
    if not solution_path.exists():
        template_path = Path(f"{BASEPATH}/solutions/d00.py")
        solution_path = Path(f"{BASEPATH}/solutions/d{day:02}.py")
        shutil.copy(template_path, solution_path)
        print(f"Creating python solution file from template: {solution_path}")
    input_folder = Path(INPUTS_FOLDER)
    if not input_folder.exists():
        input_folder.mkdir(parents=True, exist_ok=True)

    files = [
        f"d{day:02}.txt",
        f"test_d{day:02}_1.txt",
        f"test_d{day:02}_2.txt",
        f"test_results.csv"
    ]
    for file in files:
        file_path = Path(f"{input_folder}/{file}")
        if not file_path.exists():
            file_path.touch()
            print("Created file:", file_path)

    input_path = Path(f"{input_folder}/{files[0]}")
    if input_path.stat().st_size == 0:
        now = datetime.datetime.utcnow()
        available_to_download = datetime.datetime(
            2023, 12, day, 5, 0, 0)
        if now < available_to_download:
            print("Puzzle input not available to download until",
                  available_to_download.strftime("%Y-%m-%d %H:%M:%S"), "UTC\n")
        while now < available_to_download:
            print("\033[Fnow:", now.strftime("%Y-%m-%d %H:%M:%S.%f")[:-3], "UTC")
            sleep(1)
            now = datetime.datetime.utcnow()

        print("Downloading puzzle input...")
        with open(input_path, "w+") as f:
            f.write(download_puzzle_input(2023, day))
            print("Downloaded puzzle input to:", input_path)

def get_session():
    session = ""
    session_path = Path(INPUTS_FOLDER) / ".session.txt"
    with open(session_path, "r") as f:
        session = f.read().strip()
    return session

# def get_headers():
#     headers = {}
#     path = get_path()
#     headers_config_path = os.path.realpath(f"{path}/../aoc_headers.json")
#     with open(headers_config_path, "r") as f:
#         headers = json.loads(f.read().strip())
#     return headers
#

# def get_path():
#     return path if os.path.isdir(path := os.path.realpath(sys.argv[0])) else os.path.dirname(path)
