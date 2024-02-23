

import ctypes
from ctypes import LittleEndianStructure, c_char, c_ubyte, c_uint8, c_uint16
from ctypes import c_uint32, c_uint64, sizeof

BYTE_ORDER = 'little'


class Struct(LittleEndianStructure):

    @classmethod
    def from_raw(cls, raw: bytes):
        size = sizeof(cls)
        if not isinstance(raw, bytes):
            raise ValueError('raw data must be bytes')

        if len(raw) % size:
            raise ValueError('invalid raw data length')

        total = int(len(raw) / size)

        if total == 1:
            return cls.from_buffer_copy(raw)

        return [
            cls.from_buffer_copy(raw[i*size:(i+1)*size]) for i in range(total)
        ]

    def __repr__(self):
        return f'<{self.__class__.__name__} {sizeof(self)} />'


class Session(Struct):
    _fields_ = [
        ('_ip', ctypes.c_uint8 * 4),
        ('_name', ctypes.c_char * 36),
        ('timestamp', ctypes.c_uint64),
        ('token', ctypes.c_ubyte * 64)
    ]

    def dict(self):
        return {
            'ip': self.ip,
            'name': self.name,
            'timestamp': self.timestamp,
            'token': bytes(self.token),
        }

    @property
    def name(self) -> str:
        return self._name.decode(errors='ignore')

    @name.setter
    def name(self, value: str):
        value = str(value[:36])
        self._name = value.encode('utf8')

    @property
    def ip(self) -> str:
        return '.'.join([str(i) for i in self._ip])

    @ip.setter
    def ip(self, value: str):
        if not isinstance(value, str):
            raise TypeError('ip must be a str')

        value = value.split('.')
        if len(value) != 4:
            raise ValueError('invalid ip')

        for i, v in enumerate(value):
            v = int(v)
            if v > 255 or v < 0:
                raise ValueError('invalid ip')

            self._ip[i] = v


assert sizeof(Session) == 112


class GeneStruct(Struct):
    _fields_ = [
        ('id', ctypes.c_uint32),
        ('pepper', ctypes.c_uint16),
        ('server', ctypes.c_uint16),
    ]


class Gene(ctypes.LittleEndianUnion):
    _anonymous_ = ('_s',)
    _fields_ = [
        ('_s', GeneStruct),
        ('raw', ctypes.c_uint64),
    ]

    def __str__(self):
        return bytes(self).hex()

    def __int__(self):
        return self.raw

    @property
    def hex(self):
        return str(self)

    @hex.setter
    def hex(self, value: str):
        value = str(value)
        if len(value) != 16:
            raise ValueError('invalid gene length')

        self.raw = int(value, base=16)

    @classmethod
    def from_hex(cls, value: str) -> 'Gene':
        value = str(value)
        if len(value) != 16:
            raise ValueError('invalid gene length')

        return cls(raw=int(value, base=16))


assert sizeof(Gene) == 8


class Picture(Struct):
    _fields_ = [
        ('server', ctypes.c_uint32),
        ('ext', ctypes.c_uint8),
        ('salt', ctypes.c_ubyte * 3),
    ]


assert sizeof(Picture) == 8


class User(Struct):
    flag: int
    gene: Gene
    agent: Gene
    reviews: Gene
    picture: Picture
    cc: int
    session: list[Session]

    _fields_ = [
        ('flag', ctypes.c_uint64),
        ('gene', Gene),
        ('agent', Gene),
        ('reviews', Gene),
        ('picture', Picture),
        ('_phone', ctypes.c_char * 12),
        ('cc', ctypes.c_uint16),
        ('_name', ctypes.c_char * 50),
        ('sessions', Session * 3)
    ]

    def dict(self):
        return {
            'flag': self.flag,
            'cc': self.cc,
            'phone': self.phone,
            'name': self.name,
            'gene': str(self.gene),
            'agent': str(self.agent),
            'reviews': str(self.reviews),
            'has_reviews': bool(self.reviews),
            'sessions': [s.dict() for s in self.sessions]
        }

    @property
    def phone(self) -> str:
        return self._phone.decode(errors='ignore')

    @phone.setter
    def phone(self, value: str):
        value = str(value[:12])

        for c in value:
            if c not in '0123456789':
                raise ValueError('invalid phone number')

        self._phone = value.encode('utf8')

    @property
    def name(self) -> str:
        return self._name.decode(errors='ignore')

    @name.setter
    def name(self, value: str):
        value = str(value[:50])
        self._name = value.encode('utf8')


assert sizeof(User) == 440


# user = User
# a, b = User.from_raw(b'\x01\x02' * 440)
# print(a.cc)
# print(a)
# user = User.from_buffer_copy(b'\xf1' * 440)
# user.gene.id = 12
# print(str(user.gene))
# print(int(user.gene))
# print(Gene.from_hex('12' * 8))
# print(user.gene.from_hex('11' * 8))
# print((bytes(user)))
# print(user.picture.salt[0])
# user.picture.salt = b'\x01\x02\x00'
# print(user.picture.salt[0])


#
