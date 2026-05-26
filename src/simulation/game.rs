use std::collections::HashMap;
use crate::effects::{PlayerId, CardId, Zone, Target};
use crate::player::{Player, ManaPool};
use crate::zones::{Zones, ZoneCard, Permanent};
use crate::stack::{Stack, StackItemId, StackObject, StackItem};
use crate::card::{Card, Color};

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

/// Represents the central Magic simulation kernel ("operating system").
/// Coordinates the players, game zones, stack, and transition instructions.
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub format: Format,
    pub active_player: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub zones: Zones,
    pub stack: Stack,
    /// Maps a CardId to the original ZoneCard for lookup across any zone transition.
    pub card_registry: HashMap<CardId, ZoneCard>,
    /// Maps StackItemIds to their targeted entities (e.g., Counterspell targeting a spell).
    pub targets: HashMap<StackItemId, Target>,
    pub turn_order: Vec<PlayerId>,
}

/// Represents a deterministic, kernel-level state-transition instruction ("machine code").
/// These instructions have a 1:1 correspondence with the lowest-level state modifications
/// of our individual game structures.
#[derive(Debug, Clone, PartialEq)]
pub enum SimInstruction {
    /// 1:1 with player's mana pool addition (Rule 106.1)
    AddMana {
        player_id: PlayerId,
        color: Color,
        amount: u32,
    },
    /// 1:1 with player's mana pool spending (Rule 106.4)
    SpendMana {
        player_id: PlayerId,
        color: Color,
        amount: u32,
    },
    /// 1:1 with Permanent status tap category (Rule 110.5)
    TapPermanent {
        card_id: CardId,
    },
    /// 1:1 with Permanent status untap category (Rule 110.5)
    UntapPermanent {
        card_id: CardId,
    },
    /// 1:1 with Zones registry atomic movements (Rule 400.7)
    MoveCard {
        card_id: CardId,
        from: Zone,
        to: Zone,
        controller: PlayerId,
    },
    /// 1:1 with Stack push spell (Rule 405.1 / 601.2)
    PushSpell {
        card_id: CardId,
        caster: PlayerId,
    },
    /// 1:1 with Stack pop spell (Rule 405.5)
    PopStack,
    /// 1:1 with Stack remove spell (Rule 701.5 - counter/remove)
    RemoveFromStack {
        stack_item_id: StackItemId,
    },
    /// 1:1 with targeting registry
    RegisterTarget {
        stack_item_id: StackItemId,
        target: Target,
    },
}

impl Game {
    /// Creates and registers a new game simulation kernel.
    pub fn new(format: Format) -> Self {
        Self {
            format,
            active_player: 1,
            players: HashMap::new(),
            zones: Zones::new(),
            stack: Stack::new(),
            card_registry: HashMap::new(),
            targets: HashMap::new(),
            turn_order: Vec::new(),
        }
    }

    /// Registers a player into the simulation.
    pub fn add_player(&mut self, player: Player) {
        let id = player.id;
        self.players.insert(id, player);
        self.zones.register_player(id);
        self.turn_order.push(id);
    }

    /// Helper to look up a card's name from our registry.
    pub fn get_registered_card_name(&self, card_id: CardId) -> String {
        if let Some(zc) = self.card_registry.get(&card_id) {
            get_card_name(&zc.card).to_string()
        } else {
            "Unknown Card".to_string()
        }
    }

