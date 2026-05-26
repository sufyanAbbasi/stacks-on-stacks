use std::collections::HashMap;
use crate::card::{Card, PermanentStatus};
use crate::effects::{CardId, PlayerId, Timestamp, Zone, Target};

/// Represents a card instance residing in a zone.
/// Pairs the Card's static definition with its unique runtime ID and actual owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneCard {
    pub id: CardId,
    pub card: Card,
    pub owner: PlayerId,
    pub is_token: bool,
}

/// --- SECTION 110: PERMANENTS ---
/// Represents an object on the battlefield (Rule 110.1).
/// A permanent remains on the battlefield indefinitely until moved to another zone by an effect or rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permanent {
    pub id: CardId,
    pub card: Card,
    pub owner: PlayerId,
    pub controller: PlayerId,
    pub status: PermanentStatus,
    /// The timestamp when this permanent entered the battlefield (Rule 613.7d).
    pub timestamp: Timestamp,
    /// Counters placed on this permanent (e.g. +1/+1 counters, loyalty counters) (Rule 122).
    pub counters: HashMap<String, u32>,
    /// Damage marked on this creature (Rule 120.3e / 510).
    pub damage_marked: u32,
    /// Aura, Equipment, or Fortification attachment target (Rule 301.5 / 303.4 / 305.7).
    pub attached_to: Option<Target>,
    /// True if this permanent is a token (Rule 111.1).
    pub is_token: bool,
}

impl Permanent {
    /// Returns true if the permanent is currently tapped (Rule 110.5).
    pub fn is_tapped(&self) -> bool {
        self.status.tapped
    }

    /// Taps the permanent (Rule 110.5).
    pub fn tap(&mut self) {
        self.status.tapped = true;
    }

    /// Untaps the permanent (Rule 110.5).
    pub fn untap(&mut self) {
        self.status.tapped = false;
    }

    /// Returns true if the permanent is currently flipped (Rule 110.5 / 709).
    pub fn is_flipped(&self) -> bool {
        self.status.flipped
    }

    /// Flips the permanent (Rule 110.5 / 709).
    pub fn flip(&mut self) {
        self.status.flipped = true;
    }

    /// Unflips the permanent (Rule 110.5 / 709).
    pub fn unflip(&mut self) {
        self.status.flipped = false;
    }

    /// Returns true if the permanent is currently face down (Rule 110.5 / 708).
    pub fn is_face_down(&self) -> bool {
        self.status.face_down
    }

    /// Returns true if the permanent is currently face up (Rule 110.5 / 708).
    pub fn is_face_up(&self) -> bool {
        !self.status.face_down
    }

    /// Sets the permanent's physical status to face down (Rule 110.5 / 708).
    pub fn set_face_down(&mut self) {
        self.status.face_down = true;
    }

    /// Sets the permanent's physical status to face up (Rule 110.5 / 708).
    pub fn set_face_up(&mut self) {
        self.status.face_down = false;
    }

    /// Returns true if the permanent is currently phased out (Rule 702.26).
    pub fn is_phased_out(&self) -> bool {
        self.status.phased_out
    }

    /// Phases the permanent in (Rule 702.26).
    pub fn phase_in(&mut self) {
        self.status.phased_out = false;
    }

    /// Phases the permanent out (Rule 702.26).
    pub fn phase_out(&mut self) {
        self.status.phased_out = true;
    }

    /// Adds counters to this permanent (Rule 122).
    pub fn add_counters(&mut self, counter_type: &str, amount: u32) {
        let count = self.counters.entry(counter_type.to_string()).or_insert(0);
        *count += amount;
    }

    /// Removes counters from this permanent, returning the number of counters actually removed (Rule 122).
    pub fn remove_counters(&mut self, counter_type: &str, amount: u32) -> u32 {
        if let Some(count) = self.counters.get_mut(counter_type) {
            if *count <= amount {
                let removed = *count;
                self.counters.remove(counter_type);
                removed
            } else {
                *count -= amount;
                amount
            }
        } else {
            0
        }
    }

    /// Marks damage on the permanent (Rule 120.3e).
    pub fn mark_damage(&mut self, amount: u32) {
        self.damage_marked += amount;
    }

    /// Clears all marked damage (Rule 510.5).
    pub fn clear_damage(&mut self) {
        self.damage_marked = 0;
    }

