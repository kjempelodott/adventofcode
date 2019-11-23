import re

def new(p):
    while 1:
        for j in range(9):
            if p[j]!=122:
                p[j]+=1;break
            p[j]=97
        if (not any([(x in p)for x in(105,108,111)])and
            any([(p[i+2]==p[i+1]-1==p[i]-2)for i in range(6)])and
            len(re.findall(r'(.)\1',''.join(map(chr,p))))>1):
            return p

p=map(ord,'hxbxwxba'[::-1])
print''.join(map(chr,new(p))[::-1])
p=map(ord,'hxbxxyzz'[::-1])
print''.join(map(chr,new(p))[::-1])
