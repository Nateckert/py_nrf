//! A crate to make available newton_rootfinder available in Python
//!
//! It is a distinct library as it is using Rust nightly due to the PyO3 dependencies

pub mod model_trait;
pub use model_trait::UserModel;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use newton_rootfinder::solver_advanced as nrf;
use nrf::model::Model;

#[pymodule]
fn py_nrf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<UserModel>()?;
    m.add_wrapped(wrap_pyfunction!(solve_finite_diff)).unwrap();
    Ok(())
}

#[pyfunction]
pub fn solve_finite_diff(path_config: &str, model: &mut UserModel) {
    let (solver_parameters, iteratives_fd, stopping_criterias, update_methods) =
        nrf::util::from_xml_finite_diff(path_config);
    let iteratives = nrf::iteratives::Iteratives::new(&iteratives_fd);
    let residuals_config =
        nrf::residuals::ResidualsConfig::new(&stopping_criterias, &update_methods);

    let init = model.get_iteratives();

    let mut rf =
        nrf::solver::RootFinder::new(solver_parameters, init, &iteratives, &residuals_config);

    rf.solve(model);
}
