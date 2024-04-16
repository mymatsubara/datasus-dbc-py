# datasus-dbc-py

[![PyPI version](https://badge.fury.io/py/datasus-dbc.svg)](https://pypi.org/project/datasus-dbc/)

Decompress `*.dbc` files usually found in Brazil's DATASUS ftp server into `*.dbf` files in python.

This are python bindings of the sibling library [datasus-dbc](https://crates.io/crates/datasus-dbc) which is written in rust. This library should be **compatible with most platforms**.

## Instalation

```
pip install datasus-dbc
```

## Examples

To decompress a `*.dbc` file into a `*.dbf` use the `decompress` function:

```python
import datasus_dbc

datasus_dbc.decompress("input.dbc", "output.dbf")
```

If you have a `*.dbc` file's raw bytes, you can use `decompress_bytes` to get the decompressed `*.dbf` raw bytes in memory:

```python
import datasus_dbc

with open("input.dbc", "rb") as file:
    dbf_bytes = datasus_dbc.decompress_bytes(file.read())
    print(dbf_bytes)
```

## Reading *.dbf files

This library does not support reading the contents of a `*.dbf` file, but you can still use one of the following libraries: [simpledbf](https://pypi.org/project/simpledbf/) or [dbfread](https://pypi.org/project/dbfread/1.0.6/).

## Found a bug?
Feel free to create an [issue](https://github.com/mymatsubara/datasus-dbc-py/issues/new) here if you found a bug or if you want a new feature!