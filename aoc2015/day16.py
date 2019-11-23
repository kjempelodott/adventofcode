lines = open('day16_input','r').read().strip().split('\n')

s = {'children': 3,
     'cats': 7,
     'samoyeds': 2,
     'pomeranians': 3,
     'akitas': 0,
     'vizslas': 0,
     'goldfish': 5,
     'trees': 3,
     'cars': 2,
     'perfumes': 1}

data = []
for l in lines:
    l = l.split()
    data.append(dict((k.strip(':'),int(v.strip(',')))for k,v in zip(l[2::2], l[3::2])))

for i,x in enumerate(data):
    if all(s[k]==x[k] for k in x):
        print x,i
        break

y = ['trees', 'cats']
z = ['pomeranians', 'goldfish']
for i,x in enumerate(data):
    if all(((1,0)[k in y+z],
            (0,1)[k in z],
            (0,1)[k in y])[cmp(s[k],x[k])]
           for k in x):
        print x,i+1
        break
