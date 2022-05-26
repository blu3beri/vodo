//! Command line interface for `vodo`
//! Allows for basic functionality, but the TUI is gets
//! primary support, so this might lack features

#![deny(clippy::suspicious, clippy::complexity)]

use backend::model::{Note, Notes};
use std::io;
use terminal::VodoTerminal;

/// Module for terminal buildup and destruction
mod terminal;

/// Entrypoint for the TUI of `vodo`
fn main() -> Result<(), io::Error> {
    let note = Note::new(String::from("foo"));
    let note2 = Note::new(String::from("bar"));
    let mut notes = Notes::new();
    notes.append(note);
    notes.append(note2);

    // TOOD: notes.save(?path) to save to file

    let mut terminal = VodoTerminal::setup(notes)?;

    terminal.run_app()?;

    terminal.destruct()?;

    Ok(())
}
