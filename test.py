import json
import random
import string

import plutus


def randstr():
    return ''.join(random.choices(
        string.ascii_letters + string.digits + '🌊🌪🍌🐧',
        k=random.randrange(0, 1024)
    ))


user = plutus.User(b'A' * plutus.User.SIZE)
print(json.dumps(user.dict(), indent=4))
