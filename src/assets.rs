//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shooter {
    origin: (f64, f64),
    color: Color,
}

impl Shooter {
    ///
    pub fn new(origin_x: i16, origin_y: i16) -> Self {
        Self {
            origin: (origin_x as f64, origin_y as f64),
            color: Color::Green,
        }
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        &points::SHOOTER
    }
}

impl Shape for Shooter {
    fn draw(&self, painter: &mut Painter) {
        let (origin_x, origin_y) = self.origin;

        for (x, y) in self.data() {
            let x = x + origin_x;
            let y = y + origin_y;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shield {
    origin: (f64, f64),
    color: Color,
}

impl Shield {
    ///
    pub fn new(origin_x: i16, origin_y: i16) -> Self {
        Self {
            origin: (origin_x as f64, origin_y as f64),
            color: Color::Green,
        }
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        &points::SHIELD
    }
}

impl Shape for Shield {
    fn draw(&self, painter: &mut Painter) {
        let (origin_x, origin_y) = self.origin;

        for (x, y) in self.data() {
            let x = x + origin_x;
            let y = y + origin_y;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mothership {
    origin: (f64, f64),
    color: Color,
}

impl Mothership {
    ///
    pub fn new(origin_x: i16, origin_y: i16) -> Self {
        Self {
            origin: (origin_x as f64, origin_y as f64),
            color: Color::Red,
        }
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        &points::MOTHERSHIP
    }
}

impl Shape for Mothership {
    fn draw(&self, painter: &mut Painter) {
        let (origin_x, origin_y) = self.origin;

        for (x, y) in self.data() {
            let x = x + origin_x;
            let y = y + origin_y;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

///
pub struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    ///
    pub fn new(origin_x: usize, origin_y: usize) -> Self {
        let mut rows = Vec::new();
        let monsters = vec![
            Monster::Crab,
            Monster::Crab,
            Monster::Squid,
            Monster::Squid,
            Monster::Octopus,
        ];

        for (i, monster) in monsters.into_iter().enumerate() {
            let row = Row::new(monster, origin_x, origin_y + 10 * i);
            rows.push(row);
        }

        Self { rows }
    }
}

impl Shape for Grid {
    fn draw(&self, painter: &mut Painter) {
        for row in self.rows.iter() {
            row.draw(painter);
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    aliens: Vec<Option<Alien>>,
}

impl Row {
    ///
    pub fn new(monster: Monster, origin_x: usize, origin_y: usize) -> Self {
        let mut aliens = Vec::new();

        for i in 0..8 {
            let alien = Alien::new(monster, origin_x + 16 * i, origin_y);
            aliens.push(Some(alien));
        }

        Self { aliens }
    }

    // pub fn delete(&mut self, index: usize) {
    //     self.aliens[index] = None;
    // }
}

impl Shape for Row {
    fn draw(&self, painter: &mut Painter) {
        for alien in self.aliens.iter().flatten() {
            alien.draw(painter);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Alien {
    monster: Monster,
    form: Form,
    origin: (f64, f64),
}

impl Alien {
    ///
    pub fn new(monster: Monster, origin_x: usize, origin_y: usize) -> Self {
        Self {
            monster,
            form: Form::default(),
            origin: (origin_x as f64, origin_y as f64),
        }
    }

    ///
    pub fn data(&self) -> &'static [(f64, f64)] {
        match self.monster {
            Monster::Crab => match self.form {
                Form::Original => &points::CRAB,
                Form::Alternate => &points::CRAB_ALT,
            },
            Monster::Squid => match self.form {
                Form::Original => &points::SQUID,
                Form::Alternate => &points::SQUID_ALT,
            },
            Monster::Octopus => match self.form {
                Form::Original => &points::OCTOPUS,
                Form::Alternate => &points::OCTOPUS_ALT,
            },
        }
    }

    ///
    pub fn width(&self) -> f64 {
        match self.monster {
            Monster::Crab => 11.0,
            Monster::Squid => 8.0,
            Monster::Octopus => 12.0,
        }
    }
}

impl Shape for Alien {
    fn draw(&self, painter: &mut Painter) {
        let (origin_x, origin_y) = self.origin;
        let x_offset = (12.0 - self.width() / 2.0).floor();
        let color = self.monster.color();

        for (x, y) in self.data() {
            let x = x + origin_x + x_offset;
            let y = y + origin_y;

            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, color);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Monster {
    Crab,
    Squid,
    Octopus,
}

impl Monster {
    ///
    pub fn color(&self) -> Color {
        match self {
            Self::Crab => Color::White,
            Self::Squid => Color::Yellow,
            Self::Octopus => Color::Cyan,
        }
    }
}

///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Form {
    Original,
    Alternate,
}

///
impl Form {
    pub fn switch(&self) -> Self {
        match self {
            Self::Original => Self::Alternate,
            Self::Alternate => Self::Original,
        }
    }
}

///
impl Default for Form {
    fn default() -> Self {
        Self::Original
    }
}
