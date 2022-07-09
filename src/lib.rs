use pyo3::prelude::*;

use biscuit_auth::{builder::*, error, Authorizer, Biscuit, KeyPair as BiscKeyPair};

#[pyclass]
struct KeyPair {
    key_pair: BiscKeyPair,
}

/// A Python module implemented in Rust.
#[pymodule]
fn biscuit_auth(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KeyPair>()?;
    Ok(())
}
