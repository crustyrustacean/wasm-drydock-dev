// src/hooks.rs

// module declarations
pub mod use_version;

// re-exports
pub use use_version::*;

// enum type to represent loading state
#[derive(Clone, PartialEq)]
pub enum FetchState<T: Clone + PartialEq> {
    Loading,
    Success(T),
    Error(String),
}