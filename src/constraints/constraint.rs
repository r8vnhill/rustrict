/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */

use crate::errors::constraint_error::ConstraintError;

pub trait Constraint<T> {
    /// The validation function that checks if the value meets the constraint criteria.
    fn validate(&self, value: &T) -> bool;

    /// Generates a `ConstraintError` with the provided description.
    ///
    /// ## Usage:
    /// Use this function to generate a `ConstraintError` when a constraint is violated. This helps
    /// Rustrict handle the exception and provide detailed information about the constraint
    /// violation.
    ///
    /// - `description`: A string describing the reason for the exception.
    /// - Returns: A `ConstraintError` containing the provided description.
    fn generate_exception(&self, description: String) -> ConstraintError;
    
    fn generate_error_message(&self, message: &str) -> String {
        format!("{}: {}", message, self.generate_exception(message.to_string()))
    }
}

impl<T, F> Constraint<T> for F
where
    F: Fn(&T) -> bool,
{
    fn validate(&self, value: &T) -> bool {
        self(value)
    }

    fn generate_exception(&self, description: String) -> ConstraintError {
        ConstraintError::new(move || description.clone())
    }
}
