
from distutils.core import setup

setup(
    name='plutus',
    version='0.0.1',
    description='Plutus types to Python objects and vice versa',
    author='i007c',
    author_email='dr007cc@gmail.com',
    packages=['plutus'],
    package_data={'plutus': ['__init__.pyi', 'py.typed']},
    zip_safe=True,
)
