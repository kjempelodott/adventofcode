from itertools import permutations as p

x={}
for line in open('day13_input', 'r').read().strip().split('\n'):
    a,_,s,z,_,_,_,_,_,_,b=line[:-1].split()
    x[a]=x.get(a,{})
    x[a][b]=int(z)*(-1,1)[s=='gain']

y=lambda x:max(sum(x[a][b]+x[b][a]for a,b in zip(q,list(q[1:])+[q[0]]))for q in p(x))

print(y(x))

x[0]={}
for k in x:x[0][k]=x[k][0]=0

print(y(x))


