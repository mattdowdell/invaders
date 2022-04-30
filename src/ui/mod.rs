//!

mod game;
mod start;
mod util;

use tui::{backend::Backend, terminal::Frame};

use crate::app::App;

const APP_TITLE: &str = "Space Invaders";
const SCORE_TITLE: &str = "Score";
const HISCORE_TITLE: &str = "Hiscore";

const HELP_TAB_TEXT: &str = "[H]elp";
const PAUSE_TAB_TEXT: &str = "[P]ause";
const QUIT_TAB_TEXT: &str = "[Q]uit";

const VERTICAL_DOTS_PER_CHAR: u16 = 4;
const HORIZONTAL_DOTS_PER_CHAR: u16 = 2;

const BORDER_WIDTH: u16 = 1;
const ROW_HEIGHT: u16 = 1 + (2 * BORDER_WIDTH);
const GAME_WIDTH: u16 = 100 + (2 * BORDER_WIDTH);
const GAME_HEIGHT: u16 = 33 + (2 * BORDER_WIDTH);

const APP_HEIGHT: u16 = (3 * ROW_HEIGHT) + GAME_HEIGHT;
const APP_WIDTH: u16 = GAME_WIDTH;

const HELP_WIDTH: u16 = 25 + (2 * BORDER_WIDTH);
const HELP_HEIGHT: u16 = 6 + (2 * BORDER_WIDTH);

const PAUSE_WIDTH: u16 = 18 + (2 * BORDER_WIDTH);
const PAUSE_HEIGHT: u16 = 1 + (2 * BORDER_WIDTH);

///
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    // TODO: output help message if size is insufficient, otherwise we panic

    if !app.started {
        start::draw_start_screen(f);
    } else {
        game::draw_game_screen(f, app);
    }
}
