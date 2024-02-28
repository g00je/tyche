#!/bin/sh

cargo watch -c -x 'build --offline > src/main.rs'\
    -s 'echo -e "\n\nfn main() {}" >> src/main.rs'\
    -s 'maturin develop --offline'
    # -s 'python test.py'

exit

base_dir=~/projects/g00je
core_dir=$base_dir/core
tyche_dir=$base_dir/tyche

# active the python venv if is not active already.
if [[ -z $VIRTUAL_ENV ]]; then
    source $tyche_dir/.env/bin/activate
fi

cd "$(dirname "$0")"
maturin build -o dist
pip install ./dist/plutus_internal-*.whl --force-reinstall

cargo run
if [ $? != 0 ]; then
    echo error making the plutus dir
    exit 1
fi

cd pkg
# send stdout to null
python -m build --no-isolation --sdist --wheel --outdir dist/ .
if [ $? != 0 ]; then
    echo error while building
    exit 1
fi

pip install ./dist/plutus-*.whl --force-reinstall

python ../test.py
if [ $? != 0 ]; then
    echo error testing
    exit 1
fi

source $core_dir/.env/bin/activate
pip install ../dist/plutus_internal-*.whl --force-reinstall
pip install ./dist/plutus-*.whl --force-reinstall

source $tyche_dir/.env/bin/activate

rm -rf dist ../dist build *.egg-info

