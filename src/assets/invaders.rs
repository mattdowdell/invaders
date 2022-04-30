//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

use super::{Area, Cannon, Laser};

const INVADERS_PER_ROW: usize = 8;

///
#[derive(Clone, Debug, PartialEq)]
pub struct InvaderGrid {
    rows: Vec<InvaderRow>,
    origin_x: f64,
    origin_y: f64,
    direction: InvaderDirection,
}

impl InvaderGrid {
    ///
    pub fn new(level: u8) -> Self {
        let level_offset = level as f64 * points::INVADER_MOVE_Y * -1.0;
        let mut rows = Vec::new();
        let invader_types = vec![
            InvaderType::Octopus,
            InvaderType::Octopus,
            InvaderType::Crab,
            InvaderType::Crab,
            InvaderType::Squid,
        ];

        for (i, invader_type) in invader_types.into_iter().enumerate() {
            let row = InvaderRow::new(
                invader_type,
                points::GRID_INITIAL_X,
                points::GRID_INITIAL_Y + (points::ROW_HEIGHT * (i as f64)) + level_offset,
            );
            rows.push(row);
        }

        Self {
            rows,
            origin_x: points::GRID_INITIAL_X,
            origin_y: points::GRID_INITIAL_Y + level_offset,
            direction: InvaderDirection::default(),
        }
    }

    ///
    pub fn move_along(&mut self) {
        let mut on_edge = false;

        for row in self.rows.iter() {
            if row.on_edge(self.direction) {
                on_edge = true;
                break;
            }
        }

        if on_edge {
            self.direction = self.direction.switch();
            self.origin_y -= points::INVADER_MOVE_Y;
        } else {
            self.origin_x += match self.direction {
                InvaderDirection::Left => -1.0 * points::INVADER_MOVE_X,
                InvaderDirection::Right => points::INVADER_MOVE_X,
            };
        }

        for row in self.rows.iter_mut() {
            if on_edge {
                row.move_down(points::INVADER_MOVE_Y);
            } else {
                row.move_along(self.direction, points::INVADER_MOVE_X);
            }
        }
    }

    pub fn collides_with_laser(&mut self, laser: &Laser) -> Option<u32> {
        if self.area().overlaps(laser.area()) {
            for row in self.rows.iter_mut() {
                if let Some(score) = row.collides_with_laser(laser) {
                    return Some(score);
                }
            }
        }

        None
    }

    ///
    pub fn collides_with_cannon(&self, cannon: &Cannon) -> bool {
        if self.area().overlaps(cannon.area()) {
            for row in self.rows.iter() {
                if row.collides_with_cannon(cannon) {
                    return true;
                }
            }
        }

        false
    }

    ///
    pub fn is_visible(&self) -> bool {
        if self.origin_y > 0.0 {
            return true;
        }

        for row in self.rows.iter() {
            if !row.is_visible() {
                return false
            }
        }

        true
    }

    ///
    pub fn area(&self) -> Area {
        let mut row_width = 0.0;

        for row in self.rows.iter() {
            let width = row.width();

            if row_width < row.width() {
                row_width = width;
            }
        }

        Area::new(
            self.origin_x,
            self.origin_y,
            self.origin_x + row_width,
            self.origin_y + (5.0 * points::ROW_HEIGHT),
        )
    }

    ///
    pub fn count(&self) -> usize {
        let mut count = 0;

        for row in self.rows.iter() {
            count += row.count;
        }

        count
    }

    ///
    pub fn is_empty(&self) -> bool {
        for row in self.rows.iter() {
            if !row.is_empty() {
                return false;
            }
        }

        true
    }
}

