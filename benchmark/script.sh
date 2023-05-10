# !/bin/bash

set -e

VENV=venv-bench
LIBRARIES=("zxcvbn" "./target/wheels/zxcvbn_rs_py-0.1.0-cp310-cp310-macosx_11_0_arm64.whl")

rm -rf $VENV
python -m venv $VENV

for LIBRARY in ${LIBRARIES[@]}
do
  ./$VENV/bin/pip install $LIBRARY
done

./$VENV/bin/python benchmark/run.py
gnuplot benchmark/plot.plt
