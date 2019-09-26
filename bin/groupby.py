#!/usr/bin/env python
import argparse
import sys
from collections import defaultdict
import re

parser = argparse.ArgumentParser()
parser.add_argument('-f', '--field', type=int, default=0)
parser.add_argument('-d', '--delimiter', default=r'\s+')
parser.add_argument('--humanize', action='store_true')
args = parser.parse_args()

keysize = 0
groups = defaultdict(list)
for line in sys.stdin:  # TODO Find out how to pair argparse with fileinput.input
	fields = re.split(args.delimiter, line.rstrip('\n'))
	key = fields[args.field]
	if len(key) > keysize:
		keysize = len(key)
	groups[key].append(' '.join(f for i, f in enumerate(fields) if i != args.field))

for key, group in groups.items():
	if args.humanize:
		print(f'{key}:')
		for value in sorted(group, key=str.lower):
			print(f' - {value}')
	else:
		for value in sorted(group):
			print(f'{key}\t{value}')
