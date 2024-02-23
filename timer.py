

import hashlib
import json
import test
import time
import timeit

import tyche

data = {
    'user_data': b'\xf2' * 440,
    'pic_data': b'\xd1' * 8
}

a = tyche.User(data['user_data'])
b = test.User.from_raw(data['user_data'])
a.sessions[0].timestamp = int(time.time())
a.sessions[0].token = hashlib.sha512(b'hi').digest()
a.sessions[0].name = 'hi 1' * 40
ad: dict = a.dict()
bd: dict = b.dict()

print(json.dumps(ad, indent=2))


a_time = timeit.timeit(
    stmt='''
user = User(user_data)
user.dict()
    ''',
    number=1_00_000,
    globals={
        **data,
        'User': tyche.User,
        'Session': tyche.Session,
        'Picture': tyche.Picture
    }
)


print(f'{a_time= }')

b_time = timeit.timeit(
    stmt='''
user = User.from_raw(user_data)
user.dict()
    ''',
    number=1_00_000,
    globals={
        **data,
        'User': test.User,
        'Session': test.Session,
        'Picture': test.Picture
    }
)

print(f'{b_time= }')

# print('a:', a.gene)
# print('b:', b.gene)
# assert bytes(a) == bytes(b)
# timeit.timeit()
