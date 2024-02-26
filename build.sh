#!/bin/sh

cargo watch -c -x 'build --offline'\
    -s 'maturin develop --offline'\
    -s 'python test.py'
