//!

use tui::style::Color;
use tui::widgets::canvas::{Painter, Shape};

use crate::points::letters;

const ROW_HEIGHT: f64 = letters::LETTER_HEIGHT + letters::LETTER_SPACING_Y;

///
#[derive(Clone, Debug, PartialEq)]
pub struct Words {
    rows: Vec<Row>,
    color: Color,
}

impl Words {
    ///
    pub fn game_over() -> Self {
        Self {
            rows: vec![
                Row::new(vec![Letter::O, Letter::V, Letter::E, Letter::R]),
                Row::new(vec![Letter::G, Letter::A, Letter::M, Letter::E]),
            ],
            color: Color::Red,
        }
    }

    ///
    pub fn space_invaders() -> Self {
        Self {
            rows: vec![
                Row::new(vec![
                    Letter::I,
                    Letter::N,
                    Letter::V,
                    Letter::A,
                    Letter::D,
                    Letter::E,
                    Letter::R,
                    Letter::S,
                ]),
                Row::new(vec![Letter::S, Letter::P, Letter::A, Letter::C, Letter::E]),
            ],
            color: Color::Yellow,
        }
    }

    ///
    pub fn height(&self) -> f64 {
        let num_rows = self.rows.len();

        ((num_rows as f64) * letters::LETTER_HEIGHT)
            + (((num_rows - 1) as f64) * letters::LETTER_SPACING_Y)
    }

    ///
    pub fn width(&self) -> f64 {
        let mut width = 0.0;

        for row in self.rows.iter() {
            let row_width = row.width();

            if row_width > width {
                width = row_width;
            }
        }

        width
    }
}

impl Shape for Words {
    fn draw(&self, painter: &mut Painter) {
        let max_width = self.width();

        for (i, row) in self.rows.iter().enumerate() {
            row.draw(painter, max_width, i as f64 * ROW_HEIGHT, self.color);
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
struct Row {
    letters: Vec<Letter>,
}

impl Row {
    ///
    pub fn new(letters: Vec<Letter>) -> Self {
        Self { letters }
    }

    ///
    fn width(&self) -> f64 {
        let mut width = (self.letters.len() - 1) as f64 * letters::LETTER_SPACING_X;

        for letter in self.letters.iter() {
            width += letter.width();
        }

        width
    }

    pub fn draw(&self, painter: &mut Painter, row_width: f64, y_offset: f64, color: Color) {
        let row_x_offset = (row_width - self.width()) / 2.0;
        let mut letter_x_offset = 0.0;

        for letter in self.letters.iter() {
            for (x, y) in letter.data() {
                let x = x + letter_x_offset + row_x_offset;
                let y = y + y_offset;

                if let Some((x, y)) = painter.get_point(x, y) {
                    painter.paint(x, y, color);
                }
            }

            letter_x_offset += letter.width() + letters::LETTER_SPACING_X;
        }
    }
}

///
#[derive(Copy, Clone, Debug, PartialEq)]
enum Letter {
    A,
    C,
    D,
    E,
    G,
    I,
    M,
    N,
    O,
    P,
    R,
    S,
    V,
}

impl Letter {
    ///
    pub fn data(&self) -> &'static [(f64, f64)] {
        match self {
            Self::A => &letters::A,
            Self::C => &letters::C,
            Self::D => &letters::D,
            Self::E => &letters::E,
            Self::G => &letters::G,
            Self::I => &letters::I,
            Self::M => &letters::M,
            Self::N => &letters::N,
            Self::O => &letters::O,
            Self::P => &letters::P,
            Self::R => &letters::R,
            Self::S => &letters::S,
            Self::V => &letters::V,
        }
    }

    ///
    pub fn width(&self) -> f64 {
        match self {
            Self::A => letters::A_WIDTH,
            Self::C => letters::C_WIDTH,
            Self::D => letters::D_WIDTH,
            Self::E => letters::E_WIDTH,
            Self::G => letters::G_WIDTH,
            Self::I => letters::I_WIDTH,
            Self::M => letters::M_WIDTH,
            Self::N => letters::N_WIDTH,
            Self::O => letters::O_WIDTH,
            Self::P => letters::P_WIDTH,
            Self::R => letters::R_WIDTH,
            Self::S => letters::S_WIDTH,
            Self::V => letters::V_WIDTH,
        }
    }
}
