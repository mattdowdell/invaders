//!

mod assets;
mod points;

use std::time::{Duration, Instant};
use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::widgets::canvas::Canvas;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(52),
                    Constraint::Length(4),
                    Constraint::Min(0),
                ])
                .split(size);

            let titles = ["[H]elp", "[P]ause", "[Q]uit"]
                .iter()
                .cloned()
                .map(Spans::from)
                .collect();
            let tabs = Tabs::new(titles).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Space Invaders"),
            );
            f.render_widget(tabs, chunks[0]);

            let score_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            let score = Paragraph::new(Span::raw("100"))
                .block(Block::default().borders(Borders::ALL).title("Score"));
            f.render_widget(score, score_chunks[0]);

            let hiscore = Paragraph::new(Span::raw("1000"))
                .block(Block::default().borders(Borders::ALL).title("Hiscore"));
            f.render_widget(hiscore, score_chunks[1]);

            let (game_constraints, game_index) = if chunks[2].width > 102 {
                if chunks[2].width > 103 {
                    (
                        vec![
                            Constraint::Length((chunks[2].width - 102) / 2),
                            Constraint::Length(102),
                            Constraint::Min(0),
                        ],
                        1,
                    )
                } else {
                    (vec![Constraint::Length(102), Constraint::Min(0)], 0)
                }
            } else {
                (vec![Constraint::Length(102)], 0)
            };

            let game_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(game_constraints)
                .split(chunks[2]);

            let game = Canvas::default()
                .block(Block::default().borders(Borders::ALL))
                .x_bounds([0.0, ((game_chunks[game_index].width - 2) * 2) as f64])
                .y_bounds([0.0, ((game_chunks[game_index].height - 2) * 4) as f64])
                .paint(|ctx| {
                    ctx.draw(&assets::Shooter::new(30, 0));

                    ctx.draw(&assets::Shield::new(20, 16));
                    ctx.draw(&assets::Shield::new(65, 16));
                    ctx.draw(&assets::Shield::new(110, 16));
                    ctx.draw(&assets::Shield::new(155, 16));

                    ctx.draw(&assets::Grid::new(0, 138));

                    ctx.draw(&assets::Mothership::new(50, 192));
                });
            f.render_widget(game, game_chunks[game_index]);

            let lives = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Lives"))
                .x_bounds([0.0, ((chunks[3].width - 2) * 2) as f64])
                .y_bounds([0.0, ((chunks[3].height - 2) * 4) as f64])
                .paint(|ctx| {
                    ctx.draw(&assets::Shooter::new(2, 0));
                    ctx.draw(&assets::Shooter::new(20, 0));
                    ctx.draw(&assets::Shooter::new(38, 0));
                });
            f.render_widget(lives, chunks[3]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        if c == 'q' {
                            break;
                        }
                    }
                    // KeyCode::Left => app.on_left(),
                    // KeyCode::Right => app.on_right(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            // app.on_tick();
            last_tick = Instant::now();
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    terminal.show_cursor()?;

    Ok(())
}
