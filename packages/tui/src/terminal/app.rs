use backend::note::{Note, Notes, State};
use chrono::Utc;
use tui::widgets::TableState;

pub struct App {
    pub state: TableState,
    pub notes: Notes,
    pub note_state: NoteState,
}

pub enum NoteInputState {
    None,
    New,
    Editting,
    Category,
}

pub struct NoteState {
    pub input_state: NoteInputState,
    pub show_input_note: bool,
    pub input: String,
    pub category: String,
    pub should_delete: bool,
}

impl App {
    pub fn new(items: Notes) -> Self {
        Self {
            state: TableState::default(),
            notes: items,
            note_state: NoteState {
                input_state: NoteInputState::None,
                show_input_note: false,
                input: String::default(),
                category: String::default(),
                should_delete: false,
            },
        }
    }

    /// Prepare UI to add a note
    pub fn prepare_add_note(&mut self) {
        self.note_state.input_state = NoteInputState::New;
        self.note_state.show_input_note = true;
    }

    /// Continue to the next step
    pub fn add_note(&mut self) {
        self.prepare_set_category();
    }

    /// Prepare to set the category
    pub fn prepare_set_category(&mut self) {
        self.note_state.input_state = NoteInputState::Category;
        self.note_state.show_input_note = true;
    }

    /// add note
    pub fn set_category(&mut self) {
        let note = Note::new(
            self.note_state.input.to_owned(),
            self.note_state.category.to_owned(),
            State::Todo,
        );
        self.notes.put(note).unwrap();
        self.reset();
    }

    /// Prepare UI to edit the note
    pub fn prepare_edit_note(&mut self) {
        let idx = self.state.selected().unwrap_or_default();
        let note = self.notes.get(idx);
        if let Some(note) = note {
            self.note_state.input_state = NoteInputState::Editting;
            self.note_state.show_input_note = true;
            self.note_state.input = note.title.to_owned();
        } else {
            self.note_state.show_input_note = false;
        }
    }

    /// Edit and update the note
    pub fn edit_note(&mut self) {
        let idx = self.state.selected().unwrap_or_default();
        let note = self.notes.get(idx);
        if let Some(note) = note {
            let mut n = Note {
                title: self.note_state.input.to_owned(),
                ..note.to_owned()
            };
            self.notes.update(&mut n, idx).unwrap();
        }
        self.reset();
    }

    /// reset the state of the application
    pub fn reset(&mut self) {
        self.note_state.show_input_note = false;
        self.note_state.category = String::from("");
        self.note_state.input = String::from("");
        self.note_state.input_state = NoteInputState::None;
    }

    /// Show input according to the input mode
    pub fn show_input(&mut self, mode: NoteInputState) {
        self.note_state.show_input_note = true;
        match mode {
            NoteInputState::Editting => self.prepare_edit_note(),
            NoteInputState::New => self.prepare_add_note(),
            _ => panic!("Unknown note state"),
        };
    }

    /// Select the next note
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i as isize >= (self.notes.map.len() as isize) - 1 {
                    0
                } else {
                    (i + 1) as isize
                }
            }
            None => 0,
        };
        self.note_state.should_delete = false;
        self.state.select(Some(i as usize));
    }

    /// Select the previous note
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.notes.map.len() as isize - 1
                } else {
                    (i - 1) as isize
                }
            }
            None => 0,
        };
        self.note_state.should_delete = false;
        self.state.select(Some(i as usize));
    }

    /// Delete the selected note
    /// delete has to be called twice
    pub fn delete(&mut self) {
        if self.note_state.should_delete {
            if let Some(i) = self.state.selected() {
                if self.notes.map.get(i).is_some() {
                    self.notes.delete(i).unwrap();
                    if i == 0 {
                        self.state.select(Some(0));
                    } else {
                        self.state.select(Some(i - 1));
                    }
                    self.note_state.should_delete = false;
                }
            }
        } else {
            self.note_state.should_delete = true;
        }
    }

    /// Update the state of a todo with a loop
    pub fn update_state(&mut self) {
        if let Some(i) = self.state.selected() {
            if self.notes.map.get(i).is_some() {
                // TODO: could increment be implemented for this?
                let next_state = match self.notes.map[i].state {
                    State::None => State::Todo,
                    State::Todo => State::InProgress,
                    State::InProgress => State::Done,
                    State::Done => State::Expired,
                    State::Expired => State::None,
                };
                let mut n = Note {
                    state: next_state,
                    updated_at: Utc::now().to_rfc3339(),
                    ..self.notes.map[i].to_owned()
                };
                self.notes.update(&mut n, i).unwrap();
            }
        }
    }

    /// Change the priority of the note to be the first in the list
    pub fn prioritize(&mut self) {
        if let Some(i) = self.state.selected() {
            let note = self.notes.map.swap_remove(i);
            self.notes.map.insert(0, note);
        }
    }
}
