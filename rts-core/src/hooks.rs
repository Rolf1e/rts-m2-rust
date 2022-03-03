use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use crate::entity::game_actions::Action;
use crate::exceptions::RtsException;

type InnerMap<'p> = Arc<Mutex<HashMap<u64, &'p PyModule>>>;

pub struct PythonCodeExecutor<'p> {
    compiled_ai_code: InnerMap<'p>,
}

impl<'p> Default for PythonCodeExecutor<'p> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'p> PythonCodeExecutor<'p> {
    pub fn register_new_ai(&self, ai_code: String) -> Result<(), RtsException> {
        let hash = calculate_hash(&ai_code);

        let compiled_ai_code_ptr = Arc::clone(&self.compiled_ai_code);
        let mut compiled_ai_code_mutex = compiled_ai_code_ptr.lock().map_err(|_| {
            RtsException::PythonException(format!(
                "Failed to acquire mutex when register new ai with hash {}",
                hash.clone()
            ))
        })?;

        Python::with_gil(|py| {
            let module = compile_python_code(py, &ai_code, &hash)
                .map_err(|_| RtsException::PythonCompileCodeException(hash.to_string()))?;

            let insert = compiled_ai_code_mutex.insert(hash, module);
            if insert.is_none() {
                return Err(RtsException::PythonException(format!(
                    "Failed to insert ai {}",
                    hash
                )));
            }
            Ok(())
        });

        todo!()
    }

    pub fn execute_python(&self, ai_code: &str, hash: u64) -> Result<Action, RtsException> {
        todo!()
    }

    fn new() -> Self {
        PythonCodeExecutor {
            compiled_ai_code: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

fn compile_python_code<'p>(
    py: Python<'p>,
    ai_code: &str,
    hash: &u64,
) -> Result<&'p PyModule, PyErr> {
    PyModule::from_code(py, ai_code, &format!("{}.py", hash), &format!("{}", hash))
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
