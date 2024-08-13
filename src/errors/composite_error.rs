/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */
use std::error::Error;
use std::sync::Arc;
use expectest::core::{Join, Matcher};

/// Represents an error that aggregates multiple individual errors into a single composite error.
///
/// This struct is used to collect multiple errors that may occur during operations, particularly in
/// concurrent or multithreaded contexts. Each underlying error is stored in a thread-safe,
/// reference-counted pointer (`Arc`), which ensures that the errors can be safely shared across
/// threads.
///
/// ## Components:
/// - `Vec<Arc<dyn std::error::Error + Send + Sync>>`:
///   - `Vec`: A dynamic array, similar to Kotlin's `List`.
///   - `Arc`: A thread-safe reference-counting pointer, similar to Kotlin's `AtomicReference`,
///     which allows shared ownership of data across threads.
///   - `dyn std::error::Error + Send + Sync`: A trait object representing any error type
///     that implements Rust's `Error` trait (akin to Kotlin's `Throwable`), with additional
///     guarantees that the error can be sent between threads (`Send`) and safely shared
///     among them (`Sync`).
///
/// ## Usage:
/// Use `CompositeError` when you need to aggregate multiple errors into a single error
/// structure, especially when dealing with multiple potentially failing operations
/// in a concurrent environment.
///
/// ## Example:
/// ```rust
/// let error1 = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "Error 1"));
/// let error2 = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "Error 2"));
/// let composite = CompositeError::new(vec![error1, error2]);
/// ```
///
/// In this example, `CompositeError` holds two `std::io::Error` instances, allowing them
/// to be treated as a single error entity.
#[derive(Debug)]
struct CompositeError {
    errors: Vec<Arc<dyn Error + Send + Sync>>,
}

impl CompositeError {
    /// Creates a new `CompositeError` instance with the provided list of errors.
    ///
    /// This constructor takes a vector of errors wrapped in `Arc`, a thread-safe, reference-counted
    /// pointer. Each error must implement the `Error`, `Send`, and `Sync` traits, ensuring that
    /// they are safely shareable across threads.
    ///
    /// # Panics
    /// This function will panic if the provided list of errors is empty, as a `CompositeError`
    /// must contain at least one error. This is similar to requiring a non-empty collection
    /// when constructing an exception in Kotlin.
    ///
    /// # Example
    /// ```rust
    /// let error1 = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "Error 1"));
    /// let error2 = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "Error 2"));
    /// let composite = CompositeError::new(vec![error1, error2]);
    /// ```
    ///
    /// In this example, `CompositeError` is constructed with two errors, aggregated into
    /// a single error entity.
    pub fn new(errors: Vec<Arc<dyn Error + Send + Sync>>) -> Self {
        assert!(!errors.is_empty(), "The list of errors cannot be empty");

        CompositeError { errors }
    }

    /// Returns a reference to the list of errors contained within this `CompositeError`.
    ///
    /// This method provides access to the vector of errors that were aggregated when the
    /// `CompositeError` was created. The errors are stored in an `Arc`, ensuring they can be
    /// shared and accessed safely across threads.
    ///
    /// # Example
    /// ```rust
    /// let composite = CompositeError::new(vec![Arc::new(std::io::Error::new(
    ///     std::io::ErrorKind::Other, "Error 1"))]);
    /// for error in composite.errors() {
    ///     println!("{}", error);
    /// }
    /// ```
    ///
    /// This example demonstrates how to iterate over and access the individual errors within
    /// a `CompositeError`.
    pub fn errors(&self) -> &Vec<Arc<dyn std::error::Error + Send + Sync>> {
        &self.errors
    }
}