    /// Attaches this permanent to a target (Aura, Equipment, Fortification) (Rule 301.5 / 303.4 / 305.7).
    pub fn attach_to(&mut self, target: Target) {
        self.attached_to = Some(target);
    }

    /// Detaches this permanent (Rule 301.5 / 303.4).
    pub fn detach(&mut self) {
        self.attached_to = None;
    }
}

/// --- SECTION 114: EMBLEMS ---
/// An emblem is a noncard game object that exists only in the command zone (Rule 114.1).
/// It has one or more abilities, but no other characteristics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emblem {
    pub id: u32,
    pub owner: PlayerId,
    pub controller: PlayerId,
    pub rules_text: String,
    pub timestamp: Timestamp,
}

/// --- SECTION 406: EXILE ---
/// Exile is a shared zone (Rule 406.1).
/// Cards in exile may be kept face up or face down.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExiledCard {
    pub id: CardId,
    pub card: Card,
    pub owner: PlayerId,
    pub face_up: bool,
    /// Timestamp when the card entered exile.
    pub timestamp: Timestamp,
    pub is_token: bool,
}

/// --- SECTION 401: LIBRARY ---
/// A library is a player-specific zone.
/// It is kept face down in a single file, and its order is preserved (Rule 401.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Library {
    pub owner: PlayerId,
    pub cards: Vec<ZoneCard>,
}

impl Library {
    pub fn new(owner: PlayerId) -> Self {
        Self {
            owner,
            cards: Vec::new(),
        }
    }

    /// Draws a card from the top of the library (Rule 121 / 401.1).
    /// The top card is at the end of the vector.
    pub fn draw(&mut self) -> Option<ZoneCard> {
        self.cards.pop()
    }

    /// Puts a card on the top of the library.
    pub fn put_on_top(&mut self, zone_card: ZoneCard) {
        self.cards.push(zone_card);
    }

    /// Puts a card at the bottom of the library.
    pub fn put_on_bottom(&mut self, zone_card: ZoneCard) {
        self.cards.insert(0, zone_card);
    }

    /// Shuffles the library (Rule 701.20).
    pub fn shuffle(&mut self) {
        // Simple deterministic reverse as a placeholder for simulation testing.
        self.cards.reverse();
    }

    /// Searches the library for a specific card by its CardId (Rule 701.19).
    /// Returns the static Card definition if found, or None.
    pub fn search(&self, card_id: CardId) -> Option<Card> {
        self.cards.iter().find(|zc| zc.id == card_id).map(|zc| zc.card.clone())
    }
}

/// --- SECTION 402: HAND ---
/// A hand is a player-specific zone.
/// It contains a player's hand of cards (Rule 402.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    pub owner: PlayerId,
    pub cards: Vec<ZoneCard>,
}

impl Hand {
    pub fn new(owner: PlayerId) -> Self {
        Self {
            owner,
            cards: Vec::new(),
        }
    }

    /// Adds a card to the hand.
    pub fn add(&mut self, zone_card: ZoneCard) {
        self.cards.push(zone_card);
    }

    /// Removes a card from the hand by its card ID.
    pub fn remove_by_id(&mut self, card_id: CardId) -> Option<ZoneCard> {
        if let Some(pos) = self.cards.iter().position(|zc| zc.id == card_id) {
            Some(self.cards.remove(pos))
        } else {
            None
        }
    }
}

/// --- SECTION 403: BATTLEFIELD ---
/// The battlefield is a shared zone (Rule 403.1).
/// All players share the battlefield, which is where permanents reside.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Battlefield {
    pub permanents: Vec<Permanent>,
}

impl Battlefield {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a permanent to the battlefield.
    pub fn add_permanent(&mut self, permanent: Permanent) {
        self.permanents.push(permanent);
    }

    /// Removes a permanent from the battlefield.
    pub fn remove_permanent(&mut self, card_id: CardId) -> Option<Permanent> {
        if let Some(pos) = self.permanents.iter().position(|p| p.id == card_id) {
            Some(self.permanents.remove(pos))
        } else {
            None
        }
    }

    /// Gets a reference to a permanent.
    pub fn get_permanent(&self, card_id: CardId) -> Option<&Permanent> {
        self.permanents.iter().find(|p| p.id == card_id)
    }

    /// Gets a mutable reference to a permanent.
    pub fn get_permanent_mut(&mut self, card_id: CardId) -> Option<&mut Permanent> {
        self.permanents.iter_mut().find(|p| p.id == card_id)
    }
}

