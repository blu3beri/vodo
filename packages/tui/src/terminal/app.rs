use backend::model::{Note, State};
use tui::widgets::TableState;

pub struct App {
    pub state: TableState,
    pub items: Vec<Note>,
    pub show_popup: bool,
    pub input: String,
}

impl App {
    pub fn new(items: Vec<Note>) -> Self {
        Self {
            state: TableState::default(),
            items,
            show_popup: false,
            input: String::default(),
        }
    }

    pub fn reset(&mut self) {
        self.show_popup = !self.show_popup;
        self.input = String::from("");
    }

    pub fn new_note(&mut self) {
        self.show_popup = true;
    }

    pub fn add_note(&mut self) {
        self.items.push(Note {
            state: State::None,
            title: self.input.to_owned(),
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
}

