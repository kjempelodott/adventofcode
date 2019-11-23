s = open('day19_input','r').read().strip().split('\n')

r = [l.split()[::2] for l in s[:-2]]
m = s[-1]

a = set()
for x, y in r:
    i, l = m.find(x), len(x)
    while i != -1:
        a.add(m[:i] + y + m[i+l:])
        i = m.find(x, i + l)

print len(a)


q = m
r = sorted(r, key=lambda x: -len(x[1]))
i = 0
while q != 'e':
    for x, y in r:
        while y in q:
            q = q.replace(y, x, 1)
            i += 1

print i
