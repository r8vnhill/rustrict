/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */
use std::sync::Arc;

/// A struct representing a constraint-related error with a lazily evaluated message.
///
/// In Rust, `ConstraintError` is a custom error type that stores a closure for generating an error
/// message. The message is only computed when it's needed, providing flexibility and efficiency in
/// error handling. This concept is somewhat analogous to Kotlin's lazy properties but applied in
/// the context of error messages.
///
/// # Key Features:
/// - **Lazy Evaluation:** The error message is generated only when requested, which can be useful
///     in scenarios where the construction of the error message is expensive or depends on runtime
///     conditions.
/// - **Trait Implementations:** `ConstraintError` implements `Display`, `Debug`, `Error`,
///     and `Clone`, allowing it to be used effectively within Rust's error handling ecosystem,
///     similar to how exceptions might be used in Kotlin.
///
/// # Example:
/// ```rust
/// let error = ConstraintError::new(|| "This is a lazily evaluated error message.".to_string());
/// println!("{}", error);  // The message is evaluated and printed here.
/// ```
pub(crate) struct ConstraintError {
    lazy_message: Arc<dyn Fn() -> String>,
}

impl ConstraintError {
    /// Creates a new `ConstraintError` with a lazily evaluated message.
    ///
    /// This constructor takes a closure that generates the error message. The closure is stored in
    /// a `Box`, which is a heap-allocated pointer, allowing for dynamic dispatch and ensuring that
    /// the closure's lifetime is managed correctly.
    ///
    /// # Parameters:
    /// - `lazy_message`: A closure that returns a `String`. The closure is evaluated when the error
    ///     message is needed.
    ///
    /// # Returns:
    /// A `ConstraintError` instance containing the provided closure.
    ///
    /// # Example:
    /// ```rust
    /// let error = ConstraintError::new(|| "This is a custom error message.".to_string());
    /// ```
    pub(crate) fn new<F>(lazy_message: F) -> Self
    where
        F: Fn() -> String + 'static,
    {
        ConstraintError {
            lazy_message: Arc::new(lazy_message),
        }
    }

    /// Returns the computed error message.
    ///
    /// This method evaluates the stored closure to produce the error message. It's similar
    /// to calling a `toString()` method on an exception in Kotlin, except that the message
    /// is generated lazily.
    ///
    /// # Returns:
    /// A `String` containing the error message.
    ///
    /// # Example:
    /// ```rust
    /// let error = ConstraintError::new(|| "Delayed message".to_string());
    /// assert_eq!(error.message(), "Delayed message");
    /// ```
    pub(crate) fn message(&self) -> String {
        (self.lazy_message)()
    }
}

impl std::fmt::Display for ConstraintError {
    /// Formats the error message for display purposes.
    ///
    /// This method implements the `Display` trait, which is used to generate a user-friendly
    /// string representation of the `ConstraintError`. It is akin to overriding the `toString()`
    /// method in Kotlin for custom exceptions.
    ///
    /// # Parameters:
    /// - `f`: A mutable reference to a `Formatter`, which handles the formatting.
    ///
    /// # Returns:
    /// A `Result` indicating success or failure of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::fmt::Debug for ConstraintError {
    /// Provides a debug representation of the `ConstraintError`.
    ///
    /// This method implements the `Debug` trait, which is used for debugging output.
    /// The closure is represented as `"<closure>"` in the debug output, since closures
    /// cannot be easily displayed.
    ///
    /// # Parameters:
    /// - `f`: A mutable reference to a `Formatter`, which handles the formatting.
    ///
    /// # Returns:
    /// A `Result` indicating success or failure of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstraintError")
            .field("lazy_message", &"<closure>")
            .finish()
    }
}

impl std::error::Error for ConstraintError {
    // This implements Rust's standard `Error` trait, allowing `ConstraintError`
    // to be used seamlessly with Rust's error handling mechanisms, similar to
    // how custom exceptions are used in Kotlin.
}

impl Clone for ConstraintError {
    /// Creates a clone of the `ConstraintError`.
    ///
    /// This method implements the `Clone` trait, which is required to create
    /// copies of `ConstraintError` instances. It duplicates the closure stored
    /// in `lazy_message`, ensuring that each clone has its own copy of the error
    /// message generator.
    ///
    /// # Returns:
    /// A new `ConstraintError` instance that is a clone of the original.
    ///
    /// # Example:
    /// ```rust
    /// let original = ConstraintError::new(|| "Original message".to_string());
    /// let clone = original.clone();
    /// assert_eq!(original.message(), clone.message());
    /// ```
    fn clone(&self) -> Self {
        ConstraintError {
            lazy_message: self.lazy_message.clone(),
        }
    }
}

impl PartialEq for ConstraintError {
    /// Implements equality comparison for `ConstraintError`.
    ///
    /// This implementation allows you to compare two `ConstraintError` instances using the `==`
    /// operator, which is similar to overriding the `equals()` method in Kotlin. The equality is
    /// determined by comparing the error messages generated by the `message()` method of each
    /// `ConstraintError`.
    ///
    /// # Parameters:
    /// - `other`: Another instance of `ConstraintError` to compare with.
    ///
    /// # Returns:
    /// - `true` if the error messages of both `ConstraintError` instances are equal, `false`
    ///     otherwise.
    ///
    /// # Conceptual Differences:
    /// - **Equality in Rust vs. Kotlin**: In Rust, implementing the `PartialEq` trait allows you to
    ///     define custom equality logic for your types, similar to Kotlin's `equals()` method.
    ///     However, Rust’s strict type system and ownership model ensure that equality comparisons
    ///     are both explicit and safe.
    ///
    /// # Example Usage:
    /// ```rust
    /// let error1 = ConstraintError::new(|| "Error message".to_string());
    /// let error2 = ConstraintError::new(|| "Error message".to_string());
    ///
    /// assert_eq!(error1, error2); // true, since the messages are the same
    /// ```
    ///
    /// In this example, two `ConstraintError` instances are considered equal because their error
    /// messages are identical.
    fn eq(&self, other: &Self) -> bool {
        self.message() == other.message()
    }
}

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
