import fileinput
import re

for line in fileinput.input():
    for word in re.findall('\w+', line):
        print(word)
