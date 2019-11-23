import itertools

def d(n):
    f = []
    if n >= 2:
        i = 2
        while i * i <= n:
            while n % i == 0:
                f.append(i)
                n /= i
            i += 1
        if n > 1:
            f.append(n)

    for m in itertools.product(*[[x**y for y in range(f.count(x)+1)] for x in set(f)]):
        yield reduce(lambda x,y: x*y, m, 1)
    

h = s = 0
while s < 34000000:
    h += 1
    s = 0
    for f in d(h):
        if h % f == 0:
            s += 10*f
print h,s



h = s = 0
while s < 34000000:
    h += 1
    s = 0
    for i in xrange(1,50):
        f,m = divmod(h,i)
        if m == 0:
            s += 11*f
print h,s
