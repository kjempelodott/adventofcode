import itertools
def f(m, p):
    i = a/max(p) + 1
    while i < len(p):
        for g in itertools.combinations(p, i):
            s = sum(g)
            if s < a:
                continue
            if s == a:
                if m == 2: return True
                # First solution will always be min G1 size and QE
                if f(m-1, [x for x in p if x not in g]): 
                    return reduce(lambda x,y: x*y, g, 1) if m==n else True
        i += 1

p = map(int, open('day24_input').read().strip().split('\n'))
n = 3
a = sum(p)/n
print f(n, p)

n = 4
a = sum(p)/n    
print f(n, p)

    
