

from jinja2 import Environment, FileSystemLoader, Template

env = Environment(
    loader=FileSystemLoader('jtm')
)

temp = env.get_template('session.c')


with open('out/test.c', 'w') as f:
    f.write(temp.render(cool='\ngg', name='Session'))
