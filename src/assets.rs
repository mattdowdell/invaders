//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Laser {
    origin_x: f64,
    origin_y: f64,
    color: Color,
}

impl Laser {
    ///
    pub fn new(shooter_origin_x: f64) -> Self {
        Self {
            origin_x: shooter_origin_x + points::LASER_INITIAL_X_OFFSET,
            origin_y: points::LASER_INITIAL_Y,
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
        &points::LASER
    }
}

impl Shape for Laser {
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
pub struct Cannon {
    pub origin_x: f64,
    origin_y: f64,
    form: CannonForm,
    color: Color,
}

impl Cannon {
    ///
    pub fn new_normal() -> Self {
        Self {
            origin_x: points::CANNON_INITIAL_X,
            origin_y: points::CANNON_INITIAL_Y,
            form: CannonForm::Normal,
            color: Color::Green,
        }
    }

    ///
    pub fn new_small(x_offset: f64) -> Self {
        Self {
            origin_x: points::CANNON_INITIAL_X + x_offset,
            origin_y: points::CANNON_INITIAL_Y,
            form: CannonForm::Small,
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
        if self.origin_x < (points::GAME_WIDTH - points::CANNON_WIDTH) {
            self.origin_x += 2.0;
        }
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        match self.form {
            CannonForm::Normal => &points::CANNON,
            CannonForm::Small => &points::CANNON_SMALL,
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
pub enum CannonForm {
    Normal,
    Small,
}

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Bunker {
    origin_x: f64,
    origin_y: f64,
    color: Color,
}

impl Bunker {
    ///
    pub fn new(origin_x: f64, origin_y: f64) -> Self {
        Self {
            origin_x,
            origin_y,
            color: Color::Green,
        }
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        &points::BUNKER
    }
}

impl Shape for Bunker {
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
pub struct MysteryShip {
    origin_x: f64,
    origin_y: f64,
    color: Color,
}

impl MysteryShip {
    ///
    pub fn new() -> Self {
        Self {
            origin_x: points::MYSTERY_SHIP_INITIAL_X,
            origin_y: points::MYSTERY_SHIP_INITIAL_Y,
            color: Color::Red,
        }
    }

    pub fn reset(&mut self) {
        self.origin_x = points::MYSTERY_SHIP_INITIAL_X;
    }

    pub fn is_visible(&self) -> bool {
        self.origin_x > (0.0 - points::MYSTERY_SHIP_WIDTH)
    }

    ///
    pub fn move_left(&mut self) {
        if self.is_visible() {
            self.origin_x -= 2.0
        }
    }

    //
    fn data(&self) -> &'static [(f64, f64)] {
        &points::MYSTERY_SHIP
    }
}

impl Shape for MysteryShip {
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
            Monster::Octopus,
            Monster::Octopus,
            Monster::Crab,
            Monster::Crab,
            Monster::Squid,
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

    ///
    pub fn move_along(&mut self) {
        let mut on_edge = false;

        for row in self.rows.iter() {
            if row.on_edge() {
                on_edge = true;
                break;
            }
        }

        for row in self.rows.iter_mut() {
            if on_edge {
                row.move_down();
            } else {
                row.move_along();
            }
        }
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

    ///
    pub fn on_edge(&self) -> bool {
        for alien in self.aliens.iter().flatten() {
            if alien.on_edge() {
                return true;
            }
        }

        false
    }

    ///
    pub fn move_along(&mut self) {
        for alien in self.aliens.iter_mut().flatten() {
            alien.move_along();
        }
    }

    ///
    pub fn move_down(&mut self) {
        for alien in self.aliens.iter_mut().flatten() {
            alien.move_down();
        }
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
    origin_x: f64,
    origin_y: f64,
    direction: Direction,
}

impl Alien {
    ///
    pub fn new(monster: Monster, origin_x: f64, origin_y: f64) -> Self {
        Self {
            monster,
            form: AlienForm::default(),
            origin_x,
            origin_y,
            direction: Direction::default(),
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

    ///
    pub fn on_edge(&self) -> bool {
        match self.direction {
            Direction::Left => self.origin_x <= points::GRID_INITIAL_X,
            Direction::Right => (self.origin_x + points::ALIEN_WIDTH) >= points::GAME_WIDTH,
        }
    }

    ///
    pub fn move_along(&mut self) {
        self.origin_x += match self.direction {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        };

        self.form = self.form.switch();
    }

    ///
    pub fn move_down(&mut self) {
        self.origin_y -= points::ROW_HEIGHT;
        self.direction = self.direction.switch();
        self.form = self.form.switch();
    }
}

impl Shape for Alien {
    fn draw(&self, painter: &mut Painter) {
        let x_offset = ((points::ALIEN_WIDTH - self.width()) / 2.0).floor();
        let color = self.monster.color();

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

///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

///
impl Direction {
    pub fn switch(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

///
impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}
