//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shot {
    origin_x: f64,
    origin_y: f64,
    color: Color,
}

impl Shot {
    ///
    pub fn new(shooter_origin_x: f64) -> Self {
        Self {
            origin_x: shooter_origin_x + points::SHOT_INITIAL_X_OFFSET,
            origin_y: points::SHOT_INITIAL_Y,
            color: Color::Green,
        }
    }

    ///
    pub fn move_up(&mut self) {
        self.origin_y += 2.0
    }

    ///
    pub fn is_visible(&self) -> bool {
        self.origin_y < (points::GAME_HEIGHT + 1.0)
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        &points::SHOT
    }
}

impl Shape for Shot {
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
pub struct Shooter {
    pub origin_x: f64,
    origin_y: f64,
    form: ShooterForm,
    color: Color,
}

impl Shooter {
    ///
    pub fn new_normal() -> Self {
        Self {
            origin_x: points::SHOOTER_INITIAL_X,
            origin_y: points::SHOOTER_INITIAL_Y,
            form: ShooterForm::Normal,
            color: Color::Green,
        }
    }

    ///
    pub fn new_small(x_offset: f64) -> Self {
        Self {
            origin_x: points::SHOOTER_INITIAL_X + x_offset,
            origin_y: points::SHOOTER_INITIAL_Y,
            form: ShooterForm::Small,
            color: Color::Green,
        }
    }

    ///
    pub fn move_left(&mut self) {
        if self.origin_x > 1.0 {
            self.origin_x -= 2.0;
        }
    }

    ///
    pub fn move_right(&mut self) {
        if self.origin_x < (points::GAME_WIDTH - points::SHOOTER_WIDTH) {
            self.origin_x += 2.0;
        }
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        match self.form {
            ShooterForm::Normal => &points::SHOOTER,
            ShooterForm::Small => &points::SHOOTER_SMALL,
        }
    }
}

impl Shape for Shooter {
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
pub enum ShooterForm {
    Normal,
    Small,
}

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Shield {
    origin: (f64, f64),
    color: Color,
}

impl Shield {
    ///
    pub fn new(origin_x: usize, origin_y: usize) -> Self {
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
    origin_x: f64,
    origin_y: f64,
    color: Color,
}

impl Mothership {
    ///
    pub fn new() -> Self {
        Self {
            origin_x: points::MOTHERSHIP_INITIAL_X,
            origin_y: points::MOTHERSHIP_INITIAL_Y,
            color: Color::Red,
        }
    }

    pub fn reset(&mut self) {
        self.origin_x = points::MOTHERSHIP_INITIAL_X;
    }

    pub fn is_visible(&self) -> bool {
        self.origin_x > (0.0 - points::MOTHERSHIP_WIDTH)
    }

    ///
    pub fn move_left(&mut self) {
        if self.is_visible() {
            self.origin_x -= 2.0
        }
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        &points::MOTHERSHIP
    }
}

impl Shape for Mothership {
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
#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    ///
    pub fn new() -> Self {
        let mut rows = Vec::new();
        let monsters = vec![
            Monster::Crab,
            Monster::Crab,
            Monster::Squid,
            Monster::Squid,
            Monster::Octopus,
        ];

        for (i, monster) in monsters.into_iter().enumerate() {
            let row = Row::new(
                monster,
                points::GRID_INITIAL_X,
                points::GRID_INITIAL_Y + (points::ROW_HEIGHT * (i as f64)),
            );
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
    pub fn new(monster: Monster, origin_x: f64, origin_y: f64) -> Self {
        let mut aliens = Vec::new();

        for i in 0..8 {
            let alien = Alien::new(
                monster,
                origin_x + (points::ALIEN_WIDTH + points::ALIEN_BUFFER_WIDTH) * (i as f64),
                origin_y,
            );
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
    form: AlienForm,
    origin: (f64, f64),
}

impl Alien {
    ///
    pub fn new(monster: Monster, origin_x: f64, origin_y: f64) -> Self {
        Self {
            monster,
            form: AlienForm::default(),
            origin: (origin_x, origin_y),
        }
    }

    ///
    pub fn data(&self) -> &'static [(f64, f64)] {
        match self.monster {
            Monster::Crab => match self.form {
                AlienForm::Original => &points::CRAB,
                AlienForm::Alternate => &points::CRAB_ALT,
            },
            Monster::Squid => match self.form {
                AlienForm::Original => &points::SQUID,
                AlienForm::Alternate => &points::SQUID_ALT,
            },
            Monster::Octopus => match self.form {
                AlienForm::Original => &points::OCTOPUS,
                AlienForm::Alternate => &points::OCTOPUS_ALT,
            },
        }
    }

    ///
    pub fn width(&self) -> f64 {
        match self.monster {
            Monster::Crab => points::CRAB_WIDTH,
            Monster::Squid => points::SQUID_WIDTH,
            Monster::Octopus => points::OCTOPUS_WIDTH,
        }
    }
}

impl Shape for Alien {
    fn draw(&self, painter: &mut Painter) {
        let (origin_x, origin_y) = self.origin;
        let x_offset = ((points::ALIEN_WIDTH - self.width()) / 2.0).floor();
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
pub enum AlienForm {
    Original,
    Alternate,
}

///
impl AlienForm {
    pub fn switch(&self) -> Self {
        match self {
            Self::Original => Self::Alternate,
            Self::Alternate => Self::Original,
        }
    }
}

///
impl Default for AlienForm {
    fn default() -> Self {
        Self::Original
    }
}
