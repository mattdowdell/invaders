//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

const BUNKER_SPACING: f64 =
    (points::GAME_WIDTH - (2.0 * points::BUNKER_OFFSET_X) - (4.0 * points::BUNKER_WIDTH)) / 3.0;

///
#[derive(Clone, Debug, PartialEq)]
pub struct Bunkers {
    bunkers: Vec<Bunker>,
}

impl Bunkers {
    ///
    pub fn new() -> Self {
        let mut bunkers = Vec::new();

        for i in 0..4 {
            let x_offset =
                points::BUNKER_OFFSET_X + (i as f64 * (points::BUNKER_WIDTH + BUNKER_SPACING));
            let bunker = Bunker::new(x_offset, points::BUNKER_INITIAL_Y);

            bunkers.push(bunker)
        }

        Self { bunkers }
    }
}

impl Shape for Bunkers {
    fn draw(&self, painter: &mut Painter) {
        for bunker in self.bunkers.iter() {
            bunker.draw(painter);
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct Bunker {
    origin_x: f64,
    origin_y: f64,
    color: Color,
    data: Vec<(f64, f64)>,
}

impl Bunker {
    ///
    pub fn new(origin_x: f64, origin_y: f64) -> Self {
        Self {
            origin_x,
            origin_y,
            color: Color::Green,
            data: points::BUNKER.into(),
        }
    }
}

impl Shape for Bunker {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.data.iter() {
            let x = x + self.origin_x;
            let y = y + self.origin_y;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
