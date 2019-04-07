import sys
from datetime import datetime

FMT = '[%d/%b/%Y:%H:%M:%S %z]'

for line in sys.stdin:
    print(datetime.strptime(line.strip(), FMT).timestamp())
