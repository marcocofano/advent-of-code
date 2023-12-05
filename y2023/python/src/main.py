import argparse
import importlib
import datetime
# from utils.submission import Submission
from lib import files


def main():
    _today = datetime.date.today().day

    parser = argparse.ArgumentParser(description="Advent of Code solution runner")
    parser.add_argument("-d", "--day",
                        dest="day",
                        default=_today,
                        metavar="day_number",
                        type=int,
                        help="Required, day number of the AoC event")
    parser.add_argument("-p", "--part",
                        dest="part",
                        default=1,
                        metavar="part_number",
                        type=int, help="Required, part number of the day of the AoC event")
    parser.add_argument("--raw", action="store_true",
                        help="Optional, use raw input instead of stripped input")
    parser.add_argument("--add", action="store_true", help="Optional, create daily file")
    parser.add_argument("--test", action="store_true",
                        help="Optional, skipping tests")
    parser.add_argument("--benchmark", action="store_true",
                        help="Optional, benchmarking the code, and also skipping tests")
    parser.add_argument("--submit", action="store_true",
                        help="Optional, submit your answer to AoC")
    args = parser.parse_args()

    if not 0 < args.day < 26:
        print("day number must be between 1 and 25")
        exit()
    elif args.add is True:
        print("Adding day", args.day)
        files.add_day(args.day)
    elif args.part not in [1, 2]:
        print("part number must be 1 or 2")
        exit()
    else:
        print(f"Solving day {args.day} part {args.part}\n")
        sol = importlib.import_module(f"solutions.d{args.day:02d}").Solution(
            args.day)
        if args.test:
            test_result = sol.test(part=args.part)
            if test_result:
                print(f"Test:\n Passed: {test_result[0]}\n Result: {test_result[1]}\n")
        else:
            print(f"The answer is {answer}\n" if (
                answer := sol.solve(part=args.part)) is not None else "")

        # if answer and args.submit is True:
        #     Submission.send_answer(args.day, args.part, answer)


if __name__ == "__main__":
    main()
