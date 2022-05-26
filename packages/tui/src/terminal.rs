use backend::model::{Note, Notes, State};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Error, Stdout};
use std::time::{Duration, Instant};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame, Terminal,
};

/// Terminal tui for vodo
pub struct VodoTerminal {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    app: App,
    tick_rate: Duration,
}

enum InputMode {
    Normal,
    Editing,
}

struct App {
    state: TableState,
    items: Vec<Note>,
    show_popup: bool,
    input: String,
    input_mode: InputMode,
}

impl VodoTerminal {
    /// Setup a general terminal
    pub fn setup(notes: Notes) -> Result<Self, Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);

        let app = App::new(notes.map.values().cloned().collect());

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
                    if !self.app.show_popup {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Down => self.app.next(),
                            KeyCode::Up => self.app.previous(),
                            KeyCode::Char('j') => self.app.next(),
                            KeyCode::Char('k') => self.app.previous(),
                            KeyCode::Char('d') => self.app.delete(),
                            KeyCode::Char('n') => self.app.new_note(),
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char(c) => self.app.input.push(c),
                            KeyCode::Backspace => {
                                self.app.input.pop();
                            }
                            KeyCode::Esc => self.app.reset(),
                            KeyCode::Enter => self.app.add_note(),
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let rects = Layout::default()
            .constraints([Constraint::Percentage(96), Constraint::Min(3)].as_ref())
            .split(f.size());

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);

        // --- table ---
        let header_cells = ["State", "Note"].iter().map(|h| Cell::from(*h));
        let header = Row::new(header_cells).height(1);
        let rows = app.items.iter().map(|item| {
            let cells = vec![
                Cell::from(String::from(item.state.to_owned())),
                Cell::from(item.title.to_owned()),
            ];
            Row::new(cells).height(1_u16)
        });

        let table = Table::new(rows)
            .block(Block::default().borders(Borders::ALL).title("Notes"))
            .header(header)
            .highlight_style(selected_style)
            .widths(&[Constraint::Percentage(15), Constraint::Percentage(100)]);
        f.render_stateful_widget(table, rects[0], &mut app.state);
        // -------------

        // --- commands ---
        let b = Block::default().borders(Borders::ALL).title("Commands");
        let text =
            Paragraph::new("(q) quit | (j) down | (k) up | (d) delete | (n) new note").block(b);
        f.render_widget(text, rects[1]);
        // ----------------

        // --- popup ---
        if app.show_popup {
            let block = Block::default().title("Popup").borders(Borders::ALL);
            let p = Paragraph::new(app.input.as_ref())
                .style(Style::default().fg(Color::White))
                .block(block);
            let area = centered_rect(60, 4, f.size());
            f.set_cursor(area.x + app.input.len() as u16 + 1, area.y + 1);
            f.render_widget(p, area);
        }
        // ---------
    }
}

impl App {
    pub fn new(items: Vec<Note>) -> Self {
        Self {
            state: TableState::default(),
            items,
            show_popup: false,
            input: String::default(),
            input_mode: InputMode::Normal,
        }
    }

    pub fn reset(&mut self) {
        self.show_popup = !self.show_popup;
        self.input_mode = InputMode::Normal;
        self.input = String::from("");
    }

    pub fn new_note(&mut self) {
        self.show_popup = true;
    }

    fn add_note(&mut self) {
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

    fn delete(&mut self) {
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Length(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
