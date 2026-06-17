use rust_bpmn_analyzer::model_checking::properties::{Property};
use rust_bpmn_analyzer::read_bpmn_from_string;
use pyo3::prelude::*;

mod dtos;
use dtos::CounterExample;

#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct PyProperty {
    #[pyo3(get)]
    pub property_name: String,

    #[pyo3(get)]
    pub fulfilled: bool,

    #[pyo3(get)]
    pub problematic_elements: Vec<String>,

    #[pyo3(get)]
    pub description: String,

    /// JSON-encoded counterexample (`{start_state, transitions}`) showing the
    /// path to the first problematic state, or `None` when the property holds
    /// (no counterexample). Parsed into typed models on the Python side.
    #[pyo3(get)]
    pub counter_example: Option<String>,
}

#[pymethods]
impl PyProperty {
    #[new]
    fn new(
        property_name: String,
        fulfilled: bool,
        problematic_elements: Vec<String>,
        description: String,
        counter_example: Option<String>,
    ) -> Self {
        PyProperty {
            property_name,
            fulfilled,
            problematic_elements,
            description,
            counter_example,
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PyProperty(property_name='{}', fulfilled={}, problematic_elements={:?}, description={:?}, counter_example={:?})",
            self.property_name, self.fulfilled, self.problematic_elements, self.description, self.counter_example,
        ))
    }
}

fn build_counter_example(
    problematic_state_hashes: Vec<u64>,
    state_space: &rust_bpmn_analyzer::states::state_space::StateSpace,
) -> Option<String> {
    CounterExample::new(problematic_state_hashes, state_space)
        .and_then(|ce| serde_json::to_string(&ce).ok())
}

fn type_to_description(property: &Property) -> String {
    match property {
        Property::Safeness => "The process model properly synchronizes concurrent activities.".to_string(),
        Property::OptionToComplete => "The process model can definitively reach its end state, ensuring that all started activities have a clear path to completion.".to_string(),
        Property::ProperCompletion => "There is a single unambiguous way to reach the final end event.".to_string(),
        Property::NoDeadActivities => "All activities in the process model are reachable and can be executed".to_string(),
    }
}

#[pyfunction]
fn analyze_safeness(model: &str) -> PyResult<PyProperty> {
    match read_bpmn_from_string(model) {
        Ok(collaboration) => {
            let mut model_checking_result = rust_bpmn_analyzer::run(&collaboration, vec![Property::Safeness]);

            let result = model_checking_result.property_results.remove(0);
            let counter_example =
                build_counter_example(result.problematic_state_hashes, &model_checking_result.state_space);

            Ok(PyProperty {
                property_name: "Safeness".to_string(),
                fulfilled: result.fulfilled,
                problematic_elements: result.problematic_elements,
                description: type_to_description(&result.property),
                counter_example,
            })
        }
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            err.to_string(),
        )),
    }
}

#[pyfunction]
fn analyze_dead_activities(model: &str) -> PyResult<PyProperty> {
    match read_bpmn_from_string(model) {
        Ok(collaboration) => {
            let mut model_checking_result = rust_bpmn_analyzer::run(&collaboration, vec![Property::NoDeadActivities]);

            let result = model_checking_result.property_results.remove(0);
            let counter_example =
                build_counter_example(result.problematic_state_hashes, &model_checking_result.state_space);

            Ok(PyProperty {
                property_name: "No dead activities".to_string(),
                fulfilled: result.fulfilled,
                problematic_elements: result.problematic_elements,
                description: type_to_description(&result.property),
                counter_example,
            })
        }
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            err.to_string(),
        )),
    }
}

#[pyfunction]
fn analyze_option_to_complete(model: &str) -> PyResult<PyProperty> {
    match read_bpmn_from_string(model) {
        Ok(collaboration) => {
            let mut model_checking_result = rust_bpmn_analyzer::run(&collaboration, vec![Property::OptionToComplete]);

            let result = model_checking_result.property_results.remove(0);
            let counter_example =
                build_counter_example(result.problematic_state_hashes, &model_checking_result.state_space);

            Ok(PyProperty {
                property_name: "Option to complete".to_string(),
                fulfilled: result.fulfilled,
                problematic_elements: result.problematic_elements,
                description: type_to_description(&result.property),
                counter_example,
            })
        }
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            err.to_string(),
        )),
    }
}

#[pyfunction]
fn analyze_proper_completion(model: &str) -> PyResult<PyProperty> {
    match read_bpmn_from_string(model) {
        Ok(collaboration) => {
            let mut model_checking_result = rust_bpmn_analyzer::run(&collaboration, vec![Property::ProperCompletion]);

            let result = model_checking_result.property_results.remove(0);
            let counter_example =
                build_counter_example(result.problematic_state_hashes, &model_checking_result.state_space);

            Ok(PyProperty {
                property_name: "Proper completion".to_string(),
                fulfilled: result.fulfilled,
                problematic_elements: result.problematic_elements,
                description: type_to_description(&result.property),
                counter_example,
            })
        }
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            err.to_string(),
        )),
    }
}

// This is the module "entrypoint" for Python
#[pymodule]
fn rust_bpmn_analyzer_bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyProperty>()?;
    m.add_function(wrap_pyfunction!(analyze_dead_activities, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_option_to_complete, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_proper_completion, m)?)?;
    m.add_function(wrap_pyfunction!(analyze_safeness, m)?)?;

    m.add(
        "__all__",
        vec![
            "PyProperty",
            "analyze_dead_activities",
            "analyze_option_to_complete",
            "analyze_proper_completion",
            "analyze_safeness",
        ],
    )?;

    Ok(())
}
