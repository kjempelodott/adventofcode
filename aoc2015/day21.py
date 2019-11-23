w = [map(int, x.split()[1:3]) for x in
"""
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0
""".strip().split('\n')[1:]]

a = [map(int, x.split()[1::2]) for x in
"""
Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5
""".strip().split('\n')[1:]]

r = [map(int, x.split()[2:]) for x in
"""
Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3
""".strip().split('\n')[1:]]


import itertools
i,j,k=100,8,2
m,n,e=[],[],(0,0,0)
for x in w:
    for y in a+[e]:
        for z,q in[[e,e]]+list(itertools.combinations(r+[e],2)):
            (n,m)[i/max(1,x[1]+q[1]+z[1]-k)<=1+100/max(1,j-y[1]-z[2]-q[2])].append(x[0]+y[0]+z[0]+q[0])
print min(m),max(n)
