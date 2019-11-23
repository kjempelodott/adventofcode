elevator = open('day1_input', 'r').read().strip()
N = len(elevator)
print elevator.count('(') - elevator.count(')')

elevator = list(reversed(map(lambda c: 40 - ord(c) or 1, elevator)))
floor = 0
while floor != -1:
    floor += elevator.pop()
    print N - len(elevator)
