use backend::model::{Note, State};
use tui::widgets::TableState;

pub struct App {
    pub state: TableState,
    pub items: Vec<Note>,
    pub new_note_state: NewNoteState,
}

pub struct NewNoteState {
    pub show_new_note: bool,
    pub input: String,
}

impl App {
    pub fn new(items: Vec<Note>) -> Self {
        Self {
            state: TableState::default(),
            items,
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
        self.items.push(Note {
            state: State::None,
            title: self.new_note_state.input.to_owned(),
        });
        self.reset();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
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
                    self.items.len() - 1
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
            if self.items.get(i).is_some() {
                self.items.remove(i);
                if i == 0 {
                    self.state.select(Some(0));
                } else {
                    self.state.select(Some(i - 1));
                }
            }
        }
    }

    pub fn set_state(&mut self, state: State) {
        if let Some(i) = self.state.selected() {
            if self.items.get(i).is_some() {
                let note = &mut self.items[i];
                note.state = state;
            }
        }
    }
}