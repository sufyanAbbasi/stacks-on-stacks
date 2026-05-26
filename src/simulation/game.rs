use std::collections::HashMap;
use crate::effects::{PlayerId, CardId, Zone, Target};
use crate::player::{Player, ManaPool};
use crate::zones::{Zones, ZoneCard};
use crate::stack::{Stack, StackItemId, StackObject};
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
    /// 1:1 with Library draw (Rule 121 / 401.1)
    DrawCard {
        player_id: PlayerId,
    },
    /// 1:1 with Library shuffle (Rule 701.20)
    ShuffleLibrary {
        player_id: PlayerId,
    },
    /// 1:1 with Library search (Rule 701.19)
    SearchLibrary {
        player_id: PlayerId,
        card_id: CardId,
    },
    /// 1:1 with Permanent status flip category (Rule 110.5 / 709)
    FlipPermanent {
        card_id: CardId,
    },
    /// 1:1 with Permanent status flip category (Rule 110.5 / 709)
    UnflipPermanent {
        card_id: CardId,
    },
    /// 1:1 with Permanent status face category (Rule 110.5 / 708)
    TurnPermanentFaceDown {
        card_id: CardId,
    },
    /// 1:1 with Permanent status face category (Rule 110.5 / 708)
    TurnPermanentFaceUp {
        card_id: CardId,
    },
    /// 1:1 with Permanent status phasing category (Rule 702.26)
    PhaseInPermanent {
        card_id: CardId,
    },
    /// 1:1 with Permanent status phasing category (Rule 702.26)
    PhaseOutPermanent {
        card_id: CardId,
    },
    /// 1:1 with Permanent counters additions (Rule 122)
    AddCounter {
        card_id: CardId,
        counter_type: String,
        amount: u32,
    },
    /// 1:1 with Permanent counters removals (Rule 122)
    RemoveCounter {
        card_id: CardId,
        counter_type: String,
        amount: u32,
    },
    /// 1:1 with Permanent damage marking (Rule 120.3e)
    MarkDamage {
        card_id: CardId,
        amount: u32,
    },
    /// 1:1 with Permanent damage clearing (Rule 510.5)
    ClearDamage {
        card_id: CardId,
    },
    /// 1:1 with Permanent attachment (Rule 301.5 / 303.4)
    AttachPermanent {
        card_id: CardId,
        target: Target,
    },
    /// 1:1 with Permanent detachment (Rule 301.5 / 303.4)
    DetachPermanent {
        card_id: CardId,
    },
    /// 1:1 with Exile face up/down status (Rule 406)
    SetExiledFaceUp {
        card_id: CardId,
        face_up: bool,
    },
    /// 1:1 with Command Zone emblem creation (Rule 114)
    CreateEmblem {
        controller: PlayerId,
        rules_text: String,
    },
    /// 1:1 with Token creation (Rule 111)
    CreateToken {
        controller: PlayerId,
        token_id: CardId,
        card: Card,
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
                if let Some(item) = self.stack.items.iter_mut().find(|i| i.id == stack_item_id) {
                    item.targets.push(target);
                    println!("  -> \x1b[35mTarget Registered on Stack:\x1b[0m Stack Item {} targets {:?}", stack_item_id, target);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Stack Item ID {} not found on stack to register target!", stack_item_id);
                }
            }

            SimInstruction::DrawCard { player_id } => {
                if let Some(lib) = self.zones.libraries.get_mut(&player_id) {
                    if let Some(zc) = lib.draw() {
                        let card_name = get_card_name(&zc.card).to_string();
                        let id = zc.id;
                        self.zones.insert_card(zc, Zone::Hand, player_id);
                        println!(
                            "  -> \x1b[32mCard Drawn:\x1b[0m '{}' (ID: {}) drawn by Player {}.",
                            card_name, id, player_id
                        );
                    } else {
                        println!("  -> \x1b[31mError:\x1b[0m Player {} has no cards left in library to draw! (Rule 121.4)", player_id);
                    }
                }
            }

            SimInstruction::ShuffleLibrary { player_id } => {
                if let Some(lib) = self.zones.libraries.get_mut(&player_id) {
                    lib.shuffle();
                    println!("  -> \x1b[32mLibrary Shuffled:\x1b[0m Player {} shuffled their library.", player_id);
                }
            }

            SimInstruction::SearchLibrary { player_id, card_id } => {
                if let Some(card) = self.zones.search_library(player_id, card_id) {
                    let card_name = get_card_name(&card);
                    println!(
                        "  -> \x1b[32mLibrary Search:\x1b[0m Player {} searched library and found '{}' (ID: {}).",
                        player_id, card_name, card_id
                    );
                } else {
                    println!(
                        "  -> \x1b[31mLibrary Search:\x1b[0m Player {} searched library for ID {} but did NOT find it.",
                        player_id, card_id
                    );
                }
            }

            SimInstruction::FlipPermanent { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.flip();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[33mPermanent Flipped:\x1b[0m '{}' (ID: {}) flipped.", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::UnflipPermanent { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.unflip();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[32mPermanent Unflipped:\x1b[0m '{}' (ID: {}) unflipped.", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::TurnPermanentFaceDown { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.set_face_down();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[33mPermanent turned Face Down:\x1b[0m '{}' (ID: {}) is now face down.", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::TurnPermanentFaceUp { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.set_face_up();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[32mPermanent turned Face Up:\x1b[0m '{}' (ID: {}) is now face up.", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::PhaseInPermanent { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.phase_in();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[32mPermanent Phased In:\x1b[0m '{}' (ID: {}) phased in.", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::PhaseOutPermanent { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.phase_out();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[33mPermanent Phased Out:\x1b[0m '{}' (ID: {}) phased out.", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::AddCounter { card_id, counter_type, amount } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.add_counters(&counter_type, amount);
                    let name = get_card_name(&p.card);
                    println!(
                        "  -> \x1b[32mCounter Added:\x1b[0m Added {} {:?} counters to '{}' (ID: {}).",
                        amount, counter_type, name, card_id
                    );
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::RemoveCounter { card_id, counter_type, amount } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    let removed = p.remove_counters(&counter_type, amount);
                    let name = get_card_name(&p.card);
                    println!(
                        "  -> \x1b[33mCounter Removed:\x1b[0m Removed {}/{} {:?} counters from '{}' (ID: {}).",
                        removed, amount, counter_type, name, card_id
                    );
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::MarkDamage { card_id, amount } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.mark_damage(amount);
                    let name = get_card_name(&p.card);
                    println!(
                        "  -> \x1b[31mDamage Marked:\x1b[0m Marked {} damage on '{}' (ID: {}). Total Marked: {}.",
                        amount, name, card_id, p.damage_marked
                    );
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::ClearDamage { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.clear_damage();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[32mDamage Cleared:\x1b[0m Cleared all marked damage from '{}' (ID: {}).", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::AttachPermanent { card_id, target } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.attach_to(target);
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[32mPermanent Attached:\x1b[0m '{}' (ID: {}) attached to {:?}", name, card_id, target);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::DetachPermanent { card_id } => {
                if let Some(p) = self.zones.battlefield.get_permanent_mut(card_id) {
                    p.detach();
                    let name = get_card_name(&p.card);
                    println!("  -> \x1b[32mPermanent Detached:\x1b[0m '{}' (ID: {}) detached.", name, card_id);
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Permanent ID {} not found on battlefield!", card_id);
                }
            }

            SimInstruction::SetExiledFaceUp { card_id, face_up } => {
                if self.zones.exile.set_face_up(card_id, face_up) {
                    let card_name = self.get_registered_card_name(card_id);
                    println!(
                        "  -> \x1b[32mExile Visibility Changed:\x1b[0m Exiled Card '{}' (ID: {}) set to face_up = {}.",
                        card_name, card_id, face_up
                    );
                } else {
                    println!("  -> \x1b[31mError:\x1b[0m Card ID {} not found in Exile!", card_id);
                }
            }

            SimInstruction::CreateEmblem { controller, rules_text } => {
                let id = self.zones.create_emblem(controller, rules_text.clone());
                println!(
                    "  -> \x1b[35mEmblem Created:\x1b[0m New Emblem (ID: {}) created in Player {}'s Command Zone. Rules: '{}'",
                    id, controller, rules_text
                );
            }

            SimInstruction::CreateToken { controller, token_id, card } => {
                let card_name = get_card_name(&card).to_string();
                self.zones.create_token(token_id, card, controller);
                println!(
                    "  -> \x1b[35mToken Created:\x1b[0m Created '{}' token (ID: {}) on battlefield under Player {}'s control.",
                    card_name, token_id, controller
                );
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