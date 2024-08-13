/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */
use crate::constraints::constraint::Constraint;
use crate::errors::constraint_error::ConstraintError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A scope for validating constraints on a string value.
///
/// `StringScope` manages the context and results of validation rules applied to a string,
/// similar to how you might use a DSL (Domain-Specific Language) in Kotlin to define validation rules.
///
/// # Fields:
/// - `message`: The validation message or label associated with the rule.
/// - `results`: A shared, thread-safe container for storing validation results.
/// - `exception_generator`: An optional closure for generating custom exceptions.
///
/// # Conceptual Differences:
/// - **Thread Safety:** Rust's `Arc<Mutex<>>` ensures thread-safe shared ownership and mutation,
///     which is more explicit than Kotlin's coroutines and thread safety mechanisms.
/// - **Lifetimes and Ownership:** Rust's strict ownership model, enforced by the borrow checker,
///     ensures that data races are impossible without needing a garbage collector, unlike Kotlin.
pub(crate) struct StringScope {
    message: String,
    results: Arc<Mutex<Vec<Result<(), ConstraintError>>>>,
    exception_generator: Option<Box<dyn Fn(String) -> ConstraintError>>,
}

impl StringScope {
    /// Creates a new `StringScope` without a custom exception generator.
    ///
    /// This constructor is used when you want to validate constraints without specifying a custom
    /// exception type, similar to a default exception in Kotlin.
    ///
    /// # Parameters:
    /// - `message`: The validation message or label.
    /// - `results`: A shared, thread-safe container for validation results.
    ///
    /// # Returns:
    /// A `StringScope` instance.
    pub(crate) fn new(
        message: String,
        results: Arc<Mutex<Vec<Result<(), ConstraintError>>>>,
    ) -> Self {
        Self {
            message,
            results,
            exception_generator: None,
        }
    }

    /// Creates a new `StringScope` with a custom exception generator.
    ///
    /// This constructor allows for more fine-grained control over the type of exception thrown when
    /// a validation rule fails, similar to custom exceptions in Kotlin.
    ///
    /// # Parameters:
    /// - `message`: The validation message or label.
    /// - `results`: A shared, thread-safe container for validation results.
    /// - `exception_generator`: A closure that generates a custom exception.
    ///
    /// # Returns:
    /// A `StringScope` instance.
    pub(crate) fn new_with_exception_generator(
        message: String,
        results: Arc<Mutex<Vec<Result<(), ConstraintError>>>>,
        exception_generator: Box<dyn Fn(String) -> ConstraintError>,
    ) -> Self {
        Self {
            message,
            results,
            exception_generator: Some(exception_generator),
        }
    }

    /// Validates that the given value satisfies or does not satisfy the specified constraint.
    ///
    /// This method abstracts the shared logic between `must` and `must_not`, reducing code duplication.
    /// The `condition` parameter determines whether the value should satisfy or not satisfy the constraint.
    ///
    /// # Parameters:
    /// - `value`: The value to validate.
    /// - `constraint`: The constraint to check against the value.
    /// - `condition`: A boolean indicating whether the constraint should be satisfied (`true`) or not (`false`).
    fn validate<T, C>(&self, value: T, constraint: C, condition: bool)
    where
        C: Constraint<T>,
    {
        let exception = || {
            self.exception_generator
                .as_ref()
                .map(|gen| gen(self.message.clone()))
                .unwrap_or_else(|| constraint.generate_exception(self.message.clone()))
        };

        let mut results = self.results.lock().unwrap();
        results.push(if constraint.validate(&value) == condition {
            Ok(())
        } else {
            Err(exception())
        });
    }

    /// Validates that the given value satisfies the specified constraint.
    ///
    /// This method is similar to applying validation rules in Kotlin's DSLs. It either records a
    /// successful validation or pushes a custom or default `ConstraintError` into the results.
    ///
    /// # Parameters:
    /// - `value`: The value to validate.
    /// - `constraint`: The constraint that the value must satisfy.
    fn must<T, C>(&self, value: T, constraint: C)
    where
        C: Constraint<T>,
    {
        self.validate(value, constraint, true);
    }

