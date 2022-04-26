//!

use crate::assets::{Grid, Mothership, Shooter, Shot};

// triggers the mothership every 2.5 mins with a 5ms tick
// const MOTHERSHIP_INTERVAL_TICKS: u16 = 3_000;
const MOTHERSHIP_INTERVAL_TICKS: u16 = 600;

const DEFAULT_LIVES: u8 = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct App {
    show_help: bool,
    paused: bool,
    pub should_quit: bool,
    pub shooter: Shooter,
    shooter_moved: bool,
    pub mothership: Mothership,
    mothership_counter: u16,
    pub grid: Grid,
    pub shots: Vec<Shot>,
    pub lives: u8,
}

impl App {
    ///
    pub fn new() -> Self {
        Self {
            show_help: false,
            paused: false,
            should_quit: false,
            shooter: Shooter::new_normal(),
            shooter_moved: false,
            mothership: Mothership::new(),
            mothership_counter: MOTHERSHIP_INTERVAL_TICKS,
            grid: Grid::new(),
            shots: Vec::new(),
            lives: DEFAULT_LIVES,
        }
    }

    ///
    pub fn on_tick(&mut self) {
        self.shooter_moved = false;

        // TODO: split into
        if self.mothership_counter == 0 {
            if self.mothership.is_visible() {
                self.mothership.move_left();
            } else {
                self.mothership.reset();
                self.mothership_counter = MOTHERSHIP_INTERVAL_TICKS;
            }
        } else {
            self.mothership_counter -= 1;
        }

        let mut shots_to_delete = vec![];

        for (i, shot) in self.shots.iter_mut().enumerate() {
            if shot.is_visible() {
                shot.move_up();
            } else {
                shots_to_delete.push(i);
            }
        }

        for i in shots_to_delete.into_iter() {
            self.shots.remove(i);
        }
    }

    ///
    pub fn on_left(&mut self) {
        if !self.shooter_moved {
            self.shooter.move_left();
        }

        self.shooter_moved = true;
    }

    ///
    pub fn on_right(&mut self) {
        if !self.shooter_moved {
            self.shooter.move_right();
        }

        self.shooter_moved = true;
    }

    ///
    pub fn on_space(&mut self) {
        self.shots.push(Shot::new(self.shooter.origin_x))
    }

    ///
    pub fn on_h(&mut self) {
        self.show_help = true;
    }

    ///
    pub fn on_p(&mut self) {
        self.paused ^= true;
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
