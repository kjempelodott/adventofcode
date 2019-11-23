import re
from numpy import zeros, int16

p = re.compile('(.+)\s(\d+),(\d+)\s.+\s(\d+),(\d+)')
instr = [(lambda t: [t[0].strip()]+map(int, t[1:]))(p.match(i).groups())
         for i in open('day6_input', 'r').read().strip().split('\n')]

l = zeros((1000,1000), dtype=bool)
print [(lambda i,x0,y0,x1,y1: 
        (i == 'turn on'  and l[x0:x1+1,y0:y1+1].fill(True)) or
        (i == 'turn off' and l[x0:x1+1,y0:y1+1].fill(False)) or
        (i == 'toggle'   and l[x0:x1+1,y0:y1+1].__ixor__(1)))(*i) 
       for i in instr] and l.sum()

add = {'turn on' :  1,
       'turn off': -1,
       'toggle'  :  2}

l = zeros((1000,1000), dtype=int16)
for i in instr:
    x0,y0,x1,y1=i[1:]
    s = l[x0:x1+1,y0:y1+1]
    s += add[i[0]]
    s[s < 0] = 0

print l.sum()
