#!/usr/bin/env python
import argparse
import sys
from collections import defaultdict
import re


def main(field, delimiter, unique):
    keysize = 0
    groups = defaultdict(list)
    # TODO Find out how to pair argparse with fileinput.input
    for line in sys.stdin:
        fields = re.split(delimiter, line.rstrip('\n'))
        key = fields[field]
        if len(key) > keysize:
            keysize = len(key)
        fields = (f for i, f in enumerate(fields) if i != field)
        groups[key].extend(fields)

    for key in sorted(groups, key=str.lower):
        if args.humanize:
            print(f'{key}:', end=' ' if args.oneline else '\n')
            if unique:
                groups[key] = set(groups[key])

            if args.oneline:
                print(', '.join(groups[key]))
            else:
                for value in sorted(groups[key], key=str.lower):
                    print(f' - {value}')
        else:
            if unique:
                groups[key] = set(groups[key])
            for value in sorted(groups[key]):
                print(f'{key}\t{value}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('-f', '--field', type=int, default=1)
    parser.add_argument('-d', '--delimiter', default=r'\s+')
    parser.add_argument('-u', '--unique', action='store_true', default=False)
    parser.add_argument('--humanize', action='store_true')
    parser.add_argument('--oneline', action='store_true')
    args = parser.parse_args()

    main(args.field - 1, args.delimiter, args.unique)