/// --- SECTION 404: GRAVEYARD ---
/// Each player has a graveyard (Rule 404.1).
/// A graveyard is a player-specific zone representing a face-up discard pile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graveyard {
    pub owner: PlayerId,
    pub cards: Vec<ZoneCard>,
}

impl Graveyard {
    pub fn new(owner: PlayerId) -> Self {
        Self {
            owner,
            cards: Vec::new(),
        }
    }

    /// Adds a card to the graveyard.
    pub fn add(&mut self, zone_card: ZoneCard) {
        self.cards.push(zone_card);
    }

    /// Removes a card from the graveyard by its card ID.
    pub fn remove_by_id(&mut self, card_id: CardId) -> Option<ZoneCard> {
        if let Some(pos) = self.cards.iter().position(|zc| zc.id == card_id) {
            Some(self.cards.remove(pos))
        } else {
            None
        }
    }
}

/// --- SECTION 406: EXILE ---
/// Exile is a shared zone.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Exile {
    pub objects: Vec<ExiledCard>,
}

impl Exile {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an exiled card object.
    pub fn add_exiled(&mut self, exiled: ExiledCard) {
        self.objects.push(exiled);
    }

    /// Removes an exiled card object by its card ID.
    pub fn remove_by_id(&mut self, card_id: CardId) -> Option<ExiledCard> {
        if let Some(pos) = self.objects.iter().position(|e| e.id == card_id) {
            Some(self.objects.remove(pos))
        } else {
            None
        }
    }

    /// Sets the face-up status of an exiled card (Rule 406.3).
    pub fn set_face_up(&mut self, card_id: CardId, face_up: bool) -> bool {
        if let Some(e) = self.objects.iter_mut().find(|obj| obj.id == card_id) {
            e.face_up = face_up;
            true
        } else {
            false
        }
    }
}

/// --- SECTION 407: COMMAND ZONE ---
/// The command zone is a shared zone (Rule 407.1).
/// It houses commanders, emblems, dungeon cards, and various other game-type items.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommandZone {
    pub commanders: Vec<ZoneCard>,
    pub emblems: Vec<Emblem>,
    pub other_objects: Vec<ZoneCard>,
}

impl CommandZone {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a commander card to the command zone.
    pub fn add_commander(&mut self, zone_card: ZoneCard) {
        self.commanders.push(zone_card);
    }

    /// Adds an emblem to the command zone.
    pub fn add_emblem(&mut self, emblem: Emblem) {
        self.emblems.push(emblem);
    }

    /// Removes a commander card by its card ID.
    pub fn remove_commander(&mut self, card_id: CardId) -> Option<ZoneCard> {
        if let Some(pos) = self.commanders.iter().position(|zc| zc.id == card_id) {
            Some(self.commanders.remove(pos))
        } else {
            None
        }
    }

    /// Removes an emblem by its emblem ID.
    pub fn remove_emblem(&mut self, emblem_id: u32) -> Option<Emblem> {
        if let Some(pos) = self.emblems.iter().position(|e| e.id == emblem_id) {
            Some(self.emblems.remove(pos))
        } else {
            None
        }
    }
}

/// --- COMPREHENSIVE ZONES REGISTRY ---
/// Tracks all player-specific and shared zones in the simulation game state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Zones {
    pub battlefield: Battlefield,
    pub exile: Exile,
    pub command_zone: CommandZone,
    pub libraries: HashMap<PlayerId, Library>,
    pub hands: HashMap<PlayerId, Hand>,
    pub graveyards: HashMap<PlayerId, Graveyard>,
    next_timestamp: Timestamp,
    next_emblem_id: u32,
}

impl Zones {
    /// Creates a new empty zones coordinator registry.
    pub fn new() -> Self {
        Self {
            next_timestamp: 1,
            next_emblem_id: 1,
            ..Self::default()
        }
    }

    /// Generates a unique, monotonically increasing timestamp for effects (Rule 613.7d).
    pub fn get_next_timestamp(&mut self) -> Timestamp {
        let ts = self.next_timestamp;
        self.next_timestamp += 1;
        ts
    }

    /// Generates a unique, monotonically increasing ID for a newly created emblem.
    pub fn get_next_emblem_id(&mut self) -> u32 {
        let id = self.next_emblem_id;
        self.next_emblem_id += 1;
        id
    }

