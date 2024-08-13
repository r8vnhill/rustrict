/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */
use crate::constraints::constraint::Constraint;
use crate::errors::collection_constraint_error::CollectionConstraintError;
use crate::errors::constraint_error::ConstraintError;

pub trait CollectionConstraint<T>: Constraint<Vec<T>> {
    fn generate_exception(&self, description: String) -> CollectionConstraintError {
        CollectionConstraintError::new(move || description.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn should_be_able_to_generate_an_error(description: String) {
            let description_clone = description.clone();
            let constraint =
                CollectionConstraintError::new(move || description_clone.clone());
            expect!(constraint.message()).to(be_equal_to(description));
        }
    }
}
