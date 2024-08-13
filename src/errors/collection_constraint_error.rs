/*
 * Copyright (c) 2024, Ignacio Slater M.
 * 2-Clause BSD License.
 */
use std::fmt;
use std::sync::Arc;

/// A struct representing an exception related to collection constraints.
///
/// `CollectionConstraintError` is similar to a custom exception class in Kotlin, designed to handle
/// errors specific to collection constraints. This struct stores a closure that generates an error
/// message lazily, meaning the message is only computed when needed. This approach is similar to
/// passing a lambda in Kotlin to delay the evaluation of the error message until it's required.
///
/// # Fields:
/// - `lazy_message`: An `Arc<dyn Fn() -> String + Send + Sync>` that holds a closure for generating
///     the error message.
///   - **Arc**: Stands for "Atomic Reference Counting," which is a thread-safe reference-counting
///     pointer. It ensures that multiple parts of your program can share ownership of the closure
///     safely across threads, akin to Kotlin's `synchronized` or `AtomicReference`.
///   - **`Fn() -> String`**: The closure type that, when invoked, returns a `String` containing the
///     error message. This is similar to a lambda function in Kotlin that takes no parameters and
///     returns a `String`.
///   - **`Send + Sync`**: These traits ensure that the closure can be safely transferred and
///     accessed across threads, similar to making sure a lambda in Kotlin is thread-safe.
///
/// # Conceptual Differences:
/// - **Thread Safety and Concurrency**: In Rust, thread safety is more explicit than in Kotlin. The
///     use of `Arc` ensures that the closure can be shared across threads without data races,
///     enforced by Rust's type system. In contrast, Kotlin handles concurrency with coroutines and
///     higher-level abstractions, often without the need for explicit thread management.
/// - **Ownership and Lifetimes**: Rust's ownership model, which is enforced by the borrow checker,
///     requires that all data be owned by a specific part of your program, or shared using
///     constructs like `Arc`. This contrasts with Kotlin's garbage-collected memory management,
///     where objects can be passed around more freely.
///
/// # Example Usage:
/// ```rust
/// let exception = CollectionConstraintError {
///     lazy_message: Arc::new(|| "Collection constraint violated".to_string()),
/// };
///
/// println!("{}", exception.lazy_message()); // Prints: "Collection constraint violated"
/// ```
///
/// This example demonstrates how to create a `CollectionConstraintError` with a lazily evaluated message.
/// The message is generated only when the closure is invoked.
pub struct CollectionConstraintError {
    lazy_message: Arc<dyn Fn() -> String + Send + Sync>,
}

impl CollectionConstraintError {
    /// Creates a new `CollectionConstraintError` with a lazily evaluated error message.
    ///
    /// This function serves as the constructor for `CollectionConstraintError`, similar to how you
    /// might define a constructor in a Kotlin class. It takes a closure that generates the error
    /// message, allowing the message to be computed only when needed. This is akin to passing a
    /// lambda in Kotlin to defer the evaluation of an error message until it's required.
    ///
    /// # Parameters:
    /// - `lazy_message`: A closure of type `F` that returns a `String` containing the error
    ///     message.
    ///   - **`Fn() -> String`**: The closure type that, when invoked, produces a `String`. In
    ///     Kotlin, this is similar to a lambda expression that returns a `String`.
    ///   - **`'static`**: Indicates that the closure does not borrow any data with a non-static
    ///     lifetime, making it safe to store and use later, akin to a Kotlin lambda with no
    ///     external dependencies.
    ///   - **`Send + Sync`**: These traits ensure that the closure can be safely shared and
    ///     executed across threads, similar to ensuring thread safety in Kotlin using mechanisms
    ///     like `synchronized` or coroutines.
    ///
    /// # Returns:
    /// - A new instance of `CollectionConstraintError` containing the provided closure.
    ///
    /// # Example Usage:
    /// ```rust
    /// let error = CollectionConstraintError::new(|| "Collection constraint violated".to_string());
    /// ```
    ///
    /// In this example, the error message is not generated immediately. Instead, the closure is
    /// stored, and the message is generated only when needed, similar to Kotlin's lazy evaluation
    /// using lambdas.
    pub fn new<F>(lazy_message: F) -> Self
    where
        F: Fn() -> String + 'static + Send + Sync,
    {
        CollectionConstraintError {
            lazy_message: Arc::new(lazy_message),
        }
    }

