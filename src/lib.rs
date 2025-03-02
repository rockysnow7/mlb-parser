mod parser;

use parser::Parser;
use pyo3::prelude::*;

#[pymodule]
fn mlb_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Parser>()?;

    Ok(())
}
