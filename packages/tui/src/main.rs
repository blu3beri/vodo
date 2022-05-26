//! Command line interface for `vodo`
//! Allows for basic functionality, but the TUI is gets
//! primary support, so this might lack features

#![deny(clippy::suspicious, clippy::complexity, clippy::cargo)]

use backend::model::{Note, Notes, State};
use std::io;
use terminal::frontend::VodoTerminal;

/// Module for terminal buildup and destruction
mod terminal;

/// Entrypoint for the TUI of `vodo`
fn main() -> Result<(), io::Error> {
    let mut notes = Notes::default();

    for i in 0..100 {
        let note = Note::new(format!("{}", i), State::Todo);
        notes.map.insert(format!("{}a", i), note);
    }

    let mut terminal = VodoTerminal::setup(notes)?;

    terminal.run_app()?;

    terminal.destruct()?;

    Ok(())
}
