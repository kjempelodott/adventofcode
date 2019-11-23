from itertools import permutations

x = {}
for line in open('day9_input', 'r').read().strip().split('\n'):
    f,_,t,_,d=line.split()
    x[f],x[t]=x.get(f,{}),x.get(t,{})
    x[t][f]=x[f][t]=int(d)

r=sorted(sum(x[f][t]for f,t in zip(p, p[1:]))for p in permutations(set(x)))
print r[0], r[-1]

