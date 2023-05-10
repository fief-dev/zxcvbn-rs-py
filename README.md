# zxcvbn-rs-py

<p align="center">
    <em>Python bindings for <a href="https://github.com/shssoichiro/zxcvbn-rs">zxcvbn-rs</a>, the Rust implementation of zxcvbn</em>
</p>

[![build](https://github.com/fief-dev/zxcvbn-rs-py/workflows/CI/badge.svg)](https://github.com/fief-dev/zxcvbn-rs-py/actions)
[![PyPI version](https://badge.fury.io/py/zxcvbn-rs-py.svg)](https://badge.fury.io/py/zxcvbn-rs-py)

---

**Documentation**: <a href="https://fief-dev.github.io/zxcvbn-rs-py/" target="_blank">https://fief-dev.github.io/zxcvbn-rs-py/</a>

**Source Code**: <a href="https://github.com/fief-dev/zxcvbn-rs-py" target="_blank">https://github.com/fief-dev/zxcvbn-rs-py</a>

---

## Installation

```sh
pip install zxcvbn-rs-py
```

## Quickstart

```py
from zxcvbn_rs_py import zxcvbn

r = zxcvbn("correcthorsebatterystaple")
print(r.score)
```

## Benchmark

Thanks to its Rust core, zxcvbn-rs-py is **~5 times faster** than the pure Python implementation, [zxcvbn-python](https://github.com/dwolfhub/zxcvbn-python).

![zxcvbn-rs-py benchmark](https://raw.githubusercontent.com/fief-dev/zxcvbn-rs-py/main/benchmark/benchmark.svg?sanitize=true)

## Serve the documentation

You can serve the Mkdocs documentation with:

```bash
hatch run docs-serve
```

It'll automatically watch for changes in your code.

## License

This project is licensed under the terms of the MIT license.