impl std::fmt::Display for CompositeError {
    /// Implements the `Display` trait for `CompositeError` to provide a user-friendly string
    /// representation of the aggregated errors.
    ///
    /// This implementation allows `CompositeError` to be formatted as a string, similar to how you
    /// might override `toString()` in Kotlin to customize the string representation of an object.
    /// The formatted string will describe either a single error or a collection of errors in a
    /// human-readable format.
    ///
    /// # Formatting Behavior:
    /// - If `CompositeError` contains a single error, the output will be formatted as:
    ///   `"An error occurred -- [ErrorType] ErrorMessage"`.
    /// - If `CompositeError` contains multiple errors, the output will be formatted as:
    ///   `"Multiple errors occurred -- { [ErrorType] ErrorMessage },\n{ [ErrorType] ErrorMessage }, ..."`.
    ///
    /// # Example:
    /// ```rust
    /// let error1 = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "Error 1"));
    /// let error2 = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "Error 2"));
    /// let composite = CompositeError::new(vec![error1, error2]);
    /// println!("{}", composite);
    /// ```
    ///
    /// In this example, `CompositeError` will be displayed as a string listing each error,
    /// helping to quickly identify all underlying issues.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = if self.errors.len() == 1 {
            format!(
                "An error occurred -- [{}] {}",
                std::any::type_name::<dyn Error>(),
                self.errors[0].to_string()
            )
        } else {
            let error_messages = self
                .errors
                .iter()
                .map(|e: &Arc<dyn Error + Send + Sync>|
                    format!(
                        "{{ [{}] {} }}",
                        std::any::type_name::<dyn Error>(),
                        e.to_string()
                    )
                )
                .collect::<Vec<String>>()
                .join(",\n");

            format!("Multiple errors occurred -- {}", error_messages)
        };

        write!(f, "{}", message)
    }
}

/// Implements the `Error` trait for `CompositeError`.
///
/// This implementation allows `CompositeError` to be treated as a standard error type in Rust's
/// error handling system. By implementing the `Error` trait, `CompositeError` can be used with
/// Rust's `Result` type, the `?` operator, and other error handling mechanisms, enabling it
/// to be returned, propagated, or otherwise managed like any other error.
///
/// The `Error` trait in Rust is analogous to Kotlin's `Throwable` interface, which forms the
/// basis for exception handling. This implementation is crucial for integrating `CompositeError`
/// into Rust's ecosystem of error management.
///
/// # Example:
/// ```rust
/// let error1 = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "Error 1"));
/// let error2 = Arc::new(std::io::Error::new(std::io::ErrorKind::Other, "Error 2"));
/// let composite = CompositeError::new(vec![error1, error2]);
///
/// // `CompositeError` can now be returned from functions that return `Result<_, Box<dyn Error>>`.
/// ```
///
/// This implementation is typically empty, as `Error` provides default methods that are sufficient
/// for many custom error types. However, it is a necessary step to ensure that `CompositeError`
/// can fully participate in Rust's error-handling framework.
impl std::error::Error for CompositeError {}

#[cfg(test)]
mod tests {
    use core::panic;
    use expectest::prelude::*;
    use proptest::collection::vec;
    use proptest::prelude::*;
    use std::error::Error;
    use std::panic::catch_unwind;
    use std::sync::Arc;
    use super::*;

