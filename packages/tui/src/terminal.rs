use backend::model::Notes;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Error, Stdout},
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

/// Terminal tui for vodo
pub struct VodoTerminal {
    /// Main tui terminal that will be passed around
    terminal: Terminal<CrosstermBackend<Stdout>>,
    app: App,
    tick_rate: Duration,
}

struct App {
    items: StatefulList<(String, usize)>,
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
            items: StatefulList::with_items(
                notes
                    .map
                    .values()
                    .map(|n| (n.title.to_owned(), 0usize))
                    .collect(),
            ),
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
                        KeyCode::Left => self.app.items.unselect(),
                        KeyCode::Down => self.app.items.next(),
                        KeyCode::Char('j') => self.app.items.next(),
                        KeyCode::Char('k') => self.app.items.previous(),
                        KeyCode::Char('d') => self.app.items.delete(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        // Iterate through all elements in the `items` app and append some debug text to it.
        let items: Vec<ListItem> = app
            .items
            .items
            .iter()
            .map(|i| {
                let lines = vec![Spans::from(&i.0[..])];
                ListItem::new(lines).style(Style::default())
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Notes"))
            .highlight_style(
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(items, chunks[0], &mut app.items.state);
    }
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn delete(&mut self) {
        if let Some(i) = self.state.selected() {
            self.items.remove(i);
            if i == 0 {
                self.state.select(Some(0));
            } else {
                self.state.select(Some(i - 1));
            }
        }
    }

    fn next(&mut self) {
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

    fn previous(&mut self) {
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

    fn unselect(&mut self) {
        self.state.select(None);
    }
}
