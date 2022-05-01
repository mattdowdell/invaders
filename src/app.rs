//!

use rand::rngs::ThreadRng;

use crate::assets::{Bunkers, Cannon, InvaderGrid, Laser, MysteryShip};

const INVADER_LASER_COUNTER_DEFAULT: u8 = 16;
const ALIEN_COUNTER_DEFAULT: u8 = 5;
const DEFAULT_LIVES: u8 = 3;
const MAX_LEVEL: u8 = 6;

#[derive(Clone, Debug)]
pub struct App {
    pub started: bool,
    pub game_over: bool,
    pub score: u32,
    pub hiscore: u32,
    level: u8,
    pub show_help: bool,
    pub paused: bool,
    pub should_quit: bool,
    pub cannon: Cannon,
    pub bunkers: Bunkers,
    pub mystery_ship: MysteryShip,
    mystery_ship_interval: u16,
    mystery_ship_counter: u16,
    pub grid: InvaderGrid,
    pub cannon_lasers: Vec<Laser>,
    pub invader_lasers: Vec<Laser>,
    pub lives: u8,
    alien_counter: u8,
    alien_counter_max: u8,
    invader_laser_counter: u8,
    count_threshold: usize,
    max_cannon_lasers: usize,
    max_invader_lasers: usize,
    rng: ThreadRng,
}

impl App {
    ///
    pub fn new(
        mystery_ship_interval: u16,
        max_cannon_lasers: u8,
        max_invader_lasers: u8,
        level: u8,
    ) -> Self {
        let level = if level > MAX_LEVEL { MAX_LEVEL } else { level };
        let grid = InvaderGrid::new(level);

        Self {
            started: false,
            game_over: false,
            score: 0,
            hiscore: 0,
            level,
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
            cannon_lasers: Vec::new(),
            invader_lasers: Vec::new(),
            lives: DEFAULT_LIVES,
            alien_counter: ALIEN_COUNTER_DEFAULT,
            alien_counter_max: ALIEN_COUNTER_DEFAULT,
            invader_laser_counter: INVADER_LASER_COUNTER_DEFAULT,
            max_cannon_lasers: max_cannon_lasers.into(),
            max_invader_lasers: max_invader_lasers.into(),
            rng: rand::thread_rng(),
        }
    }

    ///
    pub fn start(&mut self) {
        self.started = true;
    }

    ///
    pub fn reset_game(&mut self) {
        self.game_over = false;
        self.cannon_lasers.clear();
        self.invader_lasers.clear();
        self.level = 0;
        self.score = 0;
        self.lives = DEFAULT_LIVES;
        self.bunkers = Bunkers::new();
        self.mystery_ship.hide();
        self.mystery_ship_counter = self.mystery_ship_interval;
        self.cannon.reset();

        self.reset_grid();
    }

    ///
    pub fn reset_grid(&mut self) {
        self.grid = InvaderGrid::new(self.level);
        self.count_threshold = self.grid.count();
        self.alien_counter_max = ALIEN_COUNTER_DEFAULT;
    }

    ///
    pub fn playing(&self) -> bool {
        self.started && !self.game_over && !self.paused && !self.show_help
    }

