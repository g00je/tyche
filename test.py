import json
import random
import string

import plutus


def randstr():
    return ''.join(random.choices(
        string.ascii_letters + string.digits + '🌊🌪🍌🐧',
        k=random.randrange(0, 1024)
    ))


user = plutus.User(b'\x00' * plutus.User.SIZE)
assert len(user.dict()['name']) == 0
assert len(user.name) == 0

# print(json.dumps(user.dict(), indent=4))
