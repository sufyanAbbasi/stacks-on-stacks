use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::effects::{PlayerId, CardId, Zone, Target};
use crate::player::{Player, ManaPool};
use crate::zones::{Zones, ZoneCard};
use crate::stack::{Stack, StackItemId, StackObject};
use crate::card::{Card, Color};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Game {
    pub format: Format,
    pub active_player: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub zones: Zones,
    pub stack: Stack,
    /// Maps a CardId to the original ZoneCard for lookup across any zone transition.
    pub card_registry: HashMap<CardId, ZoneCard>,
    pub turn_order: Vec<PlayerId>,
    pub active_effects: crate::effects::ActiveEffects,
    pub priority_player: Option<PlayerId>,
    pub consecutive_passes: u32,
}

/// Represents a deterministic, kernel-level state-transition instruction ("machine code").
/// These instructions have a 1:1 correspondence with the lowest-level state modifications
/// of our individual game structures.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
            active_effects: crate::effects::ActiveEffects::new(),
            priority_player: Some(1),
            consecutive_passes: 0,
        }
    }

    /// Registers a player into the simulation.
    pub fn add_player(&mut self, player: Player) {
        let id = player.id;
        self.players.insert(id, player);
        self.zones.register_player(id);
        self.turn_order.push(id);
        
        // If this is the first player added, make them the active and priority player by default
        if self.players.len() == 1 {
            self.active_player = id;
            self.priority_player = Some(id);
        }
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

/// Helper function to get a mutable reference to card attributes regardless of its variant
pub fn get_card_attributes_mut(card: &mut Card) -> &mut crate::card::CardAttributes {
    match card {
        Card::Artifact(attrs) => &mut attrs.card,
        Card::Battle(attrs) => &mut attrs.card,
        Card::Creature(attrs) => &mut attrs.card,
        Card::Enchantment(attrs) => &mut attrs.card,
        Card::Instant(attrs) => &mut attrs.card,
        Card::Land(attrs) => &mut attrs.card,
        Card::Planeswalker(attrs) => &mut attrs.card,
        Card::Sorcery(attrs) => &mut attrs.card,
        Card::Kindred(attrs) => &mut attrs.card,
        Card::Leveler(attrs) => &mut attrs.card,
        Card::Saga(attrs) => &mut attrs.card,
        Card::Class(attrs) => &mut attrs.card,
    }
}

impl Game {
    /// Applies active long-lived continuous effects to a Card in layer order (Rule 613).
    pub fn apply_active_effects(&self, card_id: CardId, mut card: Card) -> Card {
        let sorted_effects = self.active_effects.get_sorted_continuous_effects();
        for effect in sorted_effects {
            // Check if the effect applies to this card.
            // For our abstract model, we assume the effect applies if the effect source is this card,
            // or if it's targeted, or if we do a mock mapping (matching odd/even IDs for demo).
            if effect.source == card_id || effect.id % 2 == card_id % 2 {
                match effect.effect {
                    crate::effects::ContinuousEffectType::ModifyPowerToughness { power_offset, toughness_offset } => {
                        if let Card::Creature(ref mut attrs) = card {
                            let p: i32 = attrs.power.parse().unwrap_or(0);
                            let t: i32 = attrs.toughness.parse().unwrap_or(0);
                            attrs.power = (p + power_offset).max(0).to_string();
                            attrs.toughness = (t + toughness_offset).max(0).to_string();
                        }
                    }
                    crate::effects::ContinuousEffectType::SetPowerToughness { power_base, toughness_base } => {
                        if let Card::Creature(ref mut attrs) = card {
                            attrs.power = power_base.to_string();
                            attrs.toughness = toughness_base.to_string();
                        }
                    }
                    crate::effects::ContinuousEffectType::AddAbility(ref ability) => {
                        let attrs = get_card_attributes_mut(&mut card);
                        if !attrs.rules_text.is_empty() {
                            attrs.rules_text.push_str("\n");
                        }
                        attrs.rules_text.push_str(ability);
                    }
                    crate::effects::ContinuousEffectType::RemoveAbility(ref ability) => {
                        let attrs = get_card_attributes_mut(&mut card);
                        attrs.rules_text = attrs.rules_text.replace(ability, "");
                    }
                    crate::effects::ContinuousEffectType::ChangeColor(color) => {
                        if let Card::Creature(ref mut attrs) = card {
                            attrs.spell.color = vec![color];
                        } else if let Card::Artifact(ref mut attrs) = card {
                            attrs.spell.color = vec![color];
                        }
                    }
                    _ => {}
                }
            }
        }
        card
    }

    /// Run the Magic State-Based Action trigger-resolution loop (Rule 704).
    /// Loops continuously until a stable state is reached where no more SBAs trigger.
    pub fn check_state_based_actions(&mut self) -> Vec<String> {
        let mut sba_logs = Vec::new();
        let mut executed_any = true;
        let mut loop_count = 0;

        while executed_any && loop_count < 10 {
            executed_any = false;
            loop_count += 1;

            // 1. Check Player Life Total <= 0 (Rule 704.5a)
            let mut players_to_lose = Vec::new();
            for (&player_id, player) in self.players.iter() {
                if player.life_total <= 0 {
                    players_to_lose.push(player_id);
                }
            }
            for pid in players_to_lose {
                if self.players.contains_key(&pid) {
                    let name = self.players.get(&pid).unwrap().name.clone();
                    sba_logs.push(format!("Rule 704.5a: Player {} (ID {}) has 0 or less life and loses the game.", name, pid));
                    self.players.remove(&pid);
                    self.turn_order.retain(|&x| x != pid);
                    executed_any = true;
                }
            }

            // 2. Check Creature toughness <= 0 (Rule 704.5f) and marked damage >= toughness (Rule 704.5g)
            let mut creatures_to_graveyard = Vec::new();
            let mut creatures_to_destroy = Vec::new();

            for p in self.zones.battlefield.permanents.iter() {
                let card_effective = self.apply_active_effects(p.id, p.card.clone());
                if let Card::Creature(ref attrs) = card_effective {
                    let toughness: i32 = attrs.toughness.parse().unwrap_or(0);
                    if toughness <= 0 {
                        creatures_to_graveyard.push((p.id, p.controller));
                    } else if p.damage_marked >= toughness as u32 {
                        creatures_to_destroy.push((p.id, p.controller));
                    }
                }
            }

            for (card_id, controller) in creatures_to_graveyard {
                sba_logs.push(format!(
                    "Rule 704.5f: Creature (ID {}) has toughness 0 or less and is put into its owner's graveyard.",
                    card_id
                ));
                // Note: this happens regardless of any replacement effects. SBAs always get processed first.
                self.execute_instruction(SimInstruction::MoveCard {
                    card_id,
                    from: Zone::Battlefield,
                    to: Zone::Graveyard,
                    controller,
                });
                executed_any = true;
            }

            for (card_id, controller) in creatures_to_destroy {
                sba_logs.push(format!(
                    "Rule 704.5g: Creature (ID {}) has lethal damage marked and is destroyed (put into owner's graveyard).",
                    card_id
                ));
                self.execute_instruction(SimInstruction::MoveCard {
                    card_id,
                    from: Zone::Battlefield,
                    to: Zone::Graveyard,
                    controller,
                });
                executed_any = true;
            }
        }

        sba_logs
    }

    /// Dynamically determines all priority actions a player can perform in this state (Rule 117).
    pub fn get_possible_actions(&self, player_id: PlayerId) -> Vec<crate::actions::PriorityAction> {
        let mut actions = Vec::new();

        // 1. Passing is always a valid choice if you have priority
        actions.push(crate::actions::PriorityAction::PassPriority { player: player_id });

        // 2. Mana abilities (Rule 605) are dynamically gathered from permanents
        for p in &self.zones.battlefield.permanents {
            if p.controller == player_id && !p.status.tapped {
                let mana_abilities = crate::abilities::compute_all_mana_abilities(self, player_id, p.id);
                for (idx, _color) in mana_abilities.iter().enumerate() {
                    actions.push(crate::actions::PriorityAction::ActivateAbility {
                        player: player_id,
                        ability_id: idx as u32,
                        source_id: p.id,
                        is_instant_speed: true,
                        is_mana_ability: true,
                    });
                }
            }
        }

        // 3. Spells and other playable abilities (Rule 113) evaluated by decoupled engine
        let playable = crate::abilities::compute_all_playable_abilities(self, player_id);
        actions.extend(playable);

        actions
    }

    /// Executes a PriorityAction algorithmically on the game state and returns logs of any state-based actions that resolve.
    pub fn execute_action(&mut self, action: crate::actions::PriorityAction) -> Vec<String> {
        let mut logs = Vec::new();

        match action {
            crate::actions::PriorityAction::ActivateAbility { player, source_id, ability_id, .. } => {
                // Tapping a permanent for mana generically (Rule 605)
                let card_name = self.get_registered_card_name(source_id);
                self.execute_instruction(SimInstruction::TapPermanent { card_id: source_id });
                
                // Retrieve the color of mana produced by this specific ability index from our decoupled rules engine
                let mana_abilities = crate::abilities::compute_all_mana_abilities(self, player, source_id);
                let color_opt = mana_abilities.get(ability_id as usize).copied();

                if let Some(color) = color_opt {
                    self.execute_instruction(SimInstruction::AddMana {
                        player_id: player,
                        color,
                        amount: 1,
                    });
                    let player_name = self.players.get(&player).map(|p| p.name.as_str()).unwrap_or("Unknown");
                    logs.push(format!("{} taps {} for {{{:?}}}.", player_name, card_name, color));
                }
                self.consecutive_passes = 0;
            }

            crate::actions::PriorityAction::CastSpell { player, card_id, target, .. } => {
                let card = self.card_registry[&card_id].card.clone();
                
                // Get and execute payment instructions
                if let Some(player_obj) = self.players.get(&player) {
                    if let Some(pay_insts) = crate::abilities::get_payment_instructions(&card, player, &player_obj.mana_pool) {
                        for inst in pay_insts {
                            self.execute_instruction(inst);
                        }
                    }
                }

                // Move card to stack and push
                self.execute_instruction(SimInstruction::MoveCard {
                    card_id,
                    from: Zone::Hand,
                    to: Zone::Stack,
                    controller: player,
                });
                self.execute_instruction(SimInstruction::PushSpell {
                    card_id,
                    caster: player,
                });

                // Register chosen target
                if let Some(ref t) = target {
                    if let Some(top_item) = self.stack.items.last() {
                        let top_item_id = top_item.id;
                        self.execute_instruction(SimInstruction::RegisterTarget {
                            stack_item_id: top_item_id,
                            target: t.clone(),
                        });
                    }
                    
                    let player_name = self.players.get(&player).map(|p| p.name.as_str()).unwrap_or("Unknown");
                    let target_desc = match t {
                        Target::Card(id) => {
                            let card_owner = self.card_registry.get(id).map(|zc| zc.owner).unwrap_or(0);
                            let owner_name = self.players.get(&card_owner).map(|p| p.name.as_str()).unwrap_or("Unknown");
                            format!("{}'s {}", owner_name, self.get_registered_card_name(*id))
                        }
                        Target::Spell(id) => {
                            let card_owner = self.card_registry.get(id).map(|zc| zc.owner).unwrap_or(0);
                            let owner_name = self.players.get(&card_owner).map(|p| p.name.as_str()).unwrap_or("Unknown");
                            format!("{}'s {}", owner_name, self.get_registered_card_name(*id))
                        }
                        _ => "target".to_string(),
                    };
                    logs.push(format!("{} casts {} targeting {}.", player_name, card.name(), target_desc));
                } else {
                    let player_name = self.players.get(&player).map(|p| p.name.as_str()).unwrap_or("Unknown");
                    logs.push(format!("{} casts {}.", player_name, card.name()));
                }
                self.consecutive_passes = 0;
            }

            crate::actions::PriorityAction::PassPriority { player } => {
                self.consecutive_passes += 1;
                let player_name = self.players.get(&player).map(|p| p.name.as_str()).unwrap_or("Unknown");
                logs.push(format!("{} passes priority.", player_name));

                let num_players = self.players.len() as u32;
                if self.consecutive_passes < num_players {
                    // Pass priority to the next player in turn order
                    let next_priority = if let Some(pos) = self.turn_order.iter().position(|&id| id == player) {
                        let next_pos = (pos + 1) % self.turn_order.len();
                        Some(self.turn_order[next_pos])
                    } else {
                        self.turn_order.first().cloned()
                    };
                    self.priority_player = next_priority;
                } else {
                    // consecutive_passes == num_players: Resolve the top of the stack!
                    if !self.stack.items.is_empty() {
                        let top_item = self.stack.items.last().cloned().unwrap();
                        match top_item.object {
                            StackObject::Spell { card_id, caster, .. } => {
                                let card = self.card_registry[&card_id].card.clone();
                                
                                // Fetch target from stack item
                                let target = self.stack.items.last()
                                    .and_then(|item| item.targets.first().cloned());

                                // Get resolution instructions and log
                                let (res_insts, res_log) = crate::abilities::get_resolution_instructions(&card, card_id, target, caster, self);
                                logs.push(res_log);

                                for inst in res_insts {
                                    self.execute_instruction(inst);
                                }
                            }
                            _ => {
                                self.execute_instruction(SimInstruction::PopStack);
                            }
                        }

                        // Reset passes after resolution
                        self.consecutive_passes = 0;
                        // Priority goes back to active player
                        self.priority_player = Some(self.active_player);

                        // --- CRITICAL STATE-BASED ACTIONS CHECK ---
                        let sba_logs = self.check_state_based_actions();
                        for sba in sba_logs {
                            logs.push(format!("[SBA TRIGGERED] {}", sba));
                        }
                    } else {
                        // Empty stack passed twice -> Step ends
                        self.priority_player = None;
                        logs.push("Both players passed on an empty stack. Step ends.".to_string());
                    }
                }
            }

            _ => {}
        }

        logs
    }
}