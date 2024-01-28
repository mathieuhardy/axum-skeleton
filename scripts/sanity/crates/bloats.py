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
    output = subprocess.check_output(["cargo", "bloat"]).decode('utf-8')

    if args.text:
        print(output)
        return

    # Convert to JSON
    results = []
    pattern = "^([^ ]+) *([^ ]+) *([^ ]+) *([^ ]+) *(.*)$"
    rows = output.splitlines()

    for row in range(2, len(rows)):
        try:
            (file, text, size, crate, name) = re.findall(pattern, rows[row])[0]

            results.append({
                "file": file,
                "text": text,
                "size": size,
                "crate": crate,
                "name": name
            })
        except:
            pass

    print(json.dumps(results, indent=2))

if __name__ == '__main__':
    main()
