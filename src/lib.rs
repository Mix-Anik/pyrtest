use pyo3::prelude::*;


/// Asserts if two python objects are equal value representations
#[pyfunction]
fn q(a: &Bound<'_, PyAny>, b: &Bound<'_, PyAny>) -> PyResult<bool> {
    Ok(a.eq(b).unwrap())
}

#[pymodule]
fn pyrtest(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(q, m)?)?;
    Ok(())
}
