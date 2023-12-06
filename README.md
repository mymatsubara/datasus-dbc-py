# datasus-dbc-py

[![PyPI version](https://badge.fury.io/py/datasus-dbc.svg)](https://pypi.org/project/datasus-dbc/)

Decompress `*.dbc` files usually found in Brazil's DATASUS ftp server into `*.dbf` files in python.

This are python bindings of the sibling library [datasus-dbc](https://crates.io/crates/datasus-dbc) which is written in rust. This library should be **compatible with most platforms**.

## Instalation

```
pip install datasus-dbc
```

## Examples

To decompress a `*.dbc` file into a `*.dbf` use `decompress` function:

```python
import datasus_dbc

datasus_dbc.decompress("input.dbc", "output.dbc")
```

If you have a `*.dbc` file's raw bytes, you can use `decompress_bytes` to get the decompressed `*.dbf` raw bytes in memory:

```python
import datasus_dbc

with open("input.dbc", "rb") as file:
    dbf_bytes = datasus_dbc.decompress_bytes(file.read())
    print(dbf_bytes)
```

## Found a bug?
Feel free to create an [issue](https://github.com/mymatsubara/datasus-dbc-py/issues/new) here if you found a bug or if you want a new feature!