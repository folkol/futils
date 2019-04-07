import calendar
from collections import defaultdict
from datetime import datetime
from itertools import cycle, islice

import matplotlib.pyplot as plt
import numpy as np

fig = plt.figure(figsize=(20, 5))
ax = fig.add_subplot(1, 1, 1)

major_ticks = np.arange(0, 24 * 7, 24)
minor_ticks = np.arange(0, 24 * 7, 6)

ax.set_xticks(major_ticks)
ax.set_xticks(minor_ticks, minor=True)
ax.set_xticklabels([])
ax.set_xticklabels(islice(cycle([0, 6, 12, 18]), 4 * 7), minor=True, alpha=0.5)
ax.set_yticklabels([])
ax.set_yticks([])
ax.set_axisbelow(True)

# f = sys.stdin
f = open('timestamps.log')

events = defaultdict(lambda: defaultdict(int))
for datetime in (datetime.utcfromtimestamp(int(line.strip())) for line in f):
    events[datetime.weekday()][datetime.hour] += 1

xs = range(24 * 7)
ys = []
for i, day in enumerate(calendar.day_name):
    plt.text(1 / 14 + i * 1 / 7,
             1,
             day,
             alpha=0.4,
             horizontalalignment='center',
             verticalalignment='bottom',
             transform=ax.transAxes)
    ys.extend(events[i][x] for x in range(24))

ax.grid(which='minor', alpha=0.2, axis='x')
ax.grid(which='major', alpha=1, axis='x')
plt.bar(xs, ys, align='edge', width=1, linewidth=1, alpha=0.5, edgecolor='black', facecolor='gray')
plt.margins(0, 0)
plt.savefig('schedule.png')
