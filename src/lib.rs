use ndarray::Array1;
use numpy::{IntoPyArray, PyArrayDyn};
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

#[pyfunction]
/// blah
fn closure2(func: PyObject) -> PyResult<()> {
    let gil_guard = Python::acquire_gil();
    let py = gil_guard.python();
    func.call1(py, (2.56f64,))?;
    Ok(())
}

#[pyfunction]
/// blah
fn closure3(func: PyObject) -> PyResult<()> {
    let gil_guard = Python::acquire_gil();
    let py = gil_guard.python();
    let blah = Array1::from_vec(vec![1.0f64, 2.0, 3.0]);
    func.call1(py, (blah.into_pyarray(py),))?;
    Ok(())
}

/// python module
#[pymodule]
fn rupy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(closure))?;
    m.add_wrapped(wrap_pyfunction!(closure2))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
