import re
code_lines = open('day8_input', 'rb').read().strip().split('\n')
print sum(len(l) - len(eval(l)) for l in code_lines)
print sum(len(re.escape(l)) - len(l) + 2 for l in code_lines)