    /// Executes a single, deterministic low-level "machine code" instruction and modifies the simulator states.
    pub fn execute_instruction(&mut self, instruction: SimInstruction) {
        println!("\x1b[36m[CPU INSTRUCTION]\x1b[0m {:?}", instruction);

        match instruction {
            SimInstruction::AddMana { player_id, color, amount } => {
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.mana_pool.add(color, amount);
                    println!(
                        "  -> \x1b[32mMana Added:\x1b[0m +{} {:?} to {}'s pool. (Current: {:?})",
                        amount, color, player.name, player.mana_pool
                    );
                }
            }

            SimInstruction::SpendMana { player_id, color, amount } => {
                if let Some(player) = self.players.get_mut(&player_id) {
                    let mut cost_pool = ManaPool::default();
                    cost_pool.add(color, amount);
                    if let Err(e) = player.mana_pool.spend(cost_pool) {
                        println!("  -> \x1b[31mError Spending Mana:\x1b[0m {} (Pool: {:?})", e, player.mana_pool);
                    } else {
                        println!(
                            "  -> \x1b[32mMana Spent:\x1b[0m -{} {:?} from {}'s pool. (Current: {:?})",
                            amount, color, player.name, player.mana_pool
                        );
                    }
                }
            }

            SimInstruction::TapPermanent { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.tap();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[33mPermanent Tapped:\x1b[0m '{}' (ID: {})", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent with ID {} not found on Battlefield!", card_id);
                }
            }

            SimInstruction::UntapPermanent { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.untap();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[32mPermanent Untapped:\x1b[0m '{}' (ID: {})", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent with ID {} not found on Battlefield!", card_id);
                }
            }

            SimInstruction::MoveCard { card_id, from, to, controller } => {
                let card_name = self.get_registered_card_name(card_id);
                if self.zones.move_card(card_id, from, to, controller) {
                    println!(
                        "  -> \x1b[32mCard Moved:\x1b[0m '{}' (ID: {}) moved from {:?} to {:?} under control of Player {}.",
                        card_name, card_id, from, to, controller
                    );
                } else {
                    // Try to insert card directly if it's currently on the stack or in limbo
                    if let Some(zc) = self.card_registry.get(&card_id).cloned() {
                        self.zones.insert_card(zc, to, controller);
                        println!(
                            "  -> \x1b[32mCard Inserted:\x1b[0m '{}' (ID: {}) placed into {:?} under control of Player {}.",
                            card_name, card_id, to, controller
                        );
                    } else {
                        println!("  -> \x1b[31mError Moving Card:\x1b[0m Card ID {} could not be found anywhere!", card_id);
                    }
                }
            }

            SimInstruction::PushSpell { card_id, caster } => {
                let stack_item_id = self.stack.push_spell(card_id, caster, vec![]);
                let card_name = self.get_registered_card_name(card_id);
                println!(
                    "  -> \x1b[35mStack Push:\x1b[0m '{}' (Card ID: {}) pushed onto Stack. (Allocated Stack Item ID: {})",
                    card_name, card_id, stack_item_id
                );
            }

            SimInstruction::PopStack => {
                if let Some(item) = self.stack.pop() {
                    match item.object {
                        StackObject::Spell { card_id, caster, .. } => {
                            let card_name = self.get_registered_card_name(card_id);
                            println!(
                                "  -> \x1b[35mStack Pop:\x1b[0m Popped '{}' (Card ID: {}) cast by Player {}.",
                                card_name, card_id, caster
                            );
                        }
                        _ => {
                            println!("  -> \x1b[35mStack Pop:\x1b[0m Popped ability from stack.");
                        }
                    }
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Attempted to pop empty stack!");
                }
            }

            SimInstruction::RemoveFromStack { stack_item_id } => {
                if let Some(item) = self.stack.remove(stack_item_id) {
                    match item.object {
                        StackObject::Spell { card_id, .. } => {
                            let card_name = self.get_registered_card_name(card_id);
                            println!(
                                "  -> \x1b[35mStack Remove:\x1b[0m Removed '{}' (Card ID: {}) from stack.",
                                card_name, card_id
                            );
                        }
                        _ => {
                            println!("  -> \x1b[35mStack Remove:\x1b[0m Removed ability from stack.");
                        }
                    }
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Stack item ID {} not found on stack!", stack_item_id);
                }
            }

            SimInstruction::RegisterTarget { stack_item_id, target } => {
                self.targets.insert(stack_item_id, target);
                println!("  -> \x1b[35mTarget Registered:\x1b[0m Stack Item {} targets {:?}", stack_item_id, target);
            }
        }
    }
}

/// Helper function to extract a card's name regardless of its enum type.
pub fn get_card_name(card: &Card) -> &str {
    match card {
        Card::Artifact(attrs) => &attrs.card.name,
        Card::Battle(attrs) => &attrs.card.name,
        Card::Creature(attrs) => &attrs.card.name,
        Card::Enchantment(attrs) => &attrs.card.name,
        Card::Instant(attrs) => &attrs.card.name,
        Card::Land(attrs) => &attrs.card.name,
        Card::Planeswalker(attrs) => &attrs.card.name,
        Card::Sorcery(attrs) => &attrs.card.name,
        Card::Kindred(attrs) => &attrs.card.name,
        Card::Leveler(attrs) => &attrs.card.name,
        Card::Saga(attrs) => &attrs.card.name,
        Card::Class(attrs) => &attrs.card.name,
    }
}