import fileinput
import re

for line in fileinput.input():
    for word in re.findall(r'(?:\w|\.(?=\w))+', line):
        print(word)
