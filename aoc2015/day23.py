r={'a':1, 'b':0}
def hlf(x):r[x]/=2
def tpl(x):r[x]*=3
def inc(x):r[x]+=1
def jmp(j):return int(j)
def jie(x,j):return 0 if r[x]%2 else int(j)
def jio(x,j):return 0 if r[x]!=1 else int(j)
f=locals()

instr = [l.replace(',', '').split() for l in open('day23_input').read().strip().split('\n')]
j=0
while j < len(instr):
    i=instr[j]
    y=f[i[0]](*i[1:])
    j+=y if y else 1
print r
