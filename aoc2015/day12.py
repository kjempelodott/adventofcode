t = open('day12_input','r').read()

import re
print sum(map(int,re.findall('-?\d+',t)))



import json,operator
r = lambda x: (type(x)==list and reduce(operator.add,map(r,x)))or \
              (type(x)==dict and not'red'in x.values()and 
               reduce(operator.add,map(r,x.values())))or \
              (type(x)==int and x)or 0              
r(json.loads(t))

# OR

sum(map(int,re.findall('-?\d+',str(json.loads(t,None,None,lambda o:type(o)==dict
                                              and'red'in o.values()or o)))))