    /// Registers player-specific zones for a newly added player.
    pub fn register_player(&mut self, player_id: PlayerId) {
        self.libraries.insert(player_id, Library::new(player_id));
        self.hands.insert(player_id, Hand::new(player_id));
        self.graveyards.insert(player_id, Graveyard::new(player_id));
    }

    /// Searches a player's library for a specific card by its CardId (Rule 701.19).
    /// Returns the static Card definition if found, or None.
    pub fn search_library(&self, player_id: PlayerId, card_id: CardId) -> Option<Card> {
        self.libraries.get(&player_id).and_then(|lib| lib.search(card_id))
    }

    /// Creates a new emblem in the command zone for a player (Rule 114).
    pub fn create_emblem(&mut self, controller: PlayerId, rules_text: String) -> u32 {
        let id = self.get_next_emblem_id();
        let timestamp = self.get_next_timestamp();
        self.command_zone.add_emblem(Emblem {
            id,
            owner: controller,
            controller,
            rules_text,
            timestamp,
        });
        id
    }

    /// Creates a token permanent on the battlefield (Rule 111).
    /// Assigns it a unique CardId, is_token: true, and registers its controller as the owner (Rule 111.4).
    pub fn create_token(&mut self, id: CardId, card: Card, controller: PlayerId) {
        let timestamp = self.get_next_timestamp();
        self.battlefield.add_permanent(Permanent {
            id,
            card,
            owner: controller, // Rule 111.4: A token's owner is the player under whose control it entered.
            controller,
            status: PermanentStatus::default(),
            timestamp,
            counters: HashMap::new(),
            damage_marked: 0,
            attached_to: None,
            is_token: true,
        });
    }

    /// Extracts a card from its current zone (if found) (Rule 400.7).
    /// Once extracted, the card ceases to exist in that zone.
    /// Extracts a card from its current zone (if found) (Rule 400.7).
    /// Once extracted, the card ceases to exist in that zone.
    /// Under Rule 111.8, a token that has left the battlefield cannot move to another zone.
    pub fn extract_card(&mut self, card_id: CardId, from: Zone) -> Option<ZoneCard> {
        match from {
            Zone::Library => {
                for library in self.libraries.values_mut() {
                    if let Some(pos) = library.cards.iter().position(|zc| zc.id == card_id) {
                        if library.cards[pos].is_token {
                            return None; // Rule 111.8: remains in current zone instead
                        }
                        return Some(library.cards.remove(pos));
                    }
                }
                None
            }
            Zone::Hand => {
                for hand in self.hands.values_mut() {
                    if let Some(pos) = hand.cards.iter().position(|zc| zc.id == card_id) {
                        if hand.cards[pos].is_token {
                            return None; // Rule 111.8: remains in current zone instead
                        }
                        return Some(hand.cards.remove(pos));
                    }
                }
                None
            }
            Zone::Graveyard => {
                for graveyard in self.graveyards.values_mut() {
                    if let Some(pos) = graveyard.cards.iter().position(|zc| zc.id == card_id) {
                        if graveyard.cards[pos].is_token {
                            return None; // Rule 111.8: remains in current zone instead
                        }
                        return Some(graveyard.cards.remove(pos));
                    }
                }
                None
            }
            Zone::Battlefield => {
                self.battlefield.remove_permanent(card_id).map(|p| ZoneCard {
                    id: p.id,
                    card: p.card,
                    owner: p.owner,
                    is_token: p.is_token,
                })
            }
            Zone::Exile => {
                if let Some(pos) = self.exile.objects.iter().position(|e| e.id == card_id) {
                    if self.exile.objects[pos].is_token {
                        return None; // Rule 111.8: remains in current zone instead
                    }
                    let e = self.exile.objects.remove(pos);
                    Some(ZoneCard {
                        id: e.id,
                        card: e.card,
                        owner: e.owner,
                        is_token: e.is_token,
                    })
                } else {
                    None
                }
            }
            Zone::Command => {
                if let Some(pos) = self.command_zone.commanders.iter().position(|zc| zc.id == card_id) {
                    if self.command_zone.commanders[pos].is_token {
                        return None;
                    }
                    Some(self.command_zone.commanders.remove(pos))
                } else if let Some(pos) = self.command_zone.other_objects.iter().position(|zc| zc.id == card_id) {
                    if self.command_zone.other_objects[pos].is_token {
                        return None;
                    }
                    Some(self.command_zone.other_objects.remove(pos))
                } else {
                    None
                }
            }
            Zone::Stack => {
                // Stack is managed separately by the Stack struct.
                None
            }
        }
    }

