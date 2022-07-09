use pyo3::prelude::*;

use biscuit_auth::builder::BiscuitBuilder as BABiscuitBuilder;
use biscuit_auth::{builder::*, error, Biscuit as BABiscuit, KeyPair as BAKeyPair};

use ouroboros::self_referencing;
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
    fn builder(py_root: &KeyPair) -> PyResult<BiscuitBuilder> {
        let root = BAKeyPair::from(py_root.key_pair.private());
        // BiscuitBuilderTryBuilder is made by oururoboros to allow store
        // BABiscuitBuilder's in a Python object. BABiscuitBuilder has a
        // required lifetime annotation, and so we adapt a technique from crfs
        // to our own needs (see
        // https://github.com/messense/crfs-rs/blob/main/python/src/lib.rs#L56-L72)
        let builder_res = BiscuitBuilderTryBuilder {
            key_pair: root,
            builder_builder: |kp| Ok(BABiscuit::builder(kp)),
        }
        .try_build();
        return builder_res;
    }
}

#[pyclass]
#[self_referencing]
struct BiscuitBuilder {
    key_pair: BAKeyPair,
    #[borrows(key_pair)]
    #[not_covariant]
    builder: BABiscuitBuilder<'this>,
}

#[pymodule]
fn biscuit_auth(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KeyPair>()?;
    m.add_class::<Biscuit>()?;
    m.add_class::<BiscuitBuilder>()?;

    Ok(())
}
