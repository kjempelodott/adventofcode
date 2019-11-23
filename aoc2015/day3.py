santa = open('day3_input', 'r').read().strip()

from numpy import zeros

def dir(c,x,y):
    return (c == '^' and (x,y-1)) or (c == 'v' and (x,y+1)) or \
           (c == '<' and (x-1,y)) or (x+1,y)

x=y=2**10
h = zeros([2**11]*2)
h[x,y] += 1
for c in santa:
    x,y = dir(c,x,y)
    h[x,y] += 1
print (h >= 1).sum()

xs=ys=xr=yr=2**10
h = zeros([2**11]*2)
h[xs,ys] += 1
for cs,cr in zip(santa[::2],santa[1::2]):
    xs,ys = dir(cs,xs,ys)
    xr,yr = dir(cr,xr,yr)
    h[xs,ys] += 1
    h[xr,yr] += 1
print (h >= 1).sum()
