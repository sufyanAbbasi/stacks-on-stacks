use crate::player::Player;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Standard,
    Pioneer,
    Historic,
    Modern,
    Legacy,
    Vintage,
    Pauper,
    Commander,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub format: Format,
    pub active_player: Player,
    pub turn_order: Vec<Player>,
}

impl Game {
    pub fn new(format: Format, active_player: Player, turn_order: Vec<Player>) -> Self {
        Self {
            format,
            active_player,
            turn_order,
        }
    }
}