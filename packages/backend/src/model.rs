// extern crate mmap_storage;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::fs;
use uuid::Uuid;

// const STORAGE_PATH: &'static str = "notes.json";

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
    // /// Load storage
    // pub fn load_storage(storage_path: Option<String>) {
    //     match storage_path {
    //         Some(path) => {
    //             let mut storage =
    //                 mmap_storage::file::Storage::open(path).expect("To create storage");

    //             // TODO: how to fix below into and return the file contents
    //             // let mut data: HashMap<String, Note> = storage.into();
    //         }
    //         None => {
    //             let mut storage =
    //                 mmap_storage::file::Storage::open(STORAGE_PATH).expect("To create storage");
    //             // TODO: how to fix below into and return the file contents
    //             // let mut data: HashMap<String, Note> = storage.into();
    //         }
    //     }
    // }

    // /// Save the notes to file
    // pub fn save(&self, storage_path: Option<String>) {
    //     match storage_path {
    //         Some(path) => {
    //             let mut storage =
    //                 mmap_storage::file::Storage::open(path).expect("To create storage");

    //             // TODO: Figure out how to save to file with this method
    //             // storage.put_data(self.map)
    //         }
    //         None => {
    //             let mut storage =
    //                 mmap_storage::file::Storage::open(STORAGE_PATH).expect("To create storage");

    //             // TODO: Figure out how to save to file with this method
    //             // storage.put_data(self.map)
    //         }
    //     }
    // }

    /// Create an empty HashMap
    pub fn new() -> Self {
        // TODO: load from file here using above method when it works
        let hash_map: HashMap<String, Note> = HashMap::new();
        Self { map: hash_map }
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
