//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

use super::{Area, Laser};

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Cannon {
    pub left: f64,
    bottom: f64,
    color: Color,
}

impl Cannon {
    ///
    pub fn new_normal() -> Self {
        Self {
            left: points::CANNON_INITIAL_X,
            bottom: points::CANNON_INITIAL_Y,
            color: Color::Green,
        }
    }

    ///
    pub fn new_life(index: u8) -> Self {
        let x_offset = (index - 1) as f64 * (points::CANNON_WIDTH + 4.0);

        Self {
            left: points::CANNON_INITIAL_X + x_offset,
            bottom: points::CANNON_INITIAL_Y,
            color: Color::Green,
        }
    }

    ///
    pub fn move_left(&mut self) {
        if self.left > 1.0 {
            self.left -= points::CANNON_MOVE;
        }
    }

    ///
    pub fn move_right(&mut self) {
        if self.left < (points::GAME_WIDTH - points::CANNON_WIDTH) {
            self.left += points::CANNON_MOVE;
        }
    }

    ///
    pub fn reset(&mut self) {
        self.left = points::CANNON_INITIAL_X;
        self.bottom = points::CANNON_INITIAL_Y;
    }

    ///
    pub fn collides_with_laser(&self, laser: &Laser) -> bool {
        self.area().overlaps(laser.area())
    }

    ///
    pub fn area(&self) -> Area {
        Area::new(
            self.left,
            self.bottom,
            self.left + points::CANNON_WIDTH,
            self.bottom + points::CANNON_HEIGHT,
        )
    }
}

impl Shape for Cannon {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in &points::CANNON {
            let x = x + self.left;
            let y = y + self.bottom;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
