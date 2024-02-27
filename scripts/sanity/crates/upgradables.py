#!/usr/bin/env python3

import argparse
import json
import re
import subprocess

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--text", help="Text output instead of JSON", action="store_true")
    args = parser.parse_args()

    # Fetch results
    output = subprocess.check_output(["cargo", "outdated"]).decode('utf-8')

    if args.text:
        print(output)
        return

    # Convert to JSON
    results = []
    pattern = "^([^ ]+) *([^ ]+) *([^ ]+) *([^ ]+) *([^ ]+) *(.*)$"
    rows = output.splitlines()

    for row in range(2, len(rows)):
        (name, project, compat, latest, kind, platform) = re.findall(pattern, rows[row])[0]

        results.append({
            "name": name,
            "project": project,
            "compat": compat,
            "latest": latest,
            "kind": kind,
            "platform": platform
        })

    print(json.dumps(results, indent=2))

if __name__ == '__main__':
    main()
