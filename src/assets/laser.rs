//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

use super::{Area, InvaderType};

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Laser {
    left: f64,
    bottom: f64,
    color: Color,
    direction: Direction,
    pub from: LaserFrom,
}

impl Laser {
    ///
    pub fn new_cannon(cannon_left: f64) -> Self {
        Self {
            left: cannon_left + points::CANNON_LASER_INITIAL_X_OFFSET,
            bottom: points::CANNON_LASER_INITIAL_Y,
            color: Color::Green,
            direction: Direction::Up,
            from: LaserFrom::Cannon,
        }
    }

    ///
    pub fn new_invader(invader_left: f64, invader_bottom: f64, invader_type: InvaderType) -> Self {
        Self {
            left: invader_left + (points::ALIEN_WIDTH / 2.0).floor(),
            bottom: invader_bottom - points::LASER_HEIGHT,
            color: invader_type.color(),
            direction: Direction::Down,
            from: LaserFrom::Invader,
        }
    }

    ///
    pub fn on_tick(&mut self) {
        self.bottom += match self.direction {
            Direction::Up => points::LASER_MOVE,
            Direction::Down => -1.0 * points::LASER_MOVE,
        };
    }

    ///
    pub fn is_visible(&self) -> bool {
        self.bottom >= 0.0 && (self.bottom + points::LASER_HEIGHT) <= points::GAME_HEIGHT
    }

    ///
    pub fn area(&self) -> Area {
        Area::new(
            self.left,
            self.bottom,
            self.left + points::LASER_WIDTH,
            self.bottom + points::LASER_HEIGHT,
        )
    }
}

impl Shape for Laser {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in &points::LASER {
            let x = x + self.left;
            let y = y + self.bottom;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

///
#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
}

///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LaserFrom {
    Invader,
    Cannon,
}
