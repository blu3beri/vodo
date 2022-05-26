use crossterm::{
    event::DisableMouseCapture,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use std::io::{self, Error, Stdout};
use tui::{backend::CrosstermBackend, Terminal};

/// Terminal tui for vodo
pub struct VodoTerminal {
    /// Main tui terminal that will be passed around
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl VodoTerminal {
    /// Setup a general terminal
    pub fn setup() -> Result<Self, Error> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);

        Ok(Self {
            terminal: Terminal::new(backend)?,
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
}
