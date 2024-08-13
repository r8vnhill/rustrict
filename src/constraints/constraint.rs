/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */

use crate::errors::constraint_error::ConstraintError;

pub trait Constraint<T> {
    /// The validation function that checks if the value meets the constraint criteria.
    fn validator(&self, value: &T) -> bool;

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
}
