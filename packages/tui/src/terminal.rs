use backend::model::{Note, Notes};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Error, Stdout};
use std::time::{Duration, Instant};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

/// Terminal tui for vodo
pub struct VodoTerminal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    app: App,
    tick_rate: Duration,
}

struct App {
    state: TableState,
    items: Vec<Note>,
}

impl VodoTerminal {
    /// Setup a general terminal
    pub fn setup(notes: Notes) -> Result<Self, Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);

        let app = App {
            state: TableState::default(),
            items: notes.map.values().cloned().collect(),
        };

        Ok(Self {
            terminal: Terminal::new(backend)?,
            app,
            tick_rate: Duration::from_millis(250),
        })
    }

    /// Destructure the terminal for closing
    pub fn destruct(&mut self) -> Result<(), Error> {
        // restore terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        Ok(())
    }

    pub fn run_app(&mut self) -> io::Result<()> {
        let last_tick = Instant::now();
        loop {
            self.terminal.draw(|f| VodoTerminal::ui(f, &mut self.app))?;

            let timeout = self
                .tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down => self.app.next(),
                        KeyCode::Char('j') => self.app.next(),
                        KeyCode::Char('k') => self.app.previous(),
                        KeyCode::Char('d') => self.app.delete(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let header_cells = ["State", "Note"].iter().map(|h| Cell::from(*h));
        let header = Row::new(header_cells).height(1);
        let rows = app.items.iter().map(|item| {
            let cells = vec![
                Cell::from(String::from(item.state.to_owned())),
                Cell::from(item.title.to_owned()),
            ];
            Row::new(cells).height(1_u16)
        });
        let t = Table::new(rows)
            .block(Block::default().borders(Borders::ALL).title("Notes"))
            .header(header)
            .highlight_style(selected_style)
            .widths(&[Constraint::Percentage(15), Constraint::Percentage(100)]);
        f.render_stateful_widget(t, rects[0], &mut app.state);
    }
}

impl App {
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

    fn delete(&mut self) {
        if let Some(i) = self.state.selected() {
            if self.items.get(i).is_none() {
                return;
            }
            self.items.remove(i);
            if i == 0 {
                self.state.select(Some(0));
            } else {
                self.state.select(Some(i - 1));
            }
        }
    }
}
