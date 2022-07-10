use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use biscuit_auth::builder::{BiscuitBuilder as BABiscuitBuilder, Fact};
use biscuit_auth::{Authorizer as BAAuthorizer, Biscuit as BABiscuit, KeyPair as BAKeyPair};

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
        // BiscuitBuilderTryBuilder is made by ouroboros's self_referencing
        // macro. We need it to allow storing a BABiscuitBuilder in a Python
        // object because a BABiscuitBuilder has a required lifetime annotation
        // and those aren't allowed in `#[pyclass]` structs. We adapted a
        // technique from crfs to our own needs (see
        // https://github.com/messense/crfs-rs/blob/0e77658d65abefef8818cc25b0a57941ff41269b/python/src/lib.rs#L56-L72
        // Seeing `'this` in Rust code is very funny.
        return BiscuitBuilderTryBuilder {
            key_pair: root,
            builder_builder: |kp| Ok(BABiscuit::builder(kp)),
        }
        .try_build();
    }

    fn authorizer(&self) -> PyResult<Authorizer> {
        return AuthorizerTryBuilder {
            biscuit: self.biscuit.clone(),
            authorizer_builder: |biscuit| biscuit.authorizer(),
        }
        .try_build()
        .map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("Biscuit#authorizer error: {}", e)));
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

#[pymethods]
impl BiscuitBuilder {
    // FIXME not a good name, obvs :)
    fn add_authority_fact_only_predicate_name(&mut self, fact_name: String) -> PyResult<()> {
        let fact = Fact::new(fact_name, vec![]);
        let res = self.with_builder_mut(|b| return b.add_authority_fact(fact));
        return res.map_err(|e| {
            PyErr::new::<PyRuntimeError, _>(format!(
                "BiscuitBuilder.add_authority_fact error: {}",
                e
            ))
        });
    }

    fn build(&self) -> PyResult<Biscuit> {
        // This clone here makes this method a lil more expensive than desired
        // but resolves a lifetime issue. It would be nice to find a way to do
        // this without clone.
        let res = self
            .with_builder(|builder| return builder.clone().build())
            .map(|biscuit| Biscuit { biscuit });
        return res.map_err(|e| {
            PyErr::new::<PyRuntimeError, _>(format!("BiscuitBuilder.build error: {}", e))
        });
    }
}

#[pyclass]
#[self_referencing]
struct Authorizer {
    biscuit: BABiscuit,
    #[borrows(biscuit)]
    #[not_covariant]
    authorizer: BAAuthorizer<'this>,
}

#[pymodule]
fn biscuit_auth(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KeyPair>()?;
    m.add_class::<Biscuit>()?;
    m.add_class::<BiscuitBuilder>()?;
    m.add_class::<Authorizer>()?;

    Ok(())
}
