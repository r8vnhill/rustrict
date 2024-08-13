/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */

/// Custom error type for representing constraint-related errors.
///
/// This struct is designed to be used as a base for creating specific constraint-related errors
/// in your application. It allows you to provide a lazy message that will be computed only if
/// and when the error is constructed, providing flexibility in error message generation.
///
/// # Usage
///
/// You can create specific constraint-related errors by creating a new struct that contains
/// `ConstraintError` and providing a `lazy_message` closure that computes the error message.
/// For example:
///
/// ```rust
/// struct MyCustomConstraintError {
///     field: String,
/// }
///
/// impl MyCustomConstraintError {
///     fn new(field: String) -> Self {
///         MyCustomConstraintError { field }
///     }
///
///     fn to_error(&self) -> ConstraintError {
///         ConstraintError::new(move || {
///             format!("Constraint violation in field '{}': Custom error message.", self.field)
///         })
///     }
/// }
/// ```
///
/// # Fields
///
/// * `lazy_message` - A closure that computes the error message when the error is constructed.
pub(crate) struct ConstraintError {
    lazy_message: Box<dyn Fn() -> String>,
}

impl ConstraintError {
    /// Creates a new `ConstraintError` with the specified `lazy_message`.
    fn new<F>(lazy_message: F) -> Self
    where
    // 'static is required because the closure may be stored in the struct and used later.
        F: Fn() -> String + 'static,
    {
        ConstraintError {
            lazy_message: Box::new(lazy_message),
        }
    }

    /// Returns the computed error message.
    fn message(&self) -> String {
        (self.lazy_message)()
    }
}

impl std::fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::fmt::Debug for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstraintError")
            .field("lazy_message", &"<closure>")
            .finish()
    }
}

impl std::error::Error for ConstraintError {}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;


    fn arb_message() -> impl Strategy<Value = String> {
        proptest::string::string_regex(".*").unwrap()
    }

    proptest! {
        #[test]
        fn message_is_computed(lazy_message in arb_message()) {
            let message = lazy_message.clone();
            let error = ConstraintError::new(move || lazy_message.clone());
            assert_eq!(error.message(), message);
        }
    }
}
