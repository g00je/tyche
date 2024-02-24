import random
import string

import plutus


def randstr():
    return ''.join(random.choices(
        string.ascii_letters + string.digits + '🌊🌪🍌🐧',
        k=random.randrange(0, 1024)
    ))


user = plutus.User()
user.gene.id = 12
# user.gene = plutus.Gene('2f' * 8)
print(user)
assert user.gene is user.gene
