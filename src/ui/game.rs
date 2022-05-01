//!

use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::widgets::canvas::Canvas;
use tui::{
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Row, Table, Tabs},
};

use crate::app::App;
use crate::assets;

use super::util;

pub fn draw_game_screen<B: Backend>(f: &mut Frame<B>, app: &App) {
    let area = util::app_area(f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(super::ROW_HEIGHT),
            Constraint::Length(super::ROW_HEIGHT),
            Constraint::Length(super::GAME_HEIGHT),
            Constraint::Length(super::DOUBLE_ROW_HEIGHT),
            Constraint::Min(0),
        ])
        .split(area);

    draw_tabs(f, chunks[0]);
    draw_score(f, chunks[1], app.score, app.hiscore);
    draw_game(f, chunks[2], app);
    draw_lives(f, chunks[3], app.lives);

    if app.show_help {
        draw_help_popup(f, area);
    }

    if app.paused {
        draw_paused_popup(f, area);
    }
}

fn draw_tabs<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let titles = [
        super::HELP_TAB_TEXT,
        super::PAUSE_TAB_TEXT,
        super::QUIT_TAB_TEXT,
    ]
    .iter()
    .cloned()
    .map(Spans::from)
    .collect();

    let tabs_widget = Tabs::new(titles).block(
        Block::default()
            .borders(Borders::ALL)
            .title(super::APP_TITLE),
    );

    f.render_widget(tabs_widget, area);
}

fn draw_score<B: Backend>(f: &mut Frame<B>, area: Rect, score: u32, hiscore: u32) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let score_widget = Paragraph::new(Span::raw(score.to_string())).block(
        Block::default()
            .borders(Borders::ALL)
            .title(super::SCORE_TITLE),
    );

    let hiscore_widget = Paragraph::new(Span::raw(hiscore.to_string())).block(
        Block::default()
            .borders(Borders::ALL)
            .title(super::HISCORE_TITLE),
    );

    f.render_widget(score_widget, chunks[0]);
    f.render_widget(hiscore_widget, chunks[1]);
}

fn draw_game<B: Backend>(f: &mut Frame<B>, area: Rect, app: &App) {
    let (constraints, index) = util::center(area.width, super::GAME_WIDTH);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let game = Canvas::default()
        .block(Block::default().borders(Borders::ALL))
        .x_bounds([
            0.0,
            ((chunks[index].width - (super::BORDER_WIDTH * 2)) * super::HORIZONTAL_DOTS_PER_CHAR)
                as f64,
        ])
        .y_bounds([
            0.0,
            ((chunks[index].height - (super::BORDER_WIDTH * 2)) * super::VERTICAL_DOTS_PER_CHAR)
                as f64,
        ])
        .paint(|ctx| {
            ctx.draw(&app.cannon);
            ctx.draw(&app.bunkers);
            ctx.draw(&app.grid);
            ctx.draw(&app.mystery_ship);

            for laser in app.cannon_lasers.iter().chain(app.invader_lasers.iter()) {
                ctx.draw(laser);
            }
        });

    f.render_widget(game, chunks[index]);
}

fn draw_lives<B: Backend>(f: &mut Frame<B>, area: Rect, lives: u8) {
    let canvas_width = (area.width - (super::BORDER_WIDTH * 2)) * super::HORIZONTAL_DOTS_PER_CHAR;
    let canvas_height = (area.height - (super::BORDER_WIDTH * 2)) * super::VERTICAL_DOTS_PER_CHAR;

    let lives_widget = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Lives"))
        .x_bounds([0.0, canvas_width as f64])
        .y_bounds([0.0, canvas_height as f64])
        .paint(|ctx| {
            for i in 1..=lives {
                ctx.draw(&assets::Cannon::new_life(i));
            }
        });

    f.render_widget(lives_widget, area);
}

pub fn draw_help_popup<B: Backend>(f: &mut Frame<B>, area: Rect) {
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

    util::draw_popup(f, area, popup, super::HELP_WIDTH, super::HELP_HEIGHT);
}

fn draw_paused_popup<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let popup = Paragraph::new(Span::raw("Press P to unpause"))
        .block(Block::default().borders(Borders::ALL).title("Paused"));

    util::draw_popup(f, area, popup, super::PAUSE_WIDTH, super::PAUSE_HEIGHT);
}
