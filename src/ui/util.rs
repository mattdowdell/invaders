//!

use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::{
    layout::{Constraint, Direction, Layout},
    text::Span,
    widgets::{Clear, Paragraph, Widget},
};

///
pub fn app_area(area: Rect) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(super::APP_HEIGHT),
            Constraint::Min(0),
        ])
        .split(area);
    let area = chunks[0];

    let (constraints, index) = center(area.width, super::APP_WIDTH);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    chunks[index]
}

///
pub fn center(outer: u16, inner: u16) -> (Vec<Constraint>, usize) {
    let margin = (outer - inner) / 2;

    if outer < inner || margin == 0 {
        (vec![Constraint::Length(inner), Constraint::Min(0)], 0)
    } else {
        (
            vec![
                Constraint::Length(margin),
                Constraint::Length(inner),
                Constraint::Min(0),
            ],
            1,
        )
    }
}

///
pub fn draw_popup<B: Backend, W: Widget>(
    f: &mut Frame<B>,
    area: Rect,
    widget: W,
    width: u16,
    height: u16,
) {
    let (constraints, index) = center(area.height, height);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    let area = chunks[index];

    let (constraints, index) = center(area.width, width);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);
    let area = chunks[index];

    f.render_widget(Clear, area);
    f.render_widget(widget, area);
}

pub fn draw_too_small_message<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let widget = Paragraph::new(Span::raw(format!(
        "Terminal must be at least {}x{} characters, currently {}x{} characters",
        super::APP_WIDTH,
        super::APP_HEIGHT,
        area.width,
        area.height,
    )));
    f.render_widget(widget, area);
}
