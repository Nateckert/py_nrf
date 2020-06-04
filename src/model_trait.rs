use nalgebra;
use pyo3::prelude::*;
use newton_rootfinder::solver_advanced as nrf;

// Required
// fn evaluate(&mut self)
// fn len_problem(&self) -> usize
// fn set_iteratives(&mut self, iteratives: &DVector<f64>)
// fn get_iteratives(&self) -> DVector<f64>
// fn get_residuals(&self) -> ResidualsValues
//
// Provided:
// fn init(&self)
// fn len_memory(&self) -> usize
// fn set_memory(&mut self, memory: &DVector<f64>)
// fn get_memory(&self) -> DVector<f64>
// fn jacobian_provided(&self) -> bool (default false)
// fn get_jacobian(&self) -> JacobianValues (default (zeros, zeros))
use nrf::model::Model;

#[pyclass]
pub struct UserModel {
    model: Py<PyAny>,
}

impl Model for UserModel {
    fn evaluate(&mut self) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.model
            .as_ref(py)
            .call_method("evaluate", (), None)
            .unwrap();
    }

    fn len_problem(&self) -> usize {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let py_result = self.model
            .as_ref(py)
            .call_method("len_problem", (), None);

        let py_value = match py_result {
            Ok(value) => value,
            Err(_) => panic!("Error calling the len_problem() method.\nThe existence of the variable self.len_problem could lead to this issue. It can be modified into self._len_problem"),
        };

        if py_value.get_type().name() != "int" {
            panic!(
                "Expected a integer for the len_problem() method signature, got {}",
                py_value.get_type().name()
            );
        }

        py_value.extract().unwrap()
    }

    fn set_iteratives(&mut self, var: &nalgebra::DVector<f64>) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let values: Vec<f64> = var.as_slice().to_vec().clone();
        let list: PyObject = values.into_py(py);
        let py_model = self.model.as_ref(py);
        py_model
            .call_method("set_iteratives", (list,), None)
            .unwrap();
    }

    fn get_iteratives(&self) -> nalgebra::DVector<f64> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let py_result: &PyAny = self
            .model
            .as_ref(py)
            .call_method("get_iteratives", (), None)
            .unwrap();

        if py_result.get_type().name() != "list" {
            panic!(
                "Expected a list for the get_results() method signature, got {}",
                py_result.get_type().name()
            );
        }
        let vec_result = py_result.extract().unwrap();
        nalgebra::DVector::from_vec(vec_result)
    }

    fn get_residuals(&self) -> nrf::residuals::ResidualsValues {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let py_result: &PyAny = self
            .model
            .as_ref(py)
            .call_method("get_residuals", (), None)
            .unwrap();

        // TODO : remove this println! and
        // check that the return type is a tuple of length 2
        println!("{}", py_result.get_type().name());

        let py_left: &PyAny = py_result.get_item(0).unwrap();
        let py_right: &PyAny = py_result.get_item(1).unwrap();

        if (py_left.get_type().name() != "list") | (py_right.get_type().name() != "list") {
            panic!("Expected a tuple of two lists for the get_residuals() method signature, got ({}, {})", py_left.get_type().name(), py_right.get_type().name());
        }

        let left = py_left.extract().unwrap();
        let right = py_right.extract().unwrap();

        nrf::residuals::ResidualsValues::new(
            nalgebra::DVector::from_vec(left),
            nalgebra::DVector::from_vec(right),
        )
    }
}

#[pymethods]
impl UserModel {
    #[new]
    pub fn new(model: Py<PyAny>) -> Self {
        UserModel { model }
    }
    pub fn evaluate_rust(&mut self) -> PyResult<()> {
        self.evaluate();
        Ok(())
    }
    pub fn len_problem_rust(&self) -> PyResult<usize> {
        Ok(self.len_problem())
    }
    pub fn set_iteratives_rust(&mut self, inputs: Vec<f64>) -> PyResult<()> {
        self.set_iteratives(&nalgebra::DVector::from_vec(inputs));
        Ok(())
    }
}