    /// Tests that a `CompositeError` can be successfully created with multiple errors and that
    /// it correctly stores and represents each error.
    ///
    /// This test uses property-based testing, similar to Kotlin's property-based testing frameworks
    /// like Kotest. The `proptest!` macro generates random data (strings of length 1 to 50) to
    /// simulate a variety of error messages, ensuring that the `CompositeError` behaves correctly
    /// under different conditions.
    ///
    /// # Test Flow:
    ///
    /// 1. **Generate Random Error Messages:**
    ///    - The `proptest!` macro generates a vector of random alphanumeric strings (`messages`)
    ///      with lengths between 2 and 50. These strings simulate error messages.
    ///
    /// 2. **Create `std::io::Error` Instances:**
    ///    - Each message is mapped to a `std::io::Error`, a common error type in Rust, and
    ///      wrapped in an `Arc`. `Arc` (Atomic Reference Counting) is similar to Kotlin's
    ///      `AtomicReference`, allowing multiple threads to safely share ownership of an error.
    ///    - The resulting vector, `exceptions`, contains `Arc<dyn Error + Send + Sync>` objects,
    ///      which ensures that the errors are thread-safe.
    ///
    /// 3. **Construct a `CompositeError`:**
    ///    - A `CompositeError` instance is created using the `exceptions` vector. This is similar
    ///      to aggregating multiple exceptions in Kotlin using a `CompositeException`.
    ///
    /// 4. **Assertions:**
    ///    - The test checks that the `CompositeError` contains the correct number of errors
    ///      (`composite_errors.len()` should match `exceptions.len()`).
    ///    - It then iterates through each error, comparing the string representation of the
    ///      error stored in `CompositeError` with the expected string representation from `exceptions`.
    ///
    /// # Kotlin Analogies:
    /// - **Property-based Testing:** The `proptest!` macro in Rust is similar to property-based testing
    ///   in Kotlin, where you generate inputs to test edge cases and validate the properties of your
    ///   code under a wide range of conditions.
    /// - **Error Handling:** `CompositeError` aggregates multiple errors, similar to how you might
    ///   handle multiple exceptions in Kotlin with a custom `Exception` class or a `CompositeException`.
    ///
    /// # Example:
    /// This test is automatically run with different inputs generated by `proptest!`, ensuring
    /// that `CompositeError` works correctly for a wide range of potential error messages.
    ///
    /// ```rust
    /// // This is an example of what happens inside the proptest! block:
    /// use std::error::Error;
    /// use std::sync::Arc;
    /// let messages = vec!["Error 1".to_string(), "Error 2".to_string()];
    /// let exceptions: Vec<Arc<dyn Error + Send + Sync>> = messages
    ///     .into_iter()
    ///     .map(|msg| Arc::new(std::io::Error::new(std::io::ErrorKind::Other, msg)) as Arc<dyn Error + Send + Sync>)
    ///     .collect();
    /// let composite = CompositeError::new(exceptions.clone());
    ///
    /// // Check that the errors are stored correctly.
    /// assert_eq!(composite.errors().len(), exceptions.len());
    /// for (composite_error, expected_error) in composite.errors().iter().zip(exceptions.iter()) {
    ///     assert!(composite_error.to_string().contains(&expected_error.to_string()));
    /// }
    /// ```
    #[test]
    fn composite_error_can_be_created_with_multiple_errors() {
        proptest!(|(messages in vec("[a-zA-Z0-9]{1,50}", 2..50))| {
        let exceptions: Vec<Arc<dyn Error + Send + Sync>> = messages
            .into_iter()
            .map(|msg|
                Arc::new(std::io::Error::new(std::io::ErrorKind::Other, msg))
                    as Arc<dyn Error + Send + Sync>
            ).collect();

        let composite = CompositeError::new(exceptions.clone());

        // Manually compare the errors by their string representations
        let composite_errors = composite.errors();
        expect!(composite_errors.len()).to(be_equal_to(exceptions.len()));
        for (
            composite_error, expected_error) in
                composite_errors.iter().zip(exceptions.iter()
        ) {
            expect!(composite_error.to_string()).to(contain(expected_error.to_string()));
        }
    });
    }

    #[test]
    fn composite_error_can_be_created_with_a_single_error() {
        proptest!(
            |(message in "[a-zA-Z0-9]{1,50}")| {
                let exception: Arc<dyn Error + Send + Sync> =
                    Arc::new(std::io::Error::new(std::io::ErrorKind::Other, message.clone()));

                let composite = CompositeError::new(vec![exception.clone()]);

                // Convert `CompositeError` to a string and check if it contains the expected error message
                expect!(composite.to_string()).to(contain(exception.to_string()));

                // Compare the string representations of the errors
                let composite_error_strings: Vec<String> = composite.errors()
                    .iter()
                    .map(|e| e.to_string())
                    .collect();
                let expected_error_strings: Vec<String> = vec![exception.to_string()];

                expect!(composite_error_strings).to(be_equal_to(expected_error_strings));
            }
        );
    }

