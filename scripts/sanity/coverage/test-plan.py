#!/usr/bin/env python3

import argparse
import json
import re
import subprocess

import pprint

from collections import defaultdict

def find_matches(pattern, directory):
    try:
        output = subprocess.check_output(["rg", "--smart-case", "--no-line-number", "--no-filename", pattern, directory])
    except:
        try:
            output = subprocess.check_output(["grep", "-r", "--no-filename", pattern, directory])
        except:
            return []

    return output.decode("utf-8").split("\n")

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-p", "--plans", help="Directory where to find the test plans", required = True)
    parser.add_argument("-s", "--sources", help="Directory where to find the source code", required = True)
    args = parser.parse_args()

    total = 0
    found = 0

    results = {
        "plans": {},
        "percentage": 0.0
    }

    # TODO: Use argparse for input directory
    plans = find_matches("/TC/", args.plans)

    for plan in plans:
        matches = re.findall("^> (/TC/.*)$", plan)
        if len(matches) == 0:
            continue

        id = matches[0]
        is_found = len(find_matches(id, args.sources)) > 0

        results["plans"][id] = is_found

        total += 1
        found += 1 if is_found else 0

    results["percentage"] = (found / total) * 100

    print(json.dumps(results, indent=2))

if __name__ == '__main__':
    main()
