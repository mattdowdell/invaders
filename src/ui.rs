//!

use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::widgets::canvas::Canvas;
use tui::{
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
};

use crate::app::App;
use crate::assets;
use crate::points;

const APP_TITLE: &str = "Space Invaders";
const SCORE_TITLE: &str = "Score";
const HISCORE_TITLE: &str = "Hiscore";

const HELP_TAB: &str = "[H]elp";
const PAUSE_TAB: &str = "[P]ause";
const QUIT_TAB: &str = "[Q]uit";

const VERTICAL_DOTS_PER_CHAR: u16 = 4;
const HORIZONTAL_DOTS_PER_CHAR: u16 = 2;

const BORDER_WIDTH: u16 = 1;
const GAME_WIDTH: u16 = 102;

///
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    // TODO: output help message if size is insufficient, otherwise we panic

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(35),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(size);

    draw_tabs(f, chunks[0]);
    draw_score(f, chunks[1]);
    draw_game(f, chunks[2], app);
    draw_lives(f, chunks[3], app.lives);
}

// TODO: change tabs based on whether help is active
fn draw_tabs<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let titles = [HELP_TAB, PAUSE_TAB, QUIT_TAB]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();

    let tabs_widget =
        Tabs::new(titles).block(Block::default().borders(Borders::ALL).title(APP_TITLE));

    f.render_widget(tabs_widget, area);
}

// TODO: take scores as input
fn draw_score<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let score: u32 = 100;
    let hiscore: u32 = 1000;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let score_widget = Paragraph::new(Span::raw(score.to_string()))
        .block(Block::default().borders(Borders::ALL).title(SCORE_TITLE));

    let hiscore_widget = Paragraph::new(Span::raw(hiscore.to_string()))
        .block(Block::default().borders(Borders::ALL).title(HISCORE_TITLE));

    f.render_widget(score_widget, chunks[0]);
    f.render_widget(hiscore_widget, chunks[1]);
}

fn draw_game<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let (constraints, index) = game_constraints(area.width);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let game = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .x_bounds([0.0, ((chunks[index].width - (BORDER_WIDTH * 2)) * HORIZONTAL_DOTS_PER_CHAR) as f64])
        .y_bounds([0.0, ((chunks[index].height - (BORDER_WIDTH * 2)) * VERTICAL_DOTS_PER_CHAR) as f64])
        .paint(|ctx| {
            ctx.draw(&app.shooter);

            ctx.draw(&assets::Shield::new(20, 14));
            ctx.draw(&assets::Shield::new(65, 14));
            ctx.draw(&assets::Shield::new(110, 14));
            ctx.draw(&assets::Shield::new(155, 14));

            ctx.draw(&app.grid);
            ctx.draw(&app.mothership);

            for shot in app.shots.iter() {
                ctx.draw(shot);
            }
        });

    f.render_widget(game, chunks[index]);
}

fn game_constraints(width: u16) -> (Vec<Constraint>, usize) {
    let start_width = (width - GAME_WIDTH) / 2;

    if width < GAME_WIDTH || start_width == 0 {
        (vec![Constraint::Length(GAME_WIDTH), Constraint::Min(0)], 0)
    } else {
        (
            vec![
                Constraint::Length(start_width),
                Constraint::Length(GAME_WIDTH),
                Constraint::Min(0),
            ],
            1,
        )
    }
}

// TODO: take lives as input
fn draw_lives<B: Backend>(f: &mut Frame<B>, area: Rect, lives: u8) {
    let canvas_width = (area.width - (BORDER_WIDTH * 2)) * HORIZONTAL_DOTS_PER_CHAR;
    let canvas_height = (area.height - (BORDER_WIDTH * 2)) * VERTICAL_DOTS_PER_CHAR;

    let lives_widget = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Lives"))
        .x_bounds([0.0, canvas_width as f64])
        .y_bounds([0.0, canvas_height as f64])
        .paint(|ctx| {
            for i in 1..=lives {
                // TODO: move 12 to constants
                let x_offset = (i - 1) as f64 * (points::SHOOTER_SMALL_WIDTH + 4.0);
                ctx.draw(&assets::Shooter::new_small(x_offset as f64));
            }
        });

    f.render_widget(lives_widget, area);
}