    /// Validates that the given value does not satisfy the specified constraint.
    ///
    /// This is the inverse of `must`, used to ensure that a value does not meet a certain condition.
    /// It's akin to asserting a negated condition in Kotlin.
    ///
    /// # Parameters:
    /// - `value`: The value to validate.
    /// - `constraint`: The constraint that the value must not satisfy.
    fn must_not<T, C>(&self, value: T, constraint: C)
    where
        C: Constraint<T>,
    {
        self.validate(value, constraint, false);
    }

    /// Defines a custom constraint based on a predicate.
    ///
    /// This method allows for defining inline validation rules, similar to how you might define
    /// ad-hoc checks in a Kotlin DSL.
    ///
    /// # Parameters:
    /// - `predicate`: A closure that returns `true` if the constraint is satisfied.
    fn constraint(&self, predicate: impl Fn() -> bool) {
        let message = self.message.clone(); // Clone the message to have an owned value with 'static lifetime

        let mut results = self.results.lock().unwrap();
        results.push(if predicate() {
            Ok(())
        } else {
            Err(ConstraintError::new(move || message.clone())) // Use the cloned message
        });
    }
}

impl Display for StringScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "StringScope({})", self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_can_be_created_with_a_message(message in ".*") {
            let scope = create_string_scope(message.clone());
            expect!(scope.message).to(be_equal_to(message));
        }

        #[test]
        fn test_can_be_converted_to_string(message in ".*") {
            let scope = create_string_scope(message.clone());
            expect!(scope.to_string()).to(be_equal_to(format!("StringScope({})", message)));
        }
    }

    mod when_validating_a_must_clause {
        use super::*;

        #[test]
        fn should_add_a_success_or_failure_to_the_scope() {}

        #[test]
        fn test_validates_a_failed_constraint() {
            let results = Arc::new(Mutex::new(Vec::new()));
            let scope = StringScope::new("Test".to_string(), results.clone());
            scope.must("Test", |value: &&str| *value == "Not Test");

            let results = results.lock().unwrap();
            expect!(results.clone()).to(be_equal_to(vec![Err(ConstraintError::new(|| {
                "Test".to_string()
            }))]));
        }
    }

    /// Creates a new instance of `StringScope` with a given message.
    ///
    /// This helper function simplifies the creation of a `StringScope` by initializing it with
    /// the provided message and an empty, thread-safe container for validation results. It mirrors
    /// the concept of a factory function in Kotlin, where you might use such a function to
    /// encapsulate object creation logic, particularly when additional setup is required.
    ///
    /// # Parameters:
    /// - `message`: A `String` representing the validation message or label associated with the
    ///     `StringScope`.
    ///
    /// # Returns:
    /// - A `StringScope` instance initialized with the provided message and a new, empty results'
    ///     container.
    ///
    /// # Conceptual Differences:
    /// - **Thread Safety:** In Rust, the results container is wrapped in an `Arc<Mutex<>>` to
    ///     ensure thread-safe access and mutation, which is more explicit compared to Kotlin's
    ///     concurrency mechanisms like coroutines.
    /// - **Ownership and Lifetimes:** Rust's ownership model requires careful management of data
    ///     ownership and lifetimes. In this case, the `Arc<Mutex<>>` combination ensures that the
    ///     data can be safely shared and modified across threads without violating Rust's strict
    ///     borrowing rules.
    ///
    /// # Example Usage:
    /// ```rust
    /// let scope = create_string_scope("Test message".to_string());
    /// // `scope` is now an instance of `StringScope` with the message "Test message"
    /// ```
    fn create_string_scope(message: String) -> StringScope {
        StringScope::new(message, Arc::new(Mutex::new(Vec::new())))
    }
}
