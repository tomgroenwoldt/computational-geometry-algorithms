use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::{app::App, ui::draw::draw};

const X_BOUNDS: [f64; 2] = [-200.0, 200.0];
const Y_BOUNDS: [f64; 2] = [-100.0, 100.0];

pub fn run() -> Result<(), anyhow::Error> {
    // Setup terminal.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it.
    let app = App::new("Computational geometry algorithms", X_BOUNDS, Y_BOUNDS);
    let res = run_app(&mut terminal, app);

    // Restore terminal.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err}")
    }

    Ok(())
}

fn run_app<B: Backend + std::marker::Send>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), anyhow::Error> {
    loop {
        terminal.draw(|f| draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if let Err(_e) = app.on_key(key) {
                // TODO: Add error message functionality for TUI
            }
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
