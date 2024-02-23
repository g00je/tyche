#!/bin/sh

RUSTFLAGS=-Awarnings cargo watch -c -q -x 'build -q --offline'\
    -s 'maturin develop --offline'\
    -s 'python test.py'
