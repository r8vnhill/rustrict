/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */

use crate::constraints::constraint::Constraint;
use crate::errors::constraint_error::ConstraintError;
use std::sync::Arc;

pub struct HaveSize {
    predicate: Arc<dyn Fn(usize) -> bool + Send + Sync>,
}

impl HaveSize {
    /// Creates a new `HaveSize` constraint with a custom predicate.
    pub fn new<F>(predicate: F) -> Self
    where
        F: Fn(usize) -> bool + Send + Sync + 'static,
    {
        Self {
            predicate: Arc::new(predicate),
        }
    }

    /// Creates a `HaveSize` constraint for an exact size.
    pub fn with_exact_size(size: usize) -> Self {
        Self::new(move |s| s == size)
    }
}

impl<T> Constraint<Vec<T>> for HaveSize {
    fn validate(&self, value: &Vec<T>) -> bool {
        (self.predicate)(value.len())
    }

    fn generate_exception(&self, description: String) -> ConstraintError {
        ConstraintError::new(move || description.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use proptest::prelude::*;

    mod when_creating_with_a_size {
        use super::*;

        proptest! {
            #[test]
            fn should_validate_collections_with_the_specified_size(size: usize, collection: Vec<u8>) {
                let constraint = HaveSize::with_exact_size(size);
                let result = constraint.validate(&collection);
                expect!(result).to(be_equal_to(collection.len() == size));
            }

            #[test]
            fn should_generate_an_exception_with_the_specified_description(size: usize, description: String) {
                let constraint = HaveSize::with_exact_size(size);
    
                let exception = <HaveSize as Constraint<Vec<u8>>>::generate_exception(&constraint, description.clone());
    
                expect!(exception.message()).to(be_equal_to(description));
            }
        }
    }
    
    mod when_creating_with_a_predicate {
        use super::*;

        proptest! {
            #[test]
            fn should_validate_collections_with_the_specified_predicate(
                collection in proptest::collection::vec(any::<u8>(), 1..100)
            ) {
                prop_assume!(!collection.is_empty());

                let constraint = HaveSize::new(|size| size > 0);
                let result = constraint.validate(&collection);
    
                expect!(result).to(be_true());
            }

            #[test]
            fn should_generate_an_exception_with_the_specified_description(
                collection in proptest::collection::vec(any::<u8>(), 1..100),
            ) {
                prop_assume!(collection.len() > 5);

                let constraint = HaveSize::new(|size| size <= 5);
                let result = constraint.validate(&collection);
    
                expect!(result).to(be_false());
            }
        }
    }
}
