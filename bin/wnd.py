from argparse import ArgumentParser
from collections import deque
import sys
from itertools import islice

parser = ArgumentParser(description='Sliding Window of lines from stdin.')
parser.add_argument('-s', '--separator', help='Output separator', default='\t', action='store')
parser.add_argument('-n', '--window-size', help='Window size', default=2, type=int)
parser.add_argument('--accept-truncated', help='Accept truncated windows', default=False, action='store_true')

args = parser.parse_args()
window_size = args.window_size
separator = args.separator
accept_truncated = args.accept_truncated

window = deque(maxlen=window_size)
window.extend(line.rstrip('\n') for line in islice(sys.stdin, window_size-1))

too_few = True
for line in sys.stdin:
    too_few = False
    window.append(line.rstrip('\n'))
    print(*window, sep=separator)

if too_few:
    if accept_truncated:
        print(*window, sep=separator)
    else:
        print("Not enough lines to fill first window, use --accept-truncated if you are ok with this.", file=sys.stderr)
        sys.exit(1)
