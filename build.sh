#!/bin/sh

# cargo watch -c -x 'build --offline > src/main.rs'\
#     -s 'echo -e "\n\nfn main() {}" >> src/main.rs'\
#     -s 'maturin develop --offline'
#     # -s 'python test.py'
#
# exit

base_dir=~/projects/g00je
core_dir=$base_dir/core
tyche_dir=$base_dir/tyche
plutus_dir=$base_dir/plutus

function check {
    if [ $1 != 0 ]; then
        echo "Error (exit code: $1): $2 failed!"
        exit 1;
    else
        echo "Ok: $2"
    fi
}

source $tyche_dir/.env/bin/activate

cd "$(dirname "$0")"

echo -e "maturin build -r -o dist"
maturin build -q -r -o dist
check $? "martin build"

echo -e "pip install plutus_internal in tyche"
pip install ./dist/plutus_internal-*.whl --force-reinstall -q
check $? "pip install in tyche"

echo "cargo run"
cargo run -q
check $? "cargo run"

cd pkg
# send stdout to null
echo "python build sdist in pkg"
python -m build --no-isolation --sdist --wheel --outdir dist/ .
check $? "building the python package plutus"

echo "pip install plutus in tyche"
pip install ./dist/plutus-*.whl --force-reinstall -q
check $? "pip install plutus in tyche"

python ../test.py
check $? "tests"

mkdir -p $plutus_dir/include/
cp -f models.h $plutus_dir/include/

echo "pip install in $core_dir"
source $core_dir/.env/bin/activate

pip install ../dist/plutus_internal-*.whl --force-reinstall -q
check $? "pip install plutus_internal in core"

pip install ./dist/plutus-*.whl --force-reinstall -q
check $? "pip install plutus in core"


echo "pip install in $plutus_dir"
source $plutus_dir/.env/bin/activate

pip install ../dist/plutus_internal-*.whl --force-reinstall -q
check $? "pip install plutus_internal in plutus"

pip install ./dist/plutus-*.whl --force-reinstall -q
check $? "pip install plutus in plutus"

source $tyche_dir/.env/bin/activate

# rm -rf dist ../dist build *.egg-info

