import re

l=lambda s:''.join(str(len(x[0]))+x[1]for x in re.findall(r'((.)\2*)',s))
                   
s='1113222113'
for i in range(40):s=l(s)
print len(s)

s='1113222113'
for i in range(50):s=l(s)
print len(s)