    /// Retrieves the error message by evaluating the stored closure.
    ///
    /// This method calls the closure stored in the `lazy_message` field to produce the error
    /// message. This is analogous to invoking a method on a custom exception in Kotlin to get its
    /// error message, but with the added benefit that the message is computed lazily.
    ///
    /// # Returns:
    /// - A `String` containing the error message generated by the closure.
    ///
    /// # Example Usage:
    /// ```rust
    /// let error = CollectionConstraintError::new(|| "Collection constraint violated".to_string());
    /// println!("{}", error.message()); // Prints: "Collection constraint violated"
    /// ```
    ///
    /// In this example, the `message` method is used to retrieve the error message, demonstrating
    /// how the message is evaluated only when accessed, similar to calling `getMessage()` on an
    /// exception in Kotlin.
    pub fn message(&self) -> String {
        (self.lazy_message)()
    }
}

impl fmt::Display for CollectionConstraintError {
    /// Formats the `CollectionConstraintError` for display purposes.
    ///
    /// This method is an implementation of the `fmt::Display` trait, which is similar to overriding
    /// the `toString()` method in Kotlin. It defines how the `CollectionConstraintError` should be
    /// presented when converted to a string, typically for user-facing messages or logging.
    ///
    /// The method uses the `message()` function to retrieve the lazily evaluated error message and
    /// writes it to the provided formatter. This is analogous to Kotlin's `toString()` method in a
    /// custom exception class that returns a string representation of the error message.
    ///
    /// # Parameters:
    /// - `f`: A mutable reference to a `fmt::Formatter`, which handles the actual formatting of the
    ///     string.
    ///
    /// # Returns:
    /// - A `fmt::Result` indicating whether the formatting operation was successful or if an error
    ///     occurred.
    ///
    /// # Example Usage:
    /// ```rust
    /// let error = CollectionConstraintError::new(|| "Collection constraint violated".to_string());
    /// println!("{}", error); // Automatically uses the Display implementation
    /// ```
    ///
    /// In this example, the `error` is printed using the `Display` implementation, which outputs
    /// the lazily evaluated error message.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl fmt::Debug for CollectionConstraintError {
    /// Formats the `CollectionConstraintError` for debugging purposes.
    ///
    /// This method is an implementation of the `fmt::Debug` trait, which is used to create a
    /// detailed and developer-friendly representation of the `CollectionConstraintError` struct. In
    /// Kotlin, this is somewhat similar to overriding the `toString()` method with additional
    /// debugging information.
    ///
    /// The method creates a debug representation of the struct, showing the type and a placeholder
    /// for the `lazy_message` field, since the closure itself cannot be directly displayed. This
    /// provides insight into the structure of the error without evaluating the closure, which could
    /// have side effects or performance implications.
    ///
    /// # Parameters:
    /// - `f`: A mutable reference to a `fmt::Formatter`, which handles the actual formatting of the
    ///     debug string.
    ///
    /// # Returns:
    /// - A `fmt::Result` indicating whether the formatting operation was successful or if an error
    ///     occurred.
    ///
    /// # Example Usage:
    /// ```rust
    /// let error = CollectionConstraintError::new(|| "Collection constraint violated".to_string());
    /// println!("{:?}", error); // Automatically uses the Debug implementation
    /// ```
    ///
    /// In this example, the `error` is printed using the `Debug` implementation, showing the
    /// structure of the `CollectionConstraintError` for debugging purposes.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CollectionConstraintError")
            .field("lazy_message", &"<closure>")
            .finish()
    }
}
