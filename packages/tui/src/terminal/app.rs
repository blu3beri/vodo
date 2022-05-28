use backend::note::{Note, Notes, State};
use chrono::Utc;
use tui::widgets::TableState;

pub struct App {
    pub state: TableState,
    pub notes: Notes,
    pub new_note_state: NewNoteState,
}

pub struct NewNoteState {
    pub show_new_note: bool,
    pub input: String,
}

impl App {
    pub fn new(items: Notes) -> Self {
        Self {
            state: TableState::default(),
            notes: items,
            new_note_state: NewNoteState {
                show_new_note: false,
                input: String::default(),
            },
        }
    }

    pub fn reset(&mut self) {
        self.new_note_state.show_new_note = !self.new_note_state.show_new_note;
        self.new_note_state.input = String::from("");
    }

    pub fn new_note(&mut self) {
        self.new_note_state.show_new_note = true;
    }

    pub fn add_note(&mut self) {
        let note = Note::new(self.new_note_state.input.to_owned(), State::Todo);
        self.notes.put(note).unwrap();
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
