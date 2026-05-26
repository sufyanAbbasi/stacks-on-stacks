use std::collections::HashMap;
use crate::card::Card;

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub life_total: u128,
    pub commander_damage: HashMap<u32, u32>, // Maps other player's ID to commander damage dealt by them
    pub counters: HashMap<String, u32>,
    pub library: Vec<Card>,
    pub graveyard: Vec<Card>,
    pub exile: Vec<Card>,
    pub command_zone: Option<Card>,
    pub sideboard: Vec<Card>,
    pub hand_size: Option<u8>,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            life_total: 20, // Default starting life total (or 40 for Commander)
            commander_damage: HashMap::new(),
            counters: HashMap::new(),
            library: Vec::new(),
            graveyard: Vec::new(),
            exile: Vec::new(),
            command_zone: None,
            sideboard: Vec::new(),
            hand_size: Some(7),
            hand: Vec::new(),
        }
    }
}