import json
import random
import string

import plutus


def randstr():
    return ''.join(random.choices(
        string.ascii_letters + string.digits + '🌊🌪🍌🐧',
        k=random.randrange(0, 1024)
    ))


data = b'A' * plutus.User.SIZE

user = plutus.User(data)
user.cc = 122
print(user.SIZE)

# print(json.dumps(user.dict(), indent=4))
