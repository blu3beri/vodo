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
    let note = Note::new(String::from("foo"), State::Todo);
    let note2 = Note::new(String::from("bar"), State::InProgress);
    let mut notes = Notes::default();
    notes.map.insert(String::from("hash-01"), note);
    notes.map.insert(String::from("hash-02"), note2);

    let mut terminal = VodoTerminal::setup(notes)?;

    terminal.run_app()?;

    terminal.destruct()?;

    Ok(())
}
