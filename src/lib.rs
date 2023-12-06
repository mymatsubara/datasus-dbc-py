use pyo3::{exceptions::PyValueError, prelude::*, types::PyBytes};
use std::{
    fmt,
    io::{self, Read},
};

fn to_py_result<T, Error>(e: std::result::Result<T, Error>) -> PyResult<T>
where
    Error: fmt::Display,
{
    e.map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Decompress a .dbc file into a .dbf file
#[pyfunction]
fn decompress(dbc_path: String, dbf_path: String) -> PyResult<()> {
    to_py_result(datasus_dbc::decompress(dbc_path, dbf_path))
}

/// Decompress a .dbc file in memory and return the resulting .dbf as bytes
#[pyfunction]
fn decompress_bytes(py: Python, dbc_bytes: Vec<u8>) -> PyResult<PyObject> {
    let dbc_reader = io::Cursor::new(&dbc_bytes);
    let mut dbf_reader = to_py_result(datasus_dbc::into_dbf_reader(dbc_reader))?;
    let mut dbf_bytes = vec![];
    to_py_result(dbf_reader.read_to_end(&mut dbf_bytes))?;

    Ok(PyBytes::new(py, &dbf_bytes).into())
}

/// Decompress DATASUS's *.dbc files into *.dbf files
#[pymodule]
#[pyo3(name = "datasus_dbc")]
fn datasus_dbc_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decompress, m)?)?;
    m.add_function(wrap_pyfunction!(decompress_bytes, m)?)?;

    Ok(())
}
