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
}

pub struct NoteState {
    pub input_state: NoteInputState,
    pub show_input_note: bool,
    pub input: String,
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
            },
        }
    }

    pub fn reset(&mut self) {
        self.note_state.show_input_note = !self.note_state.show_input_note;
        self.note_state.input = String::from("");
    }

    pub fn new_note(&mut self) {
        self.note_state.show_input_note = true;
        self.note_state.input_state = NoteInputState::New;
    }

    pub fn add_note(&mut self) {
        let note = Note::new(self.note_state.input.to_owned(), State::Todo);
        self.notes.put(note).unwrap();
        self.reset();
    }

    pub fn edit_note(&mut self) {
        let idx = self.state.selected().unwrap_or_default();
        self.note_state.show_input_note = true;
        self.note_state.input = self.notes.get(idx).title.to_owned();
        self.note_state.input_state = NoteInputState::Editting;
    }

    pub fn edit(&mut self) {
        let idx = self.state.selected().unwrap_or_default();
        let mut n = Note {
            title: self.note_state.input.to_owned(),
            ..self.notes.get(idx).to_owned()
        };
        self.notes.update(&mut n, idx).unwrap();
        self.reset();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.notes.map.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.notes.map.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn delete(&mut self) {
        if let Some(i) = self.state.selected() {
            if self.notes.map.get(i).is_some() {
                self.notes.delete(i).unwrap();
                if i == 0 {
                    self.state.select(Some(0));
                } else {
                    self.state.select(Some(i - 1));
                }
            }
        }
    }

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
}
