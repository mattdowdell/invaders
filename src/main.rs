//!

mod app;
mod assets;
mod points;
mod ui;

use std::env;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{backend::CrosstermBackend, Terminal};

///
#[derive(Parser, Debug)]
struct Args {
    /// The interval in ticks between appearances of the mystery ship.
    #[clap(long, default_value_t = 200)]
    mystery_ship_interval: u16,

    /// The maximum number of cannon lasers that can be present.
    #[clap(long, default_value_t = 1)]
    max_cannon_lasers: u8,

    /// The maximum number of invader lasers that can be present.
    #[clap(long, default_value_t = 3)]
    max_invader_lasers: u8,

    /// The number of milliseconds per tick.
    #[clap(long, default_value_t = 50)]
    tick_length: u64,

    /// The level to start at.
    #[clap(long, default_value_t = 0)]
    start_level: u8,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new(
        args.mystery_ship_interval,
        args.max_cannon_lasers,
        args.start_level,
    );
    let tick_rate = Duration::from_millis(args.tick_length);
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
            if app.playing() {
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

//
fn xdg_cache_home() -> PathBuf {
    match env::var_os("XDG_CACHE_HOME") {
        Some(cache_home) => PathBuf::from(cache_home),
        None => {
            let mut home_dir = home::home_dir().expect("Unable to get home directory");
            home_dir.push(".cache");

            home_dir
        }
    }
}
