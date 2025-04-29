use pyo3::prelude::*              ;
use unicode_width::UnicodeWidthStr;


#[pyfunction]
fn actual_len(a:&str) -> PyResult<usize> {
    Ok(UnicodeWidthStr::width(a))
}

#[pymodule]
fn libtext(m:&Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(actual_len, m)?)?;
    Ok(())
}