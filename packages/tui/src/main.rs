//! Command line interface for `vodo`
//! Allows for basic functionality, but the TUI is gets
//! primary support, so this might lack features

#![deny(
    clippy::missing_docs_in_private_items,
    clippy::suspicious,
    clippy::complexity,
    clippy::cargo
)]

use std::io;
use terminal::VodoTerminal;

/// Module for terminal buildup and destruction
mod terminal;

/// Entrypoint for the TUI of `vodo`
fn main() -> Result<(), io::Error> {
    let mut terminal = VodoTerminal::setup()?;

    terminal.destruct()?;

    Ok(())
}
