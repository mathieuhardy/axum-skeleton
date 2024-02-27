#!/usr/bin/env python3

import re
import sys

if len(sys.argv) != 3:
    print("Missing input and output files")
    sys.exit(0)

input = sys.argv[1]
output = sys.argv[2]

CRATE = 1
THEAD_START = 2
HEADER = 3
THEAD_END = 4
RESULT = 5
TFOOT_START = 6
TOTAL = 7
TFOOT_END = 8
FINISHED = 9

TERRIBLE = 'terrible'
BAD = 'bad'
BAD_2 = 'bad-2'
BAD_3 = 'bad-3'
BAD_4 = 'bad-4'
QUITE_GOOD = 'quite-good'
QUITE_GOOD_2 = 'quite-good-2'
GOOD = 'good'
GOOD_2 = 'good'
VERY_GOOD = 'very-good'

CSS = '''
body {
    background-color: #272727;
    color: #eeeeee;
    font-family: sans-serif;
}

.terrible {
    color: #e74c3c;
    font-weight: bold;
}

.bad {
    color: #d35400;
    font-weight: bold;
}

.bad-2 {
    color: #e67e22;
    font-weight: bold;
}

.bad-3 {
    color: #f39c12;
    font-weight: bold;
}

.bad-4 {
    color: #f1c40f;
    font-weight: bold;
}

.quite-good {
    color: #c39bd3;
    font-weight: bold;
}

.quite-good-2 {
    color: #8e44ad;
    font-weight: bold;
}

.good {
    color: #aed6f1;
    font-weight: bold;
}

.good-2 {
    color: #3498db;
    font-weight: bold;
}

.very-good {
    color: #2ecc71;
    font-weight: bold;
}

h1 {
    margin-left: 1rem;
    font-size: 1.5em;
}

h2 {
    margin-left: 1rem;
    font-size: 1em;
    color: #dddddd;
}

table {
    border-collapse: collapse;
    font-size: 0.9em;
    margin-left: 1rem;
    margin-bottom: 3rem;
}

table thead tr {
    background-color: #009879;
    text-align: left;
}

table tfoot tr {
    background-color: #566573;
    text-align: left;
}

table th,
table td {
    padding: 12px 15px;
}

table thead tr,
table tbody tr,
table tfoot tr {
    border: 1px solid #dddddd;
}
'''

def get_class(entry):
    if entry.find('%') > 0:
        number = float(entry.replace('%', ''))
        if number < 10.0:
            return TERRIBLE
        if number < 20.0:
            return BAD
        if number < 30.0:
            return BAD_2
        if number < 40.0:
            return BAD_3
        if number < 60.0:
            return QUITE_GOOD
        if number < 70.0:
            return QUITE_GOOD_2
        if number < 80.0:
            return GOOD
        if number < 90.0:
            return GOOD_2
        if number <= 100.0:
            return VERY_GOOD

    return ''

def to_html(type, match, file):
    if type == CRATE:
        file.write(f'<h1>{match.group(1)}</h1><h2>{match.group(2)}</h2>')
    elif type == THEAD_START:
        file.write('<table><thead>')
    elif type == HEADER:
        file.write('<tr>')
        for group in match.groups():
            file.write(f'<th>{group.strip()}</th>')
        file.write('</tr>')
    elif type == THEAD_END:
        file.write('</thead><tbody>')
    elif type == RESULT:
        file.write('<tr>')
        for group in match.groups():
            group = group.strip()
            c = get_class(group)
            file.write(f'<td class="{c}">{group}</td>')
        file.write('</tr>')
    elif type == TFOOT_START:
        file.write('</tbody><tfoot>')
    elif type == TOTAL:
        file.write('<tr>')
        for group in match.groups():
            group = group.strip()
            c = get_class(group)
            file.write(f'<th class="{c}">{group}</th>')
        file.write('</tr>')
    elif type == TFOOT_END:
        file.write('</tfoot></table>')
    elif type == FINISHED:
        pass

def next_expectation(type):
    if type == CRATE:
        return [CRATE, THEAD_START]
    elif type == THEAD_START:
        return [HEADER]
    elif type == HEADER:
        return [THEAD_END]
    elif type == THEAD_END or type == RESULT:
        return [RESULT, TFOOT_START]
    elif type == TFOOT_START:
        return [TOTAL]
    elif type == TOTAL:
        return [TFOOT_END]
    elif type == TFOOT_END:
        return [CRATE, FINISHED]
    elif type == FINISHED:
        return []

    print('Internal error: no expectation found')
    sys.exit(1)

with open(output, 'w') as f_out:
    # Write html header
    f_out.write(f'<html><head><title>Documentation coverage</title><style>{CSS}</style></head><body>')

    with open(input, 'r') as f_in:
        # Read all lines of the input file
        lines = f_in.readlines()

        # Prepare regexes
        regexes = {
            CRATE: re.compile('^ Documenting (.*) \((.*)\)'),
            THEAD_START: re.compile('^\+[-+]+\+$'),
            HEADER: re.compile('^\| (.*) +\| +(.*) +\| +(.*) +\| +(.*) +\| +(.*) +\|$'),
            THEAD_END: re.compile('^\+[-+]+\+$'),
            RESULT: re.compile('^\| (.*) +\| +(.*) +\| +(.*) +\| +(.*) +\| +(.*) +\|$'),
            TFOOT_START: re.compile('^\+[-+]+\+$'),
            TOTAL: re.compile('^\| (.*) +\| +(.*) +\| +(.*) +\| +(.*) +\| +(.*) +\|$'),
            TFOOT_END: re.compile('^\+[-+]+\+$'),
            FINISHED: re.compile('^ +Finished.*$'),
        }

        # First pattern expected is a crate module
        expect = [CRATE]

        # Parse all lines
        for line in lines:
            found = False

            # Search for any of our expectations
            for expectation in expect:
                m = regexes[expectation].match(line)
                if not m:
                    continue

                # Convert and write to output
                to_html(expectation, m, f_out)

                # Get next epxectations
                expect = next_expectation(expectation)

                found = True
                break

            # if not found:
                # print('Parsing error: expectation has not been found')

    # Write html footer
    f_out.write('</body></html>')

