# !/bin/bash

set -e

VENV=venv-bench
LIBRARIES=("zxcvbn" "zxcvbn-rs-py")

rm -rf $VENV
python -m venv $VENV

for LIBRARY in ${LIBRARIES[@]}
do
  ./$VENV/bin/pip install $LIBRARY
done

./$VENV/bin/python benchmark/run.py
gnuplot benchmark/plot.plt
