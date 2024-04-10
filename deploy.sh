#!/bin/sh

base_dir=/g00je
core_dir=$base_dir/core
tyche_dir=$base_dir/tyche
plutus_dir=$base_dir/plutus

function check {
    if [ $1 != 0 ]; then
        echo "Error: $2 failed!"
        exit 1;
    fi
}

source $tyche_dir/.env/bin/activate

cd "$(dirname "$0")"
maturin build -o dist
check $? "martin build"
pip install ./dist/plutus_internal-*.whl --force-reinstall
check $? "pip install in tyche"

cargo run
check $? "making plutus dir (cargo run)"

cd pkg
# send stdout to null
python -m build --no-isolation --sdist --wheel --outdir dist/ .
check $? "building the python package plutus"

pip install ./dist/plutus-*.whl --force-reinstall
check $? "pip install plutus in tyche"

python ../test.py
check $? "tests"

mkdir -p $plutus_dir/include/
cp -f models.h $plutus_dir/include/

echo "pip install in $core_dir"
source $core_dir/.env/bin/activate

pip install ../dist/plutus_internal-*.whl --force-reinstall
check $? "pip install plutus_internal in core"

pip install ./dist/plutus-*.whl --force-reinstall
check $? "pip install plutus in core"


echo "pip install in $plutus_dir"
source $plutus_dir/.env/bin/activate

pip install ../dist/plutus_internal-*.whl --force-reinstall
check $? "pip install plutus_internal in plutus"

pip install ./dist/plutus-*.whl --force-reinstall
check $? "pip install plutus in plutus"

source $tyche_dir/.env/bin/activate

rm -rf dist ../dist build *.egg-info

