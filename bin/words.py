import fileinput

for line in fileinput.input():
    for word in line.split():
        print(word)

