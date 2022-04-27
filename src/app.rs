//!

use crate::assets::{Cannon, Grid, Laser, MysteryShip};

// triggers the mystery ship every 2.5 mins with a 5ms tick
// const MYSTERY_SHIP_INTERVAL_TICKS: u16 = 3_000;
const MYSTERY_SHIP_INTERVAL_TICKS: u16 = 200;
const ALIEN_COUNTER_DEFAULT: u8 = 4;
const SHOT_RATELIMIT_MAX: u8 = 3;

const DEFAULT_LIVES: u8 = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct App {
    pub score: u32,
    pub hiscore: u32,
    pub show_help: bool,
    pub paused: bool,
    pub should_quit: bool,
    pub cannon: Cannon,
    pub mystery_ship: MysteryShip,
    mystery_ship_counter: u16,
    pub grid: Grid,
    pub lasers: Vec<Laser>,
    pub lives: u8,
    alien_counter: u8,
    laser_ratelimit: u8,
}

impl App {
    ///
    pub fn new() -> Self {
        Self {
            score: 0,
            hiscore: 0,
            show_help: false,
            paused: false,
            should_quit: false,
            cannon: Cannon::new_normal(),
            mystery_ship: MysteryShip::new(),
            mystery_ship_counter: MYSTERY_SHIP_INTERVAL_TICKS,
            grid: Grid::new(),
            lasers: Vec::new(),
            lives: DEFAULT_LIVES,
            alien_counter: ALIEN_COUNTER_DEFAULT,
            laser_ratelimit: 0,
        }
    }

    ///
    pub fn is_paused(&self) -> bool {
        self.paused || self.show_help
    }

    ///
    pub fn on_tick(&mut self) {
        self.move_mystery_ship();
        self.move_shots();
        self.move_grid();

        // decrement shot ratelimit
        if self.laser_ratelimit > 0 {
            self.laser_ratelimit -= 1;
        }
    }

    fn move_mystery_ship(&mut self) {
        if self.mystery_ship_counter == 0 {
            if self.mystery_ship.is_visible() {
                self.mystery_ship.move_left();
            } else {
                self.mystery_ship.reset();
                self.mystery_ship_counter = MYSTERY_SHIP_INTERVAL_TICKS;
            }
        } else {
            self.mystery_ship_counter -= 1;
        }
    }

    fn move_shots(&mut self) {
        let mut lasers_to_delete = vec![];

        for (i, laser) in self.lasers.iter_mut().enumerate() {
            if laser.is_visible() {
                laser.move_up();
            } else {
                lasers_to_delete.push(i);
            }
        }

        for i in lasers_to_delete.into_iter() {
            self.lasers.remove(i);
        }
    }

    fn move_grid(&mut self) {
        if self.alien_counter == 0 {
            self.grid.move_along();
            self.alien_counter = ALIEN_COUNTER_DEFAULT;
        } else {
            self.alien_counter -= 1;
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
        if !self.is_paused() && self.laser_ratelimit == 0 {
            self.lasers.push(Laser::new(self.cannon.origin_x));
            self.laser_ratelimit = SHOT_RATELIMIT_MAX;
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
