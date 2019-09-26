#!/usr/bin/env python
import argparse
import sys
from collections import defaultdict
import re


def main(field, delimiter):
    keysize = 0
    groups = defaultdict(list)
    # TODO Find out how to pair argparse with fileinput.input
    for line in sys.stdin:
        fields = re.split(delimiter, line.rstrip('\n'))
        key = fields[field]
        if len(key) > keysize:
            keysize = len(key)
        fields = (f for i, f in enumerate(fields) if i != field)
        groups[key].append(' '.join(fields))

    for key in sorted(groups, key=str.lower):
        if args.humanize:
            print(f'{key}:')
            for value in sorted(groups[key], key=str.lower):
                print(f' - {value}')
        else:
            for value in sorted(groups[key]):
                print(f'{key}\t{value}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('-f', '--field', type=int, default=1)
    parser.add_argument('-d', '--delimiter', default=r'\s+')
    parser.add_argument('--humanize', action='store_true')
    args = parser.parse_args()

    main(args.field - 1, args.delimiter)