impl Shape for InvaderGrid {
    fn draw(&self, painter: &mut Painter) {
        for row in self.rows.iter() {
            row.draw(painter);
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
struct InvaderRow {
    invaders: Vec<Option<Invader>>,
    origin_x: f64,
    origin_y: f64,
    pub size: usize,
    pub count: usize,
}

impl InvaderRow {
    ///
    pub fn new(invader_type: InvaderType, origin_x: f64, origin_y: f64) -> Self {
        let mut invaders = Vec::new();

        for i in 0..INVADERS_PER_ROW {
            let invader = Invader::new(
                invader_type,
                origin_x + (points::ALIEN_WIDTH + points::ALIEN_BUFFER_WIDTH) * (i as f64),
                origin_y,
            );
            invaders.push(Some(invader));
        }

        Self {
            invaders,
            origin_x,
            origin_y,
            size: INVADERS_PER_ROW,
            count: INVADERS_PER_ROW,
        }
    }

    ///
    pub fn on_edge(&self, direction: InvaderDirection) -> bool {
        for invader in self.invaders.iter().flatten() {
            if invader.on_edge(direction) {
                return true;
            }
        }

        false
    }

    ///
    pub fn move_along(&mut self, direction: InvaderDirection, movement: f64) {
        self.origin_x += match direction {
            InvaderDirection::Left => -1.0 * movement,
            InvaderDirection::Right => movement,
        };

        for invader in self.invaders.iter_mut().flatten() {
            invader.move_along(direction, movement);
        }
    }

    ///
    pub fn move_down(&mut self, movement: f64) {
        self.origin_y -= movement;

        for invader in self.invaders.iter_mut().flatten() {
            invader.move_down(movement);
        }
    }

    ///
    pub fn collides_with_laser(&mut self, laser: &Laser) -> Option<u32> {
        if self.area().overlaps(laser.area()) {
            for (i, invader) in self.invaders.iter_mut().enumerate() {
                if let Some(invader) = invader {
                    if let Some(score) = invader.collides_with_laser(laser) {
                        self.delete(i);
                        return Some(score);
                    }
                }
            }
        }

        None
    }

    ///
    pub fn collides_with_cannon(&self, cannon: &Cannon) -> bool {
        if self.area().overlaps(cannon.area()) {
            for invader in self.invaders.iter().flatten() {
                if invader.collides_with_cannon(cannon) {
                    return true;
                }
            }
        }

        false
    }

    ///
    pub fn area(&self) -> Area {
        Area::new(
            self.origin_x,
            self.origin_y,
            self.origin_x + self.width(),
            self.origin_y + points::ALIEN_HEIGHT,
        )
    }

    ///
    pub fn width(&self) -> f64 {
        let size = self.size as f64;
        (size * points::ALIEN_WIDTH) + ((size - 1.0) * points::ALIEN_BUFFER_WIDTH)
    }

    ///
    pub fn delete(&mut self, index: usize) {
        if self.invaders[index].is_some() {
            self.invaders[index] = None;
            self.count -= 1;
            // TODO: refresh self.size to optimise width()
        }
    }

    ///
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    ///
    pub fn is_visible(&self) -> bool {
        self.origin_y > 0.0 || self.is_empty()
    }
}

impl Shape for InvaderRow {
    fn draw(&self, painter: &mut Painter) {
        for invader in self.invaders.iter().flatten() {
            invader.draw(painter);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Invader {
    invader_type: InvaderType,
    animation: InvaderAnimation,
    origin_x: f64,
    origin_y: f64,
}

impl Invader {
    ///
    pub fn new(invader_type: InvaderType, origin_x: f64, origin_y: f64) -> Self {
        Self {
            invader_type,
            animation: InvaderAnimation::default(),
            origin_x,
            origin_y,
        }
    }

    ///
    pub fn data(&self) -> &'static [(f64, f64)] {
        match self.invader_type {
            InvaderType::Crab => match self.animation {
                InvaderAnimation::Original => &points::CRAB,
                InvaderAnimation::Alternate => &points::CRAB_ALT,
            },
            InvaderType::Squid => match self.animation {
                InvaderAnimation::Original => &points::SQUID,
                InvaderAnimation::Alternate => &points::SQUID_ALT,
            },
            InvaderType::Octopus => match self.animation {
                InvaderAnimation::Original => &points::OCTOPUS,
                InvaderAnimation::Alternate => &points::OCTOPUS_ALT,
            },
        }
    }

    ///
    pub fn width(&self) -> f64 {
        match self.invader_type {
            InvaderType::Crab => points::CRAB_WIDTH,
            InvaderType::Squid => points::SQUID_WIDTH,
            InvaderType::Octopus => points::OCTOPUS_WIDTH,
        }
    }

    ///
    pub fn on_edge(&self, direction: InvaderDirection) -> bool {
        match direction {
            InvaderDirection::Left => self.origin_x <= points::GRID_INITIAL_X,
            InvaderDirection::Right => (self.origin_x + points::ALIEN_WIDTH) >= points::GAME_WIDTH,
        }
    }

    ///
    pub fn move_along(&mut self, direction: InvaderDirection, movement: f64) {
        self.origin_x += match direction {
            InvaderDirection::Left => -1.0 * movement,
            InvaderDirection::Right => movement,
        };

        self.animation = self.animation.switch();
    }

    ///
    pub fn move_down(&mut self, movement: f64) {
        self.origin_y -= movement;
        self.animation = self.animation.switch();
    }

    ///
    pub fn collides_with_laser(&self, laser: &Laser) -> Option<u32> {
        if self.area().overlaps(laser.area()) {
            Some(self.invader_type.score())
        } else {
            None
        }
    }

    ///
    pub fn collides_with_cannon(&self, cannon: &Cannon) -> bool {
        self.area().overlaps(cannon.area())
    }

    ///
    pub fn area(&self) -> Area {
        let x_offset = self.draw_x_offset();

        Area::new(
            self.origin_x + x_offset,
            self.origin_y,
            self.origin_x + x_offset + self.width(),
            self.origin_y + points::ALIEN_HEIGHT,
        )
    }

    fn draw_x_offset(&self) -> f64 {
        ((points::ALIEN_WIDTH - self.width()) / 2.0).floor()
    }
}

impl Shape for Invader {
    fn draw(&self, painter: &mut Painter) {
        let x_offset = self.draw_x_offset();
        let color = self.invader_type.color();

        for (x, y) in self.data() {
            let x = x + self.origin_x + x_offset;
            let y = y + self.origin_y;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, color);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InvaderType {
    Crab,
    Squid,
    Octopus,
}

impl InvaderType {
    ///
    pub fn color(&self) -> Color {
        match self {
            Self::Crab => Color::White,
            Self::Squid => Color::Yellow,
            Self::Octopus => Color::Cyan,
        }
    }

    ///
    pub fn score(&self) -> u32 {
        match self {
            Self::Crab => 20,
            Self::Squid => 30,
            Self::Octopus => 10,
        }
    }
}

///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InvaderAnimation {
    Original,
    Alternate,
}

///
impl InvaderAnimation {
    ///
    pub fn switch(&self) -> Self {
        match self {
            Self::Original => Self::Alternate,
            Self::Alternate => Self::Original,
        }
    }
}

///
impl Default for InvaderAnimation {
    fn default() -> Self {
        Self::Original
    }
}

///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InvaderDirection {
    Left,
    Right,
}

///
impl InvaderDirection {
    pub fn switch(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

///
impl Default for InvaderDirection {
    fn default() -> Self {
        Self::Right
    }
}
