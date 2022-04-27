//!

use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::widgets::canvas::Canvas;
use tui::{
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, Tabs, Widget},
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
const GAME_WIDTH: u16 = 100 + (2 * BORDER_WIDTH);
const GAME_HEIGHT: u16 = 33 + (2 * BORDER_WIDTH);
const ROW_HEIGHT: u16 = 1 + (2 * BORDER_WIDTH);

const APP_HEIGHT: u16 = (3 * ROW_HEIGHT) + GAME_HEIGHT;

const HELP_WIDTH: u16 = 25 + (2 * BORDER_WIDTH);
const HELP_HEIGHT: u16 = 6 + (2 * BORDER_WIDTH);

const PAUSE_WIDTH: u16 = 18 + (2 * BORDER_WIDTH);
const PAUSE_HEIGHT: u16 = 1 + (2 * BORDER_WIDTH);

///
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    // TODO: output help message if size is insufficient, otherwise we panic

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(ROW_HEIGHT),
            Constraint::Length(ROW_HEIGHT),
            Constraint::Length(GAME_HEIGHT),
            Constraint::Length(ROW_HEIGHT),
            Constraint::Min(0),
        ])
        .split(size);

    draw_tabs(f, chunks[0]);
    draw_score(f, chunks[1], app.score, app.hiscore);
    draw_game(f, chunks[2], app);
    draw_lives(f, chunks[3], app.lives);

    if app.show_help {
        draw_help_popup(f, size);
    }

    if app.paused {
        draw_paused_popup(f, size);
    }
}

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

fn draw_score<B: Backend>(f: &mut Frame<B>, area: Rect, score: u32, hiscore: u32) {
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
        .x_bounds([
            0.0,
            ((chunks[index].width - (BORDER_WIDTH * 2)) * HORIZONTAL_DOTS_PER_CHAR) as f64,
        ])
        .y_bounds([
            0.0,
            ((chunks[index].height - (BORDER_WIDTH * 2)) * VERTICAL_DOTS_PER_CHAR) as f64,
        ])
        .paint(|ctx| {
            ctx.draw(&app.cannon);

            let shield_buffer =
                (points::GAME_WIDTH - (2.0 * 20.0) - (4.0 * points::BUNKER_WIDTH)) / 3.0;

            for i in 0..4 {
                let shield_offset_x = 20.0 + (i as f64 * (points::BUNKER_WIDTH + shield_buffer));
                let shield = assets::Bunker::new(shield_offset_x, 14.0);
                ctx.draw(&shield);
            }

            ctx.draw(&app.grid);
            ctx.draw(&app.mystery_ship);

            for laser in app.lasers.iter() {
                ctx.draw(laser);
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

fn draw_lives<B: Backend>(f: &mut Frame<B>, area: Rect, lives: u8) {
    let canvas_width = (area.width - (BORDER_WIDTH * 2)) * HORIZONTAL_DOTS_PER_CHAR;
    let canvas_height = (area.height - (BORDER_WIDTH * 2)) * VERTICAL_DOTS_PER_CHAR;

    let lives_widget = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Lives"))
        .x_bounds([0.0, canvas_width as f64])
        .y_bounds([0.0, canvas_height as f64])
        .paint(|ctx| {
            for i in 1..=lives {
                let x_offset = (i - 1) as f64 * (points::CANNON_SMALL_WIDTH + 4.0);
                ctx.draw(&assets::Cannon::new_small(x_offset as f64));
            }
        });

    f.render_widget(lives_widget, area);
}

fn draw_help_popup<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let popup = Table::new(vec![
        Row::new(vec!["  H", "Open/close help"]),
        Row::new(vec!["  ←", "Move cannon left"]),
        Row::new(vec!["  →", "Move cannon right"]),
        Row::new(vec!["SPACE", "Fire cannon"]),
        Row::new(vec!["  P", "Pause/unpause"]),
        Row::new(vec!["  Q", "Quit"]),
    ])
    .widths(&[Constraint::Length(5), Constraint::Length(17)])
    .column_spacing(3)
    .block(Block::default().borders(Borders::ALL).title("Help"));

    draw_popup(f, area, popup, HELP_WIDTH, HELP_HEIGHT);
}

fn draw_paused_popup<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let popup = Paragraph::new(Span::raw("Press P to unpause"))
        .block(Block::default().borders(Borders::ALL).title("Paused"));

    draw_popup(f, area, popup, PAUSE_WIDTH, PAUSE_HEIGHT);
}

fn draw_popup<B: Backend, W: Widget>(
    f: &mut Frame<B>,
    area: Rect,
    widget: W,
    width: u16,
    height: u16,
) {
    let y_offset = (APP_HEIGHT - height) / 2;
    let x_offset = (area.width - width) / 2;

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(y_offset),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(x_offset),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(vertical_chunks[1]);

    f.render_widget(Clear, horizontal_chunks[1]);
    f.render_widget(widget, horizontal_chunks[1]);
}
