use macroquad::color::{self, Color};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X = 0,
    O = 1,
}

impl Player {
    pub fn other(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Player::O => color::BLUE,
            Player::X => color::GRAY,
        }
    }
}
