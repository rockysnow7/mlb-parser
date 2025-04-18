mod parser;

use parser::Parser;
use pyo3::prelude::*;
use rzozowski::Regex;

#[pyfunction]
fn get_next_valid_chars(prefix: &str, pattern: &str) -> PyResult<Vec<char>> {
    let mut regex = Regex::new(pattern).unwrap();
    for c in prefix.chars() {
        regex = regex.derivative(c);
    }

    let mut valid_chars = Vec::new();
    for c in 0..=255u8 {
        if regex.derivative(c as char) != Regex::Empty {
            valid_chars.push(c as char);
        }
    }
    Ok(valid_chars)
}

#[pymodule]
fn mlb_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Parser>()?;
    m.add_function(wrap_pyfunction!(get_next_valid_chars, m)?)?;

    Ok(())
}
