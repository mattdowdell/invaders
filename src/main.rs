//!

mod app;
mod assets;
mod points;
mod ui;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new();
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.modifiers {
                    KeyModifiers::CONTROL => {
                        if key.code == KeyCode::Char('c') {
                            app.on_ctrl_c();
                        }
                    }
                    KeyModifiers::NONE => match key.code {
                        KeyCode::Char('h') => app.on_h(),
                        KeyCode::Char('p') => app.on_p(),
                        KeyCode::Char('q') => app.on_q(),
                        KeyCode::Char(' ') => app.on_space(),
                        KeyCode::Left => app.on_left(),
                        KeyCode::Right => app.on_right(),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        if app.should_quit {
            break;
        }

        if last_tick.elapsed() >= tick_rate {
            if !app.is_paused() {
                app.on_tick();
            }

            last_tick = Instant::now();
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    Ok(())
}
