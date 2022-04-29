//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

use super::{Area, Laser};

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MysteryShip {
    left: f64,
    right: f64,
}

impl MysteryShip {
    ///
    pub fn new() -> Self {
        Self {
            left: points::MYSTERY_SHIP_INITIAL_X,
            right: points::MYSTERY_SHIP_INITIAL_Y,
        }
    }

    /// Reset the location of the myster ship to the far right of the game window.
    pub fn reset(&mut self) {
        self.left = points::MYSTERY_SHIP_INITIAL_X;
    }

    /// Test whether the mystery ship is within the game window.
    pub fn is_visible(&self) -> bool {
        self.left > (0.0 - points::MYSTERY_SHIP_WIDTH)
    }

    /// Instantly move the mystery ship to it's final position. Should be used if a laser
    /// successfully collides with the mystery ship.
    pub fn hide(&mut self) {
        self.left = 0.0 - points::MYSTERY_SHIP_WIDTH - 1.0;
    }

    ///
    pub fn on_tick(&mut self) {
        if self.is_visible() {
            self.left -= points::MYSTERY_SHIP_MOVE;
        }
    }

    ///
    pub fn collides_with(&mut self, laser: &Laser) -> Option<u32> {
        if self.area().overlaps(laser.area()) {
            let laser_left = laser.area().left;
            let mystery_ship_left = self.area().left;

            let offset = (laser_left - mystery_ship_left) as u8;

            match offset {
                0..=1 | 14..=15 => Some(50),
                2..=3 | 12..=13 => Some(100),
                4..=5 | 10..=11 => Some(150),
                6..=9 => Some(200),
                _ => panic!(
                    "Unexpected mystery ship offset: {} ({}, {})",
                    offset, laser_left, mystery_ship_left
                ),
            }
        } else {
            None
        }
    }

    ///
    fn area(&self) -> Area {
        Area::new(
            self.left,
            self.right,
            self.left + points::MYSTERY_SHIP_WIDTH,
            self.right + points::MYSTERY_SHIP_HEIGHT,
        )
    }
}

impl Shape for MysteryShip {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in &points::MYSTERY_SHIP {
            let x = x + self.left;
            let y = y + self.right;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, Color::Red);
            }
        }
    }
}
