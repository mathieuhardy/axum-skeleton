#!/usr/bin/env python3

import argparse
import json
import subprocess
import sys

from collections import defaultdict

def main():
    metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--format-version", "1"]))
    deps = defaultdict(list)
    duplicates = {}
    has_duplicates = False

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--text", help="Text output instead of JSON", action="store_true")
    parser.add_argument("-r", "--raw", help="Raw output without colors", action="store_true")
    args = parser.parse_args()

    color = "" if args.raw else "\033[1;32m"

    for node in metadata["resolve"]["nodes"]:
        name, version, _ = node["id"].split(" ")
        deps[name].append(version)

    for name, versions in deps.items():
        if len(versions) <= 1:
            continue

        if args.text:
            print("{}{}\033[0m: {}".format(color, name, versions))
            has_duplicates = True
        else:
            duplicates[name] = versions

    if not args.text and len(duplicates) > 0:
        print(json.dumps(duplicates, indent=2))
        has_duplicates = True

    if has_duplicates:
        sys.exit(1)

if __name__ == '__main__':
    main()
