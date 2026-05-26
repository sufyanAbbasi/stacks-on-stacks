use std::collections::HashMap;
use crate::card::{Card, Color};

/// Models a player's mana pool tracking colored and colorless mana (Section 106.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManaPool {
    pub white: u32,
    pub blue: u32,
    pub black: u32,
    pub red: u32,
    pub green: u32,
    pub colorless: u32,
}

impl ManaPool {
    /// Creates a new empty mana pool.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a given amount of mana of a specific color (or colorless).
    pub fn add(&mut self, color: Color, amount: u32) {
        match color {
            Color::W => self.white += amount,
            Color::U => self.blue += amount,
            Color::B => self.black += amount,
            Color::R => self.red += amount,
            Color::G => self.green += amount,
            Color::C => self.colorless += amount,
        }
    }

    /// Tries to spend a specific combination of mana.
    /// Returns Ok(()) if successful, or Err if there is insufficient mana of any category.
    pub fn spend(&mut self, cost: ManaPool) -> Result<(), &'static str> {
        if self.can_pay(cost) {
            self.white -= cost.white;
            self.blue -= cost.blue;
            self.black -= cost.black;
            self.red -= cost.red;
            self.green -= cost.green;
            self.colorless -= cost.colorless;
            Ok(())
        } else {
            Err("Insufficient mana in pool to cover the specified cost")
        }
    }

    /// Checks if the mana pool contains at least the specified combination of mana.
    pub fn can_pay(&self, cost: ManaPool) -> bool {
        self.white >= cost.white
            && self.blue >= cost.blue
            && self.black >= cost.black
            && self.red >= cost.red
            && self.green >= cost.green
            && self.colorless >= cost.colorless
    }

    /// Empties the mana pool (Rule 106.4).
    pub fn clear(&mut self) {
        self.white = 0;
        self.blue = 0;
        self.black = 0;
        self.red = 0;
        self.green = 0;
        self.colorless = 0;
    }

    /// Checks if the mana pool is completely empty.
    pub fn is_empty(&self) -> bool {
        self.white == 0
            && self.blue == 0
            && self.black == 0
            && self.red == 0
            && self.green == 0
            && self.colorless == 0
    }

    /// Returns the total amount of mana in the pool.
    pub fn total(&self) -> u32 {
        self.white + self.blue + self.black + self.red + self.green + self.colorless
    }

    /// Checks if the total mana in the pool is sufficient to pay a generic mana cost of the given amount.
    pub fn can_pay_generic(&self, amount: u32) -> bool {
        self.total() >= amount
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub life_total: u128,
    pub commander_damage: HashMap<u32, u32>, // Maps other player's ID to commander damage dealt by them
    pub counters: HashMap<String, u32>,
    pub sideboard: Vec<Card>,
    pub hand_size: Option<u8>,
    pub mana_pool: ManaPool,
}

impl Player {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            life_total: 20, // Default starting life total (or 40 for Commander)
            commander_damage: HashMap::new(),
            counters: HashMap::new(),
            sideboard: Vec::new(),
            hand_size: Some(7),
            mana_pool: ManaPool::default(),
        }
    }
}