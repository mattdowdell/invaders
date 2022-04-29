//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cannon {
    pub origin_x: f64,
    origin_y: f64,
    cannon_type: CannonType,
    color: Color,
}

impl Cannon {
    ///
    pub fn new_normal() -> Self {
        Self {
            origin_x: points::CANNON_INITIAL_X,
            origin_y: points::CANNON_INITIAL_Y,
            cannon_type: CannonType::Normal,
            color: Color::Green,
        }
    }

    ///
    pub fn new_small(x_offset: f64) -> Self {
        Self {
            origin_x: points::CANNON_INITIAL_X + x_offset,
            origin_y: points::CANNON_INITIAL_Y,
            cannon_type: CannonType::Small,
            color: Color::Green,
        }
    }

    ///
    pub fn move_left(&mut self) {
        if self.origin_x > 1.0 {
            self.origin_x -= points::CANNON_MOVE;
        }
    }

    ///
    pub fn move_right(&mut self) {
        if self.origin_x < (points::GAME_WIDTH - points::CANNON_WIDTH) {
            self.origin_x += points::CANNON_MOVE;
        }
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        match self.cannon_type {
            CannonType::Normal => &points::CANNON,
            CannonType::Small => &points::CANNON_SMALL,
        }
    }
}

impl Shape for Cannon {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.data() {
            let x = x + self.origin_x;
            let y = y + self.origin_y;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

///
#[derive(Copy, Clone, Debug, PartialEq)]
enum CannonType {
    Normal,
    Small,
}
