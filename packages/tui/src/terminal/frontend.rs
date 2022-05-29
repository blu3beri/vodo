use crate::terminal::app::NoteInputState;

use super::app::App;
use backend::note::Notes;
use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Error, Result, Stdout};
use std::time::{Duration, Instant};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame, Terminal,
};

/// Terminal tui for vodo
pub struct VodoTerminal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    app: App,
    tick_rate: Duration,
}

impl VodoTerminal {
    /// Setup a general terminal
    pub fn setup(notes: Notes) -> std::result::Result<Self, Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);

        let app = App::new(notes);

        Ok(Self {
            terminal: Terminal::new(backend)?,
            app,
            tick_rate: Duration::from_millis(250),
        })
    }

    /// Destructure the terminal for closing
    pub fn destruct(&mut self) -> std::result::Result<(), Error> {
        // restore terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        self.terminal.show_cursor()
    }

    pub fn run_app(&mut self) -> Result<()> {
        let last_tick = Instant::now();
        loop {
            self.terminal.draw(|f| VodoTerminal::ui(f, &mut self.app))?;

            let timeout = self
                .tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = read()? {
                    if !self.app.note_state.show_input_note {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Down => self.app.next(),
                            KeyCode::Up => self.app.previous(),
                            KeyCode::Char('j') => self.app.next(),
                            KeyCode::Char('k') => self.app.previous(),
                            KeyCode::Char('d') => self.app.delete(),
                            KeyCode::Char('n') => self.app.show_input(NoteInputState::New),
                            KeyCode::Char('s') => self.app.update_state(),
                            KeyCode::Char('e') => self.app.show_input(NoteInputState::Editting),
                            KeyCode::Char('p') => self.app.prioritize(),
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char(c) => match self.app.note_state.input_state {
                                NoteInputState::New | NoteInputState::Editting => {
                                    self.app.note_state.input.push(c)
                                }
                                NoteInputState::Category => self.app.note_state.category.push(c),
                                _ => panic!("Unknown state"),
                            },
                            KeyCode::Backspace => {
                                self.app.note_state.input.pop();
                            }
                            KeyCode::Esc => self.app.reset(),
                            KeyCode::Enter => match self.app.note_state.input_state {
                                NoteInputState::New => self.app.add_note(),
                                NoteInputState::Editting => self.app.edit_note(),
                                NoteInputState::Category => self.app.set_category(),
                                _ => panic!("Unknown note state"),
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let rects = Layout::default()
            .constraints([Constraint::Min(10), Constraint::Length(3)].as_ref())
            .split(f.size());

        let selected_style = match app.note_state.should_delete {
            true => Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Red),
            false => Style::default().add_modifier(Modifier::REVERSED),
        };

        // --- table ---
        let header_cells = ["State", "Category", "Note"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().add_modifier(Modifier::BOLD)));
        let header = Row::new(header_cells).height(1);
        let rows = app.notes.map.iter().map(|item| {
            let cells = vec![
                Cell::from(String::from(item.state.to_owned())),
                Cell::from(item.category.to_owned()),
                Cell::from(item.title.to_owned()),
            ];
            Row::new(cells)
        });

        let table = Table::new(rows)
            .block(Block::default().borders(Borders::ALL).title("Notes"))
            .header(header)
            .highlight_style(selected_style)
            .widths(&[
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(100),
            ]);
        f.render_stateful_widget(table, rects[0], &mut app.state);
        // -------------

        // --- commands ---
        if !app.note_state.show_input_note {
            let b = Block::default().borders(Borders::ALL).title("Commands");
            let text = Paragraph::new(
                "(q) quit | (j) down | (k) up | (d) delete | (n) new note | (e) edit note | (p) prioritize",
            )
            .block(b);
            f.render_widget(text, rects[1]);
        }
        // ----------------

        // --- new note ---
        // TODO: one giant match
        if app.note_state.show_input_note {
            let (title, text, len) = match app.note_state.input_state {
                NoteInputState::Editting => (
                    "Edit Note",
                    app.note_state.input.as_ref(),
                    app.note_state.input.len(),
                ),
                NoteInputState::New => (
                    "New Note",
                    app.note_state.input.as_ref(),
                    app.note_state.input.len(),
                ),
                NoteInputState::Category => (
                    "Category",
                    app.note_state.category.as_ref(),
                    app.note_state.category.len(),
                ),
                _ => panic!("Unknown state"),
            };
            let block = Block::default().title(title).borders(Borders::ALL);
            let p = Paragraph::new(text)
                .style(Style::default().fg(Color::White))
                .block(block);
            f.set_cursor(rects[1].x + len as u16 + 1, rects[1].y + 1);
            f.render_widget(p, rects[1]);
        }
        // ----------------
    }
}
