//!

mod bunker;
mod cannon;
mod invaders;
pub mod letters;
mod mystery_ship;

pub use bunker::BUNKER;
pub use cannon::{CANNON, CANNON_SMALL};
pub use invaders::{CRAB, CRAB_ALT, OCTOPUS, OCTOPUS_ALT, SQUID, SQUID_ALT};
pub use mystery_ship::MYSTERY_SHIP;

pub const GAME_WIDTH: f64 = 200.0;
pub const GAME_HEIGHT: f64 = 132.0;

pub const CANNON_WIDTH: f64 = 15.0;
pub const CANNON_SMALL_WIDTH: f64 = 8.0;
pub const BUNKER_WIDTH: f64 = 20.0;
pub const CRAB_WIDTH: f64 = 11.0;
pub const SQUID_WIDTH: f64 = 8.0;
pub const OCTOPUS_WIDTH: f64 = 12.0;
pub const ALIEN_WIDTH: f64 = OCTOPUS_WIDTH;
pub const ALIEN_BUFFER_WIDTH: f64 = 4.0;
pub const MYSTERY_SHIP_WIDTH: f64 = 16.0;

pub const LASER_WIDTH: f64 = 1.0;
pub const LASER_HEIGHT: f64 = 2.0;

pub const CANNON_HEIGHT: f64 = 7.0;
pub const MYSTERY_SHIP_HEIGHT: f64 = 8.0;
pub const ALIEN_HEIGHT: f64 = 8.0;
pub const ALIEN_BUFFER_HEIGHT: f64 = 2.0;
pub const ROW_HEIGHT: f64 = ALIEN_HEIGHT + ALIEN_BUFFER_HEIGHT;
pub const MYSTERY_SHIP_GRID_BUFFER: f64 = 2.0;

pub const CANNON_INITIAL_X: f64 = 1.0;
pub const CANNON_INITIAL_Y: f64 = 0.0;

pub const BUNKER_OFFSET_X: f64 = 20.0;
pub const BUNKER_INITIAL_Y: f64 = 14.0;

pub const MYSTERY_SHIP_INITIAL_X: f64 = GAME_WIDTH + 1.0;
pub const MYSTERY_SHIP_INITIAL_Y: f64 = GAME_HEIGHT - MYSTERY_SHIP_HEIGHT;

pub const CANNON_LASER_INITIAL_Y: f64 = CANNON_HEIGHT + 1.0;
pub const CANNON_LASER_INITIAL_X_OFFSET: f64 = (CANNON_WIDTH / 2.0) - 1.0;

pub const GRID_INITIAL_X: f64 = 1.0;
pub const GRID_INITIAL_Y: f64 =
    MYSTERY_SHIP_INITIAL_Y - (5.0 * ROW_HEIGHT) - MYSTERY_SHIP_GRID_BUFFER;

pub const INVADER_MOVE_X: f64 = 2.0;
pub const INVADER_MOVE_Y: f64 = ROW_HEIGHT / 2.0;
pub const MYSTERY_SHIP_MOVE: f64 = 2.0;
pub const CANNON_MOVE: f64 = 2.0;
pub const LASER_MOVE: f64 = 2.0;

/// ⡄
pub static LASER: [(f64, f64); 2] = [(0.0, 0.0), (0.0, 1.0)];
