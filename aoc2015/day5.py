strings = open('day5_input', 'r').read().strip().split('\n')

print sum(all([(x not in s) for x in ('ab','cd','pq','xy')]) and \
          (sum([s.count(v) for v in 'aeiou']) > 2) and \
          any([t[0]==t[1] for t in zip(s, s[1:])])
          for s in strings)

import re
print len(filter(None, [re.search(r'(.).\1', s) and re.search(r'(..).*?\1', s)
                        for s in strings]))
