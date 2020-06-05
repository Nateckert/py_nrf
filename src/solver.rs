use pyo3::prelude::*;

use newton_rootfinder::solver_advanced as nrf;
use nrf::model::Model;
use crate::model_trait::UserModel;

#[pyfunction]
pub fn solve_finite_diff(path_config: &str, model: &mut UserModel) {

    let (solver_parameters, iteratives_fd, stopping_criterias, update_methods) = nrf::util::from_xml_finite_diff(path_config);
    let iteratives = nrf::iteratives::Iteratives::new(&iteratives_fd);
    let residuals_config =
       nrf::residuals::ResidualsConfig::new(&stopping_criterias, &update_methods);
    let problem_size = solver_parameters.get_problem_size();

    let init = model.get_iteratives();

    let mut rf = nrf::solver::RootFinder::new(
       solver_parameters,
       init,
       &iteratives,
       &residuals_config,
    );

    rf.solve(model);
}
