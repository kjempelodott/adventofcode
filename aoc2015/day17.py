import itertools

c = sorted(map(int, open('day17_input').read().strip().split('\n')))

l = len(c)
M = [sum(c[:i]) >= 150 for i in xrange(1, l)].index(True)
m = l - [sum(c[-i:]) <= 150 for i in xrange(l, 1, -1)].index(True) + 1

j = 0
for n in xrange(m,M):
    i = sum(sum(y)==150 for y in itertools.combinations(c, n))
    j += i
    print n,i
print j
