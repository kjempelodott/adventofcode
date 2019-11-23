packages = open('day2_input', 'r').read().strip().split('\n')

print sum((lambda h,w,l: 3*h*w+2*l*(w+h))(*sorted(map(int, pkg.split('x')))) for pkg in packages)

print sum((lambda h,w,l: h+h+w+w+h*w*l)(*sorted(map(int, pkg.split('x')))) for pkg in packages)
