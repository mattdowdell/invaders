//!

use crate::assets::{Bunkers, Cannon, InvaderGrid, Laser, MysteryShip};

const ALIEN_COUNTER_DEFAULT: u8 = 5;
const DEFAULT_LIVES: u8 = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct App {
    pub score: u32,
    pub hiscore: u32,
    pub show_help: bool,
    pub paused: bool,
    pub should_quit: bool,
    pub cannon: Cannon,
    pub bunkers: Bunkers,
    pub mystery_ship: MysteryShip,
    mystery_ship_interval: u16,
    mystery_ship_counter: u16,
    pub grid: InvaderGrid,
    pub lasers: Vec<Laser>,
    pub lives: u8,
    alien_counter: u8,
    alien_counter_max: u8,
    count_threshold: usize,
    max_cannon_lasers: usize,
}

impl App {
    ///
    pub fn new(mystery_ship_interval: u16, max_cannon_lasers: u8) -> Self {
        let grid = InvaderGrid::new();

        Self {
            score: 0,
            hiscore: 0,
            show_help: false,
            paused: false,
            should_quit: false,
            cannon: Cannon::new_normal(),
            bunkers: Bunkers::new(),
            mystery_ship: MysteryShip::new(),
            mystery_ship_interval,
            mystery_ship_counter: mystery_ship_interval,
            count_threshold: grid.count(),
            grid,
            lasers: Vec::new(),
            lives: DEFAULT_LIVES,
            alien_counter: ALIEN_COUNTER_DEFAULT,
            alien_counter_max: ALIEN_COUNTER_DEFAULT,
            max_cannon_lasers: max_cannon_lasers.into(),
        }
    }

    ///
    pub fn is_paused(&self) -> bool {
        self.paused || self.show_help
    }

    ///
    pub fn on_tick(&mut self) {
        self.mystery_ship_on_tick();
        self.move_grid();
        self.check_lasers();

        self.lasers_on_tick();
        self.check_lasers();

        if self.grid.is_empty() {
            self.grid = InvaderGrid::new();
            self.count_threshold = self.grid.count();
            self.alien_counter_max = ALIEN_COUNTER_DEFAULT;
        } else {
            self.check_threshold();
        }
    }

    fn mystery_ship_on_tick(&mut self) {
        if self.mystery_ship_counter == 0 {
            if self.mystery_ship.is_visible() {
                self.mystery_ship.on_tick();
            } else {
                self.mystery_ship.reset();
                self.mystery_ship_counter = self.mystery_ship_interval;
            }
        } else {
            self.mystery_ship_counter -= 1;
        }
    }

    fn lasers_on_tick(&mut self) {
        let mut to_delete = vec![];

        for (i, laser) in self.lasers.iter_mut().enumerate() {
            laser.on_tick();

            if !laser.is_visible() {
                to_delete.push(i);
            }
        }

        for i in to_delete.into_iter().rev() {
            self.lasers.remove(i);
        }
    }

    fn check_lasers(&mut self) {
        let mut lasers_to_delete = vec![];

        for (i, laser) in self.lasers.iter().enumerate() {
            if let Some(score) = self.grid.dies(laser) {
                self.score += score;
                lasers_to_delete.push(i);
                continue;
            }

            if let Some(score) = self.mystery_ship.collides_with(laser) {
                self.score += score;
                self.mystery_ship.hide();
                lasers_to_delete.push(i);
            }
        }

        // go through in reverse order so we can delete multiple elements in one pass
        // otherwise deleting causes all subsequent elements to move to the previous index
        // and we delete the wrong one (or panic if there's none left)
        for i in lasers_to_delete.into_iter().rev() {
            self.lasers.remove(i);
        }
    }

    fn move_grid(&mut self) {
        if self.alien_counter == 0 {
            self.grid.move_along();
            self.alien_counter = self.alien_counter_max;
        } else {
            self.alien_counter -= 1;
        }
    }

    fn check_threshold(&mut self) {
        if self.grid.count() <= (self.count_threshold / 2) {
            self.count_threshold /= 2;

            if self.alien_counter_max > 0 {
                self.alien_counter_max -= 1;
            }
        }
    }

    ///
    pub fn on_left(&mut self) {
        if !self.is_paused() {
            self.cannon.move_left();
        }
    }

    ///
    pub fn on_right(&mut self) {
        if !self.is_paused() {
            self.cannon.move_right();
        }
    }

    ///
    pub fn on_space(&mut self) {
        if !self.is_paused() && self.lasers.len() < self.max_cannon_lasers {
            self.lasers.push(Laser::new_cannon(self.cannon.origin_x));
        }
    }

    ///
    pub fn on_h(&mut self) {
        if !self.paused {
            self.show_help ^= true;
        }
    }

    ///
    pub fn on_p(&mut self) {
        if !self.show_help {
            self.paused ^= true;
        }
    }

    ///
    pub fn on_q(&mut self) {
        self.should_quit = true;
    }

    ///
    pub fn on_ctrl_c(&mut self) {
        self.should_quit = true;
    }
}