    ///
    pub fn on_tick(&mut self) {
        if !self.playing() {
            return;
        }

        self.mystery_ship_on_tick();
        self.move_grid();
        self.check_collisions();

        if !self.playing() {
            return;
        }

        self.lasers_on_tick();
        self.check_collisions();

        if self.grid.is_empty() {
            if self.level < MAX_LEVEL {
                self.level += 1;
            } else {
                self.level = 0;
            }

            self.reset_grid();
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
        let mut cannon_lasers_to_delete = vec![];
        let mut invader_lasers_to_delete = vec![];

        for (i, laser) in self.cannon_lasers.iter_mut().enumerate() {
            laser.on_tick();

            if !laser.is_visible() {
                cannon_lasers_to_delete.push(i);
            }
        }

        for (i, laser) in self.invader_lasers.iter_mut().enumerate() {
            laser.on_tick();

            if !laser.is_visible() {
                invader_lasers_to_delete.push(i);
            }
        }

        for i in cannon_lasers_to_delete.into_iter().rev() {
            self.cannon_lasers.remove(i);
        }

        for i in invader_lasers_to_delete.into_iter().rev() {
            self.invader_lasers.remove(i);
        }
    }

    fn check_collisions(&mut self) {
        if !self.grid.is_visible() {
            self.game_over = true;
            return;
        }

        if self.grid.collides_with_cannon(&self.cannon) {
            self.game_over = true;
            return;
        }

        let mut cannon_lasers_to_delete = vec![];

        for (i, laser) in self.cannon_lasers.iter().enumerate() {
            if let Some(score) = self.grid.collides_with_laser(laser) {
                self.score += score;
                cannon_lasers_to_delete.push(i);
                continue;
            }

            if let Some(score) = self.mystery_ship.collides_with(laser) {
                self.score += score;
                self.mystery_ship.hide();
                cannon_lasers_to_delete.push(i);
            }

            if self.bunkers.collides_with_laser(laser) {
                cannon_lasers_to_delete.push(i);
            }
        }

        // go through in reverse order so we can delete multiple elements in one pass
        // otherwise deleting causes all subsequent elements to move to the previous index
        // and we delete the wrong one (or panic if there's none left)
        for i in cannon_lasers_to_delete.into_iter().rev() {
            self.cannon_lasers.remove(i);
        }

        let mut invader_lasers_to_delete = vec![];

        for (i, laser) in self.invader_lasers.iter().enumerate() {
            if self.cannon.collides_with_laser(laser) {
                if self.lives == 0 {
                    self.game_over = true;
                    return;
                }

                self.lives -= 1;

                self.cannon.reset();
                invader_lasers_to_delete.push(i);
            }

            if self.bunkers.collides_with_laser(laser) {
                invader_lasers_to_delete.push(i);
            }
        }

        // go through in reverse order so we can delete multiple elements in one pass
        // otherwise deleting causes all subsequent elements to move to the previous index
        // and we delete the wrong one (or panic if there's none left)
        for i in invader_lasers_to_delete.into_iter().rev() {
            self.invader_lasers.remove(i);
        }

        for invader in self.grid.collides_with_bunkers(&self.bunkers).into_iter() {
            self.bunkers.collides_with_invader(invader);
        }
    }

    fn move_grid(&mut self) {
        if self.alien_counter == 0 {
            self.grid.move_along();
            self.alien_counter = self.alien_counter_max;

            if self.invader_laser_counter > 0 {
                self.invader_laser_counter -= 1;
            }

            if self.invader_laser_counter == 0
                && self.invader_lasers.len() < self.max_invader_lasers
            {
                if let Some(laser) = self.grid.laser(&mut self.rng) {
                    self.invader_lasers.push(laser);
                    self.invader_laser_counter = INVADER_LASER_COUNTER_DEFAULT;
                }
            }
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
        if self.playing() {
            self.cannon.move_left();
        }
    }

    ///
    pub fn on_right(&mut self) {
        if self.playing() {
            self.cannon.move_right();
        }
    }

    ///
    pub fn on_space(&mut self) {
        if !self.started {
            self.start()
        } else if self.game_over {
            self.reset_game();
        } else if self.playing() && self.cannon_lasers.len() < self.max_cannon_lasers {
            self.cannon_lasers.push(Laser::new_cannon(self.cannon.left));
        }
    }

    ///
    pub fn on_h(&mut self) {
        if self.playing() || self.show_help {
            self.show_help ^= true;
        }
    }

    ///
    pub fn on_p(&mut self) {
        if self.playing() || self.paused {
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

    ///
    pub fn on_esc(&mut self) {
        self.should_quit = true;
    }
}
