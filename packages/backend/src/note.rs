use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::File;
use std::io::BufReader;

use crate::error::{Error, Result};

/// State of a note
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl From<State> for String {
    fn from(s: State) -> Self {
        let s = match s {
            State::None => "",
            State::Todo => "Todo",
            State::InProgress => "In progress",
            State::Done => "Done",
            State::Expired => "Expired",
        };

        Self::from(s)
    }
}

/// List of all your notes
#[derive(Serialize, Deserialize, Debug)]
pub struct Notes {
    /// The actual notes
    pub map: Vec<Note>,

    /// File path of the notes
    path: String,
}

impl Default for Notes {
    fn default() -> Self {
        let mut home = env::var("HOME").unwrap();
        home.push_str("/.config/vodo/notes");
        Self {
            map: Default::default(),
            path: home,
        }
    }
}

impl Notes {
    /// Load new lotes
    pub fn new() -> Result<Self> {
        Notes::load_storage(&Notes::default())
    }

    /// Load the notes from storage
    pub fn load_storage(&self) -> Result<Notes> {
        let file = File::open(&self.path).map_err(|_| Error::UnableToOpenFile)?;
        let reader = BufReader::new(file);
        let map: Vec<Note> = serde_json::from_reader(reader).unwrap_or_default();
        Ok(Notes {
            map,
            path: self.path.to_owned(),
        })
    }

    /// Save the notes to storage
    fn save(&self) -> Result<()> {
        let file = &File::create(&self.path)
            .map_err(|_| Box::new(Error::UnableToCreateFile) as Box<dyn std::error::Error>)?;

        serde_json::to_writer(file, &self.map).map_err(|_| Error::UnableToSaveFile.into())
    }

    /// Add a new note to storage
    pub fn put(&mut self, note: Note) -> Result<()> {
        self.map.push(note);
        self.save()
    }

    /// Delete a note from storage
    pub fn delete(&mut self, idx: usize) -> Result<()> {
        self.map.remove(idx);
        self.save()
    }

    /// Get a note, by index, from storage
    pub fn get(&self, idx: usize) -> &Note {
        &self.map[idx]
    }

    /// Update a note in storage
    pub fn update(&mut self, note: &mut Note, idx: usize) -> Result<()> {
        std::mem::swap(&mut self.map[idx], note);
        self.save()
    }
}

/// A note / todo
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
    /// When the note was created
    pub created_at: String,

    /// When the note was last updated
    pub updated_at: String,
    // /// What the priority level of the note is
    // priority: Option<u8>,
}

impl Note {
    /// Create a new note with a title
    pub fn new(title: impl Into<String>, state: State) -> Self {
        Self {
            title: title.into(),
            state,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        }
    }
}