    /// A custom matcher to check if a panic of a specific type occurs.
    fn panic_with_type<F, T>(f: F)
    where
        F: FnOnce() + panic::UnwindSafe,
        T: 'static + std::fmt::Debug,
    {
        let result = catch_unwind(f);
        match result {
            Ok(_) => expect!(false).to(be_true()), // Expecting a panic, but none occurred
            Err(err) => expect!(err.is::<T>()).to(be_true()), // Expecting a panic of type T
        };
    }

    #[test]
    fn composite_error_should_throw_when_empty() {
        panic_with_type::<_, &'static str>(|| {
            CompositeError::new(vec![]).errors();
        });
    }

}

/// A struct representing a value to be checked for containment within another value.
///
/// In Rust, `Contains<T>` is a generic struct that stores a value of type `T`. This value
/// can then be used to check if it is contained within another value, such as a `String`.
/// This concept is similar to creating a custom matcher in Kotlin for checking if a string
/// contains a substring.
///
/// # Type Parameters:
/// - `T`: The type of the value that you want to check for containment. For example, if you
///   want to check if a `String` contains another `String`, `T` would be `String`.
///
/// # Example:
/// ```rust
/// let contains_hello = contain("Hello".to_string());
/// assert!(contains_hello.matches(&"Hello, world!".to_string()));
/// ```
///
/// In this example, `contains_hello` is a `Contains<String>` instance that checks if a given
/// `String` contains the word "Hello".
struct Contains<T> {
    value: T,
}

/// A function to create a `Contains<T>` instance.
///
/// This function serves as a constructor for `Contains<T>`, making it easy to create
/// instances of `Contains` without having to explicitly write out the struct syntax.
/// It's similar to factory functions you might define in Kotlin.
///
/// # Parameters:
/// - `value`: The value that you want to check for containment. This value is stored
///   inside the `Contains` struct.
///
/// # Returns:
/// - A `Contains<T>` instance that holds the provided value.
///
/// # Example:
/// ```rust
/// let contains_rust = contain("Rust".to_string());
/// assert!(contains_rust.matches(&"Learning Rust is fun!".to_string()));
/// ```
///
/// This example shows how to create a `Contains<String>` instance using the `contain`
/// function and then use it to check if a `String` contains the word "Rust".
fn contain<T>(value: T) -> Contains<T> {
    Contains { value }
}

/// Implementation of the `Matcher` trait for `Contains<String>`.
///
/// This implementation allows `Contains<String>` to be used as a matcher in Rust's
/// testing framework. The `Matcher` trait defines methods for checking if a condition
/// is met (`matches`) and for generating failure messages (`failure_message`).
/// This is conceptually similar to writing custom assertions or matchers in Kotlin.
///
/// # Methods:
///
/// - `failure_message(&self, join: Join, actual: &String) -> String`:
///   Generates a detailed failure message if the `actual` string does not contain
///   the expected value. This method is useful for providing clear feedback in tests
///   when an assertion fails. It works similarly to custom failure messages you might
///   write in Kotlin testing frameworks.
///
/// - `matches(&self, actual: &String) -> bool`:
///   Checks if the `actual` string contains the expected value stored in `self.value`.
///   This method returns `true` if the value is found, and `false` otherwise. It's
///   analogous to using `contains` or similar methods in Kotlin to check for substrings.
///
/// # Example:
/// ```rust
/// let matcher = contain("Rust".to_string());
/// assert!(matcher.matches(&"Learning Rust is fun!".to_string()));
/// ```
///
/// In this example, `matches` will return `true` because the string "Rust" is contained
/// within the given `actual` string.
impl Matcher<String, String> for Contains<String> {
    fn failure_message(&self, join: Join, actual: &String) -> String {
        if join.is_assertion() {
            format!(
                "expected {} contain <{:?}>, at index <{:?}>. Got <{:?}>",
                join, self.value, actual.find(&self.value), actual
            )
        } else {
            format!("expected {} contain <{:?}>", join, self.value)
        }
    }

    fn matches(&self, actual: &String) -> bool {
        actual.contains(&self.value)
    }
}
