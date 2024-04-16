# datasus-dbc-py

[![PyPI version](https://badge.fury.io/py/datasus-dbc.svg)](https://pypi.org/project/datasus-dbc/)

This python library enables the decompression of `*.dbc` files commonly found on Brazil's DATASUS FTP server into `*.dbf` files.

This is a python bindings of the sibling library [datasus-dbc](https://crates.io/crates/datasus-dbc), written in rust. Because of that, this library should be **compatible with most platforms**.

## Instalation

```
pip install datasus-dbc
```

## Usage

- To decompress a `*.dbc` file into a `*.dbf`, use the `decompress` function:

```python
import datasus_dbc

datasus_dbc.decompress("input.dbc", "output.dbf")
```

- If you have the raw bytes of a `*.dbc` file, you can use `decompress_bytes` function to obtain the decompressed `*.dbf` raw bytes in memory:

```python
import datasus_dbc

with open("input.dbc", "rb") as file:
    dbf_bytes = datasus_dbc.decompress_bytes(file.read())
    print(dbf_bytes)
```

## Reading *.dbf files

This library does not support reading the contents of a `*.dbf` file. However, you can still use one of the following libraries: [simpledbf](https://pypi.org/project/simpledbf/) or [dbfread](https://pypi.org/project/dbfread/1.0.6/).

## Found a bug?
If you encounter a bug or have a feature request, please feel free to create an [issue](https://github.com/mymatsubara/datasus-dbc-py/issues/new) on our GitHub repository. We welcome your feedback!