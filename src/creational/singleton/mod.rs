//! The single source of truth
//!
//! The Singleton pattern ensures that a class has only one instance and provides a global point of access to it.
//! This is useful when exactly one object is needed to coordinate actions across the system.
//!
//! In this module, we demonstrate the Singleton pattern with two examples:
//! - `singleton_config_manager`: Manages configuration settings as a single instance.
//! - `singleton_logger`: Provides a single instance for logging purposes.
//!
//! Both examples ensure thread safety and demonstrate how to implement the Singleton pattern in Rust.

/// The Global Config Manager
///
/// Load settings from a file or environment variables and provide both read-only and mutable access to the rest of the application. Using a singleton ensures that configuration is loaded only once and is accessible globally.
mod config_manager;
/// The Global Logger
///
///  Provide a global point of access to a logging instance, ensuring that all parts of the application use the same logger configuration and output destination.
mod logger;
