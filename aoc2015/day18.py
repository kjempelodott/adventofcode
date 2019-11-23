l1 = [[1 if c=='#' else 0 for c in line] for line in
      open('day18_input','r').read().strip().split('\n')]

r = range(100)
s = [0] + r + [100,100]


l = [k[:] for k in l1]
ln = [k[:] for k in l]


for _ in r:
    for i in r:
        for j in r:
            n = sum(sum(k[s[j]:s[j+3]]) for k in l[s[i]:s[i+3]])
            if l[i][j]:
                if n not in (3,4):
                    ln[i][j] = 0
            elif n == 3:
                ln[i][j] = 1
    l = ln
    ln = [k[:] for k in l]
print sum(map(sum, l))



l = [k[:] for k in l1]
l[0][0] = l[0][99] = l[99][0] = l[99][99] = 1
ln = [k[:] for k in l]

for _ in r:
    for i in r:
        for j in r:
            if i in (0,99) and j in (0,99):
                continue
            n = sum(sum(k[s[j]:s[j+3]]) for k in l[s[i]:s[i+3]])
            if l[i][j]:
                if n not in (3,4):
                    ln[i][j] = 0
            elif n == 3:
                ln[i][j] = 1
    l = ln
    ln = [k[:] for k in l]
print sum(map(sum, l))
