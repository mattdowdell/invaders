//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

use super::{Area, Invader, Laser};

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

    ///
    pub fn collides_with_laser(&mut self, laser: &Laser) -> bool {
        if self.area().overlaps(laser.area()) {
            for bunker in self.bunkers.iter_mut() {
                if bunker.collides_with_laser(laser) {
                    return true;
                }
            }
        }

        false
    }

    ///
    pub fn collides_with_invader(&mut self, invader: &Invader) {
        for bunker in self.bunkers.iter_mut() {
            if bunker.collides_with_invader(invader) {
                break;
            }
        }
    }

    //
    pub fn area(&self) -> Area {
        Area::new(
            0.0,
            points::BUNKER_INITIAL_Y,
            points::GAME_WIDTH,
            points::BUNKER_INITIAL_Y + points::BUNKER_HEIGHT,
        )
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
    left: f64,
    bottom: f64,
    color: Color,
    data: Vec<(f64, f64)>,
}

impl Bunker {
    ///
    pub fn new(left: f64, bottom: f64) -> Self {
        Self {
            left,
            bottom,
            color: Color::Green,
            data: points::BUNKER.into(),
        }
    }

    ///
    pub fn collides_with_laser(&mut self, laser: &Laser) -> bool {
        let mut collision = false;
        let mut to_delete = vec![];
        let collision_area = laser.bunker_collision_area();

        if self.area().overlaps(laser.area()) {
            for (i, (x, y)) in self.data.iter().enumerate() {
                let x = x + self.left;
                let y = y + self.bottom;

                if collision_area.contains(x, y) {
                    to_delete.push(i);
                    collision = true;
                }
            }
        }

        for i in to_delete.into_iter().rev() {
            self.data.remove(i);
        }

        collision
    }

    ///
    pub fn collides_with_invader(&mut self, invader: &Invader) -> bool {
        let mut collision = false;
        let mut to_delete = vec![];
        let invader_area = invader.area();

        if self.area().overlaps(invader_area) {
            for (i, (x, y)) in self.data.iter().enumerate() {
                let x = x + self.left;
                let y = y + self.bottom;

                if invader_area.contains(x, y) {
                    to_delete.push(i);
                    collision = true;
                }
            }
        }

        for i in to_delete.into_iter().rev() {
            self.data.remove(i);
        }

        collision
    }

    //
    fn area(&self) -> Area {
        Area::new(
            self.left,
            self.bottom,
            self.left + points::BUNKER_WIDTH,
            self.bottom + points::BUNKER_HEIGHT,
        )
    }
}

impl Shape for Bunker {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.data.iter() {
            let x = x + self.left;
            let y = y + self.bottom;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
