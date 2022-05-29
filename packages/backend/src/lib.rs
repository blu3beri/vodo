//! main backend for all the functionality of `vodo`

#![deny(
    clippy::missing_docs_in_private_items,
    clippy::suspicious,
    clippy::complexity,
    clippy::cargo
)]

/// Error module
mod error;

/// Data structure model
pub mod note;
