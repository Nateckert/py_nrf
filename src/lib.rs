//! A crate to make available newton_rootfinder available in Python
//!
//! It is a distinct library as it is using Rust nightly due to the PyO3 dependencies

pub mod model_trait;

use model_trait::UserModel;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn py_nrf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UserModel>()?;
    //m.add_wrapped(wrap_pyfunction!(solve_wrapper)).unwrap();
    Ok(())
}
