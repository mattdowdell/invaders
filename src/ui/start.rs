//!

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    terminal::Frame,
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};

use crate::assets;

use super::{game, util};

pub fn draw_start_screen<B: Backend>(f: &mut Frame<B>) {
    let area = util::app_area(f.size());

    draw_outer(f, area);

    let inner = get_inner(area);
    let logo = assets::Words::space_invaders();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(logo.height() as u16 / super::VERTICAL_DOTS_PER_CHAR),
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(super::HELP_HEIGHT),
            Constraint::Min(0),
        ])
        .split(inner);

    draw_logo(f, chunks[1], logo);
    draw_start_text(f, chunks[3]);
    game::draw_help_popup(f, chunks[5]);
}

fn draw_outer<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let outer = Block::default()
        .borders(Borders::ALL)
        .title(super::APP_TITLE);
    f.render_widget(outer, area);
}

fn get_inner(outer: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(super::BORDER_WIDTH),
            Constraint::Length(super::APP_HEIGHT - (2 * super::BORDER_WIDTH)),
            Constraint::Min(0),
        ])
        .split(outer);

    let area = chunks[1];
    let (constraints, index) =
        util::center(area.width, super::GAME_WIDTH - 2 * super::BORDER_WIDTH);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    chunks[index]
}

fn draw_logo<B: Backend>(f: &mut Frame<B>, area: Rect, logo: assets::Words) {
    let (constraints, index) = util::center(
        area.width,
        logo.width() as u16 / super::HORIZONTAL_DOTS_PER_CHAR,
    );
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let widget = Canvas::default()
        .block(Block::default())
        .x_bounds([0.0, logo.width()])
        .y_bounds([0.0, logo.height()])
        .paint(|ctx| {
            ctx.draw(&logo);
        });

    f.render_widget(widget, chunks[index]);
}

fn draw_start_text<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let (constraints, index) = util::center(area.width, 20);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    let widget = Paragraph::new(Span::raw("Press SPACE to start"));
    f.render_widget(widget, chunks[index]);
}
