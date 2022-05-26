use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// State of a note
#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    /// No state
    None,

    /// Still have to work on the note
    Todo,

    /// Working on the note
    InProgress,

    /// Note has been finished
    Done,

    /// Expiry date has been reached
    Expired,
}

/// Hashmap of notes where the key is a sha-1 hash
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Notes {
    /// The actual notes
    /// TODO: how do zero-width unnamed structures work?
    pub map: HashMap<String, Note>,
}

/// A note / todo
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    /// Title of the note as displayed to the user
    pub title: String,

    /// Simple state of the note
    pub state: State,
    // /// When the expiry date hits
    // ends_at: Option<String>,
    // /// Tags to categorize and filter on by the user
    // tags: Vec<String>,
    // /// Custom description in `md` format
    // description: Option<Path>,
    // /// When the note was created
    // /// TODO: date type?
    // created_at: String,
    // /// When the note was last updated
    // /// TODO: date type?
    // updated_at: String,
    // /// What the priority level of the note is
    // priority: Option<u8>,
}

impl Note {
    /// Create a new note with a title
    pub fn new(title: String) -> Self {
        Self {
            title,
            state: State::None,
        }
    }
}
