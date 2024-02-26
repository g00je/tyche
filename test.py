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

genes = plutus.Gene.batch(b'g' * plutus.Gene.SIZE * 10)
print(genes, len(genes))

# print(json.dumps(user.dict(), indent=4))