    /// Inserts a ZoneCard into its destination zone.
    /// Handles timestamp registration for Battlefield and Exile entries.
    pub fn insert_card(&mut self, zone_card: ZoneCard, to: Zone, controller: PlayerId) {
        match to {
            Zone::Library => {
                if let Some(lib) = self.libraries.get_mut(&zone_card.owner) {
                    lib.put_on_top(zone_card);
                }
            }
            Zone::Hand => {
                if let Some(h) = self.hands.get_mut(&zone_card.owner) {
                    h.add(zone_card);
                }
            }
            Zone::Graveyard => {
                if let Some(g) = self.graveyards.get_mut(&zone_card.owner) {
                    g.add(zone_card);
                }
            }
            Zone::Battlefield => {
                let timestamp = self.get_next_timestamp();
                self.battlefield.add_permanent(Permanent {
                    id: zone_card.id,
                    card: zone_card.card,
                    owner: zone_card.owner,
                    controller,
                    status: PermanentStatus::default(),
                    timestamp,
                    counters: HashMap::new(),
                    damage_marked: 0,
                    attached_to: None,
                    is_token: zone_card.is_token,
                });
            }
            Zone::Exile => {
                let timestamp = self.get_next_timestamp();
                self.exile.add_exiled(ExiledCard {
                    id: zone_card.id,
                    card: zone_card.card,
                    owner: zone_card.owner,
                    face_up: true,
                    timestamp,
                    is_token: zone_card.is_token,
                });
            }
            Zone::Command => {
                // If it is a commander, place it in the commanders list.
                // Otherwise, place it in the other_objects list.
                self.command_zone.commanders.push(zone_card);
            }
            Zone::Stack => {
                // Stack is managed separately by Stack zone in stack.rs.
            }
        }
    }

    /// Performs an atomic, zone-to-zone card movement (Rule 400.7).
    /// Returns true if the card was successfully found and moved.
    pub fn move_card(&mut self, card_id: CardId, from: Zone, to: Zone, controller: PlayerId) -> bool {
        if let Some(zc) = self.extract_card(card_id, from) {
            self.insert_card(zc, to, controller);
            true
        } else {
            false
        }
    }

    /// Automatically removes any tokens from zones other than the battlefield (Rule 111.7).
    pub fn clean_up_tokens(&mut self) -> Vec<CardId> {
        let mut removed_ids = Vec::new();

        // 1. Clean libraries
        for library in self.libraries.values_mut() {
            let mut i = 0;
            while i < library.cards.len() {
                if library.cards[i].is_token {
                    let removed = library.cards.remove(i);
                    removed_ids.push(removed.id);
                } else {
                    i += 1;
                }
            }
        }

        // 2. Clean hands
        for hand in self.hands.values_mut() {
            let mut i = 0;
            while i < hand.cards.len() {
                if hand.cards[i].is_token {
                    let removed = hand.cards.remove(i);
                    removed_ids.push(removed.id);
                } else {
                    i += 1;
                }
            }
        }

        // 3. Clean graveyards
        for graveyard in self.graveyards.values_mut() {
            let mut i = 0;
            while i < graveyard.cards.len() {
                if graveyard.cards[i].is_token {
                    let removed = graveyard.cards.remove(i);
                    removed_ids.push(removed.id);
                } else {
                    i += 1;
                }
            }
        }

        // 4. Clean exile
        let mut i = 0;
        while i < self.exile.objects.len() {
            if self.exile.objects[i].is_token {
                let removed = self.exile.objects.remove(i);
                removed_ids.push(removed.id);
            } else {
                i += 1;
            }
        }

        // 5. Clean command zone
        let mut i = 0;
        while i < self.command_zone.commanders.len() {
            if self.command_zone.commanders[i].is_token {
                let removed = self.command_zone.commanders.remove(i);
                removed_ids.push(removed.id);
            } else {
                i += 1;
            }
        }
        let mut i = 0;
        while i < self.command_zone.other_objects.len() {
            if self.command_zone.other_objects[i].is_token {
                let removed = self.command_zone.other_objects.remove(i);
                removed_ids.push(removed.id);
            } else {
                i += 1;
            }
        }

        removed_ids
    }
}
