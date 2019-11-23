import re

data = open('day14_input').read().strip().split('\n')
T = 2503

# Part 1
print max((lambda v,t,z:v*(t*(T/(t+z))+min(t,T%(t+z))))(*map(int,re.findall('\d+',l)))for l in data)


# Part 2
# speed, duration, recovery, distance, clock, points
rd = [map(int, re.findall('\d+', line)) + [0]*3 for line in data]
for _ in xrange(T):
    for r in rd:
        if r[4] < 0:
            r[4] += 1
        else:
            r[3] += r[0]
            r[4] += (r[1] - 1, -1)[r[4]>0]
            r[4] = r[4] or -r[2]
    md = max(r[3] for r in rd)
    for r in rd:
        r[5] += (0,1)[r[3] == md]
print max(r[5] for r in rd)
