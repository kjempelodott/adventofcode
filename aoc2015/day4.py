from hashlib import md5

secret = 'bgvyzdsv'
salt = 0
while md5(secret + str(salt)).hexdigest()[:5] != '0'*5:
    salt += 1
print salt

while md5(secret + str(salt)).hexdigest()[:6] != '0'*6:
    salt += 1
print salt
