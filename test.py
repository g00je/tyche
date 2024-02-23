import random
import string

import plutus


def randstr():
    return ''.join(random.choices(
        string.ascii_letters + string.digits + '🌊🌪🍌🐧',
        k=random.randrange(0, 1024)
    ))


#
user = plutus.User()
# print(dir(user))
# assert user.flag == 0
# user.flag = 2939
# assert user.flag == 2939
# print(f'------------\n{user}')


# gene = plutus.Gene
# gene = plutus.Gene('ff' * 8)
# other = plutus.Gene(bytes.fromhex('ff' * 8))
# print(gene, other)
# assert gene == other
# assert gene == gene
# user.gene = gene
# gene.id = 12
# gene.server = 33
# gene.pepper = 0xf2ba
user.gg = b'H' * 12
print(user.gg)

# user.gene = bytes.fromhex('f2' * 8)
# print(bytes(user.gene))
# print(bytes(user))
# assert bytes(user) == bytes(user)

# g1 = plutus.Gene()
# g1.id = 292
# g1.pepper = 2921
# g1.server = 49971
# g2 = plutus.Gene(g1)
# assert g1.id == g2.id
#
# print(bytes(g1))
#
# g3 = plutus.Gene(bytes(g1))
# assert g1.id == g3.id
#
# g4 = plutus.Gene(bytes(g1).hex())
# assert g1.id == g4.id
#
# print(f'{g1=}')
# print(f'{g2=}')
# print(f'{g3=}')
# print(f'{g4=}')
