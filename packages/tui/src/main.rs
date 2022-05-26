//! Command line interface for `vodo`
//! Allows for basic functionality, but the TUI is gets
//! primary support, so this might lack features

#![deny(
    clippy::missing_docs_in_private_items,
    clippy::suspicious,
    clippy::complexity,
    clippy::cargo
)]

use backend::model::Note;

/// Entrypoint for the TUI of `vodo`
fn main() {
    let note = Note::new(String::from("foo"));
    println!("{:?}", note);
}
