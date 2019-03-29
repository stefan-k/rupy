use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
/// blah
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
/// blah
fn closure(func: PyObject) -> PyResult<()> {
    let gil_guard = Python::acquire_gil();
    let py = gil_guard.python();
    func.call0(py)?;
    Ok(())
}

/// python module
#[pymodule]
fn rupy(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(closure))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
