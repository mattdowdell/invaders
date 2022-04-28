//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points;

const INVADERS_PER_ROW: usize = 8;

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
        self.origin_y += points::LASER_MOVE;
    }

    ///
    pub fn is_visible(&self) -> bool {
        self.origin_y < (points::GAME_HEIGHT + 1.0)
    }

    ///
    pub fn area(&self) -> Area {
        Area::new(
            self.origin_x,
            self.origin_y,
            self.origin_x + points::LASER_WIDTH,
            self.origin_y + points::LASER_HEIGHT,
        )
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
#[derive(Clone, Debug, PartialEq)]
pub struct Bunkers {
    bunkers: Vec<Bunker>,
}

impl Bunkers {
    ///
    pub fn new() -> Self {
        let mut bunkers = Vec::new();
        let x_spacing =
            (points::GAME_WIDTH - (2.0 * points::BUNKER_OFFSET_X) - (4.0 * points::BUNKER_WIDTH))
                / 3.0;

        for i in 0..4 {
            let x_offset =
                points::BUNKER_OFFSET_X + (i as f64 * (points::BUNKER_WIDTH + x_spacing));
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

///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Area {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl Area {
    ///
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    ///
    pub fn overlaps(&self, other: Area) -> bool {
        let x_check = compare(self.min_x, other.min_x)
            + compare(self.min_x, other.max_x)
            + compare(self.max_x, other.min_x)
            + compare(self.max_x, other.max_x);
        let y_check = compare(self.min_y, other.min_y)
            + compare(self.min_y, other.max_y)
            + compare(self.max_y, other.min_y)
            + compare(self.max_y, other.max_y);

        (x_check != 4 && x_check != -4) && (y_check != 4 && y_check != -4)
    }
}

fn compare(lhs: f64, rhs: f64) -> i8 {
    if lhs < rhs {
        -1
    } else if lhs > rhs {
        1
    } else {
        0
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

    pub fn hide(&mut self) {
        self.origin_x = 0.0 - points::MYSTERY_SHIP_WIDTH - 1.0;
    }

    ///
    pub fn move_left(&mut self) {
        if self.is_visible() {
            self.origin_x -= points::MYSTERY_SHIP_MOVE;
        }
    }

    ///
    pub fn dies(&mut self, laser: &Laser) -> Option<u32> {
        if self.area().overlaps(laser.area()) {
            let laser_min_x = laser.area().min_x;
            let mystery_ship_min_x = self.area().min_x;

            let offset = (laser_min_x - mystery_ship_min_x) as u8;

            match offset {
                0..=1 | 14..=15 => Some(50),
                2..=3 | 12..=13 => Some(100),
                4..=5 | 10..=11 => Some(150),
                6..=9 => Some(200),
                _ => panic!(
                    "Unexpected mystery ship offset: {} ({}, {})",
                    offset, laser_min_x, mystery_ship_min_x
                ),
            }
        } else {
            None
        }
    }

    ///
    pub fn area(&self) -> Area {
        Area::new(
            self.origin_x,
            self.origin_y,
            self.origin_x + points::MYSTERY_SHIP_WIDTH,
            self.origin_y + points::MYSTERY_SHIP_HEIGHT,
        )
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
    origin_x: f64,
    origin_y: f64,
    direction: Direction,
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

        Self {
            rows,
            origin_x: points::GRID_INITIAL_X,
            origin_y: points::GRID_INITIAL_Y,
            direction: Direction::default(),
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
            self.origin_y -= points::ROW_HEIGHT;
        } else {
            self.origin_x += match self.direction {
                Direction::Left => -1.0 * points::INVADER_MOVE,
                Direction::Right => points::INVADER_MOVE,
            };
        }

        for row in self.rows.iter_mut() {
            if on_edge {
                row.move_down(points::ROW_HEIGHT);
            } else {
                row.move_along(self.direction, points::INVADER_MOVE);
            }
        }
    }

    pub fn dies(&mut self, laser: &Laser) -> Option<u32> {
        if self.area().overlaps(laser.area()) {
            for row in self.rows.iter_mut() {
                if let Some(score) = row.dies(laser) {
                    return Some(score);
                }
            }
        }

        None
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
    origin_x: f64,
    origin_y: f64,
    pub size: usize,
    pub count: usize,
}

impl Row {
    ///
    pub fn new(monster: Monster, origin_x: f64, origin_y: f64) -> Self {
        let mut aliens = Vec::new();

        for i in 0..INVADERS_PER_ROW {
            let alien = Alien::new(
                monster,
                origin_x + (points::ALIEN_WIDTH + points::ALIEN_BUFFER_WIDTH) * (i as f64),
                origin_y,
            );
            aliens.push(Some(alien));
        }

        Self {
            aliens,
            origin_x,
            origin_y,
            size: INVADERS_PER_ROW,
            count: INVADERS_PER_ROW,
        }
    }

    ///
    pub fn on_edge(&self, direction: Direction) -> bool {
        for alien in self.aliens.iter().flatten() {
            if alien.on_edge(direction) {
                return true;
            }
        }

        false
    }

    ///
    pub fn move_along(&mut self, direction: Direction, movement: f64) {
        self.origin_x += match direction {
            Direction::Left => -1.0 * points::INVADER_MOVE,
            Direction::Right => points::INVADER_MOVE,
        };

        for alien in self.aliens.iter_mut().flatten() {
            alien.move_along(direction, movement);
        }
    }

    ///
    pub fn move_down(&mut self, movement: f64) {
        self.origin_y -= points::ROW_HEIGHT;

        for alien in self.aliens.iter_mut().flatten() {
            alien.move_down(movement);
        }
    }

    ///
    pub fn dies(&mut self, laser: &Laser) -> Option<u32> {
        if self.area().overlaps(laser.area()) {
            for (i, alien) in self.aliens.iter_mut().enumerate() {
                if let Some(alien) = alien {
                    if let Some(score) = alien.dies(laser) {
                        self.delete(i);
                        return Some(score);
                    }
                }
            }
        }

        None
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
        if self.aliens[index].is_some() {
            self.aliens[index] = None;
            self.count -= 1;
            // TODO: refresh self.size
        }
    }

    ///
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
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
}

impl Alien {
    ///
    pub fn new(monster: Monster, origin_x: f64, origin_y: f64) -> Self {
        Self {
            monster,
            form: AlienForm::default(),
            origin_x,
            origin_y,
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
    pub fn on_edge(&self, direction: Direction) -> bool {
        match direction {
            Direction::Left => self.origin_x <= points::GRID_INITIAL_X,
            Direction::Right => (self.origin_x + points::ALIEN_WIDTH) >= points::GAME_WIDTH,
        }
    }

    ///
    pub fn move_along(&mut self, direction: Direction, movement: f64) {
        self.origin_x += match direction {
            Direction::Left => -1.0 * movement,
            Direction::Right => movement,
        };

        self.form = self.form.switch();
    }

    ///
    pub fn move_down(&mut self, movement: f64) {
        self.origin_y -= movement;
        self.form = self.form.switch();
    }

    ///
    pub fn dies(&self, laser: &Laser) -> Option<u32> {
        if self.area().overlaps(laser.area()) {
            Some(self.monster.score())
        } else {
            None
        }
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

impl Shape for Alien {
    fn draw(&self, painter: &mut Painter) {
        let x_offset = self.draw_x_offset();
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
pub enum AlienForm {
    Original,
    Alternate,
}

///
impl AlienForm {
    ///
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
