//! Command line interface for `vodo`
//! Allows for basic functionality, but the TUI is gets
//! primary support, so this might lack features

#![deny(clippy::suspicious, clippy::complexity)]

use backend::model::Notes;
use std::io;
use terminal::frontend::VodoTerminal;

/// Module for terminal buildup and destruction
mod terminal;

/// Entrypoint for the TUI of `vodo`
fn main() -> Result<(), io::Error> {
    let notes = Notes::new();

    let mut terminal = VodoTerminal::setup(notes)?;

    terminal.run_app()?;

    terminal.destruct()?;

    Ok(())
}
