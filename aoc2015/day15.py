import re

data = open('day15_input', 'r').read().strip().split('\n')

z = zip(*[map(int, re.findall('-?\d+', line)) for line in data])
m, q = 100, len(z) - 1
x = xrange(1, m-2)

def i(n, *args):
    j = sum(args)
    for y in x:
        if y + j > m:
            raise StopIteration
        if n > 2:
            for z in i(n-1, y, *args):
                yield [y] + z
        else:
            yield [y, m - j - y]

print max(reduce(lambda a,b:a*b,[max(0,a)for a in[sum(a*b for a,b in zip(c, y))for y in z[:-1]]])
          for c in i(q))

print max(reduce(lambda a,b:a*b,[max(0,a)for a in[sum(a*b for a,b in zip(c, y))for y in z[:-1]]
                                 if sum(a*b for a,b in zip(c,z[-1]))==500],1)for c in i(q))
