import re

instructions = open('day7_input', 'r').read().strip().split('\n')

no_op = re.compile('(.+)\s->\s(.+)')
op = re.compile('((.+)\s)?(.+)\s(.+)\s->\s(.+)')

om = {'AND'    : lambda x,y: x & y,
      'OR'     : lambda x,y: x | y,
      'LSHIFT' : lambda x,y: x << y,
      'RSHIFT' : lambda x,y: x >> y,
      'NOT'    : lambda x,y: ~y + 2**16}

class circ(dict):
    
    class wire:
        def __init__(self, x, o=None, y=None):
            self.x, self.o, self.y = x, o, y
            self.val = 0
        def __call__(self, c):
            x, o, y = self.x, self.o, self.y
            if not self.val:
                self.val = c[x] if not o else o(c[x], c[y])
            return self.val

    def __getitem__(self, i):
        if isinstance(i, int):
            return i
        w = dict.__getitem__(self, i)
        return w(self)

    def reset(self):
        for w in self.itervalues():
            w.val = 0

c = circ()

for instr in instructions:
    try:
        x,o,y,z = op.match(instr).groups()[1:]
        x = bool(x) and (x if not x.isdigit() else int(x))
        y = y if not y.isdigit() else int(y)
        c[z] = circ.wire(x, om[o], y)
    except AttributeError:
        x,z = no_op.match(instr).groups()
        x = x if not x.isdigit() else int(x)
        c[z] = circ.wire(x)

print c['a']

b_val = c['a']
c.reset()
c.get('b').val = b_val
print c['a']
