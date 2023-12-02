import os
from pathlib import Path

import chevron

OUT_DIR = Path(__file__).parent / 'out'
OUT_DIR.mkdir(parents=True, exist_ok=True)

TMPL_DIR = Path(__file__).parent / 'tmpl'

TMPL = {}

for fn in os.listdir(TMPL_DIR):
    key, *_ = fn.split('.')
    with open(TMPL_DIR / fn, 'r') as f:
        TMPL[key] = f.read()


data = [
    {
        'type': 'class',
        'name': 'Session',
        'struct': '''
            typedef struct Session {
                uint8_t ip[4];
                char name[36];
                // if timestamp is 0, Session is Dead;
                uint64_t timestamp;
                byte token[64];
            } Session;
        ''',
        'fields': [
            # {
            #     'type': 'ip',
            #     'name': 'ip'
            # },
            {
                'type': 'list',
                'name': 'ip',
                'length': 4,
                'item_type': 'u8'
            },
            {
                'type': 'str',
                'name': 'name',
                'size': 36,
            },
            {
                'type': 'int',
                'name': 'timestamp',
                'size': 64
            },
            {
                'type': 'byte',
                'name': 'token',
                'size': 64,
            }
        ]
    }
]


def format_struct(struct: str) -> str:
    lines = struct.strip().split('\n')
    lines = ['    ' + line.strip() for line in lines]
    lines[0] = lines[0].strip()
    lines[-1] = lines[-1].strip()
    return '\n'.join(lines)


SIZE_TYPE = {
    32.0: 'T_FLOAT',
    64.0: 'T_DOUBLE',
    -8: 'T_BYTE',
    -16: 'T_SHORT',
    -32: 'T_INT',
    -64: 'T_LONGLONG',
    8: 'T_UBYTE',
    16: 'T_USHORT',
    32: 'T_UINT',
    64: 'T_ULONGLONG',
}


def main():
    for item in data:
        name = item['name']
        fn = name.lower() + '.c'

        struct = format_struct(item['struct'])

        view = {
            'name': name,
            'name_lower': name.lower(),
            'struct': struct,
            'bytes': [],
            'strs': [],
            'ints': []
        }

        for f in item['fields']:
            if f['type'] == 'byte':
                view['bytes'].append({'attr': f['name']})
            elif f['type'] == 'str':
                view['strs'].append({'attr': f['name']})
            elif f['type'] == 'int':
                view['ints'].append({
                    'attr': f['name'],
                    'type': SIZE_TYPE[f['size']]
                })

        out = open(OUT_DIR / fn, 'w')
        out.write(chevron.render(TMPL['main'], view))

        # out.write(TMPL['header'].substitute(struct=struct, name=name))
        # out.write(TMPL['new'].substitute(name=name))
        out.close()


if __name__ == '__main__':
    main()
