enum Format {
    Standard,
    Pioneer,
    Historic,
    Modern,
    Legacy,
    Vintage,
    Pauper,
    Commander,
}

struct Game {
    format: Format,
    active_player: Player,
    turn_order: Vec<Player>,
}