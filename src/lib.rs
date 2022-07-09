use pyo3::prelude::*;

use biscuit_auth::builder::BiscuitBuilder as BABiscuitBuilder;
use biscuit_auth::{builder::*, error, Biscuit as BABiscuit, KeyPair as BAKeyPair};

#[pyclass]
struct KeyPair {
    key_pair: BAKeyPair,
}

#[pymethods]
impl KeyPair {
    #[new]
    fn py_new() -> Self {
        return Self {
            key_pair: BAKeyPair::new(),
        };
    }
}

#[pyclass]
struct Biscuit {
    biscuit: BABiscuit,
}

#[pymethods]
impl Biscuit {
    #[staticmethod]
    fn builder(root: &KeyPair) -> PyResult<BiscuitBuilder> {
        return Ok(BiscuitBuilder {
            builder: BABiscuit::builder(root),
        });
    }
}

#[pyclass]
struct BiscuitBuilder {
    builder: BABiscuitBuilder,
}

#[pymodule]
fn biscuit_auth(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KeyPair>()?;
    m.add_class::<Biscuit>()?;
    m.add_class::<BiscuitBuilder>()?;

    Ok(())
}
