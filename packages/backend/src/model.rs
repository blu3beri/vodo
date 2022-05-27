use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use uuid::Uuid;

use crate::error::Error;

/// The standard storage path
const STORAGE_PATH: &str = "notes.json";

/// State of a note
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

impl Notes {
    /// Load storage
    pub fn load_storage(storage_path: Option<&'static str>) -> Result<Notes, Error> {
        let path = storage_path.unwrap_or(STORAGE_PATH);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let file_data: Notes = serde_json::from_reader(reader).unwrap_or_default();

        Ok(file_data)
    }

    /// Save the notes to file
    pub fn save(&self, storage_path: Option<&'static str>) {
        let path = storage_path.unwrap_or(STORAGE_PATH);
        serde_json::to_writer(&File::create(path).unwrap(), &self).unwrap()
    }

    /// Create an empty HashMap
    pub fn new() -> Self {
        // TODO: load from file here using above method when it works
        Notes::load_storage(None).unwrap()
    }

    /// Append a note to the hashList of notes
    pub fn append(&mut self, note: Note) {
        let id = Uuid::new_v4();
        self.map.insert(id.to_string(), note);
    }

    /// Remove a note from the hashList by its ID
    pub fn remove_by_id(&mut self, id: String) {
        self.map.retain(|k, _| *k != id)
    }

    /// Remove a note from the hashList by the value of the note itself
    pub fn remove_by_note(&mut self, note: Note) {
        self.map.retain(|_, v| *v != note)
    }
}

/// A note / todo
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
