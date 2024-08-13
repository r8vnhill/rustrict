/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */
mod constraints;
mod errors;
mod string_scope;

use errors::constraint_error::ConstraintError;
use std::sync::{Arc, Mutex};
use string_scope::StringScope;

struct RustrictScope {
    results: Arc<Mutex<Vec<Result<(), ConstraintError>>>>,
}

impl RustrictScope {
    fn new() -> RustrictScope {
        RustrictScope {
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn results(&self) -> Vec<Result<(), ConstraintError>> {
        self.results.lock().unwrap().clone()
    }

    fn failures(&self) -> Vec<ConstraintError> {
        self.results
            .lock()
            .unwrap()
            .iter()
            .filter_map(|r| r.as_ref().err().cloned())
            .collect()
    }

    fn validate_string<F>(&self, message: &str, predicate: F)
    where
        F: FnOnce(&mut StringScope),
    {
        let mut scope = StringScope::new(message.to_string(), Arc::clone(&self.results));
        predicate(&mut scope);
    }

    fn validate_string_with_custom_exception<F, G>(
        &self,
        message: &str,
        exception_generator: G,
        predicate: F,
    ) where
        F: FnOnce(&mut StringScope),
        G: Fn(String) -> ConstraintError + 'static,
    {
        let mut scope = StringScope::new_with_exception_generator(
            message.to_string(),
            Arc::clone(&self.results),
            Box::new(exception_generator),
        );
        predicate(&mut scope);
    }
}
