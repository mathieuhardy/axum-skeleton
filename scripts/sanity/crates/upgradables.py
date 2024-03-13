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
    output = subprocess.check_output(["cargo", "outdated", "--workspace", "--root-deps-only"]).decode('utf-8')

    if args.text:
        print(output)
        return

    # Convert to JSON
    results = []
    crate = ''
    re_crate = "^([^ ]+)$"
    re_scope_sep = "^(=+)$"
    re_header = "^Name.*$"
    re_header_sep = "^([- ]+)$"
    re_entry = "^([^ ]+) *([^ ]+) *([^ ]+) *([^ ]+) *([^ ]+) *(.*)$"
    rows = output.splitlines()

    for idx in range(0, len(rows)):
        row = rows[idx]

        if re.match(re_scope_sep, row) or re.match(re_header_sep, row):
            continue
        if re.match(re_header, row):
            continue
        elif re.match(re_crate, row):
            crate = row
        elif re.match(re_entry, row):
            (name, project, compat, latest, kind, platform) = re.findall(re_entry, row)[0]

            results.append({
                "crate": crate,
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
