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
    pub current_phase: crate::turns::Phase,
}

/// Represents a deterministic, kernel-level state-transition instruction ("machine code").
/// These instructions have a 1:1 correspondence with the lowest-level state modifications
/// of our individual game structures.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    /// 1:1 with player life gain (Rule 119.3)
    GainLife {
        player_id: PlayerId,
        amount: u32,
    },
    /// 1:1 with player life loss (Rule 119.3)
    LoseLife {
        player_id: PlayerId,
        amount: u32,
    },
    /// Assert that a card has a specific type (Rule 300)
    CheckType {
        card_id: CardId,
        expected_type: crate::card::CardType,
    },
    /// Assert that a card does NOT have a specific type (Rule 300)
    CheckNotType {
        card_id: CardId,
        not_type: crate::card::CardType,
    },
    /// Assert that the stack is empty (Rule 117.4)
    CheckStackEmpty,
    /// Assert that the current phase is a specific phase (Rule 500)
    CheckPhase {
        expected_phase: crate::turns::Phase,
    },
    /// Assert that the active player is an opponent of the effect source (for Grand Abolisher style checks)
    CheckIsOpponent {
        player_id: PlayerId,
        source_card_id: CardId,
    },
    /// Assert that it is the turn of the player who controls the source card
    CheckIsSourceControllerTurn {
        source_card_id: CardId,
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
            current_phase: crate::turns::Phase::PrecombatMain,
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
    pub fn execute_instruction_raw(&mut self, instruction: SimInstruction) -> Result<(), String> {
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

            SimInstruction::GainLife { player_id, amount } => {
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.life_total += amount as u128;
                    println!(
                        "  -> \x1b[32mLife Gained:\x1b[0m +{} to Player {} (New total: {})",
                        amount, player.name, player.life_total
                    );
                }
            }

            SimInstruction::LoseLife { player_id, amount } => {
                if let Some(player) = self.players.get_mut(&player_id) {
                    player.life_total = player.life_total.saturating_sub(amount as u128);
                    println!(
                        "  -> \x1b[31mLife Lost:\x1b[0m -{} from Player {} (New total: {})",
                        amount, player.name, player.life_total
                    );
                }
            }

            SimInstruction::CheckType { card_id, expected_type } => {
                if let Some(err) = self.evaluate_check(&SimInstruction::CheckType { card_id, expected_type }) {
                    println!("  -> \x1b[31mCheck Failed:\x1b[0m {}", err);
                    return Err(err);
                }
            }
            SimInstruction::CheckNotType { card_id, not_type } => {
                if let Some(err) = self.evaluate_check(&SimInstruction::CheckNotType { card_id, not_type }) {
                    println!("  -> \x1b[31mCheck Failed:\x1b[0m {}", err);
                    return Err(err);
                }
            }
            SimInstruction::CheckStackEmpty => {
                if let Some(err) = self.evaluate_check(&SimInstruction::CheckStackEmpty) {
                    println!("  -> \x1b[31mCheck Failed:\x1b[0m {}", err);
                    return Err(err);
                }
            }
            SimInstruction::CheckPhase { expected_phase } => {
                if let Some(err) = self.evaluate_check(&SimInstruction::CheckPhase { expected_phase }) {
                    println!("  -> \x1b[31mCheck Failed:\x1b[0m {}", err);
                    return Err(err);
                }
            }
            SimInstruction::CheckIsOpponent { player_id, source_card_id } => {
                if let Some(err) = self.evaluate_check(&SimInstruction::CheckIsOpponent { player_id, source_card_id }) {
                    println!("  -> \x1b[31mCheck Failed:\x1b[0m {}", err);
                    return Err(err);
                }
            }
            SimInstruction::CheckIsSourceControllerTurn { source_card_id } => {
                if let Some(err) = self.evaluate_check(&SimInstruction::CheckIsSourceControllerTurn { source_card_id }) {
                    println!("  -> \x1b[31mCheck Failed:\x1b[0m {}", err);
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    /// Intercepts a SimInstruction, resolves replacement/prevention effects, and executes the raw mutations.
    pub fn execute_instruction(&mut self, instruction: SimInstruction) -> Result<(), String> {
        let resolved_insts = self.apply_replacement_and_prevention_effects(instruction);
        for inst in resolved_insts {
            self.execute_instruction_raw(inst)?;
        }
        Ok(())
    }

    /// Executes multiple deterministic CPU instructions sequentially, aborting if any instruction fails (e.g. check failures).
    pub fn execute_instructions(&mut self, instructions: Vec<SimInstruction>) -> Result<(), String> {
        for inst in instructions {
            self.execute_instruction(inst)?;
        }
        Ok(())
    }

    /// Intercepts a SimInstruction and applies active replacement (Rule 614) and prevention (Rule 615) effects.
    /// Returns the final vector of instructions to actually execute.
    pub fn apply_replacement_and_prevention_effects(&mut self, inst: SimInstruction) -> Vec<SimInstruction> {
        let mut instructions = vec![inst];
        let mut changed = true;
        let mut loop_count = 0;
        let mut applied_ids = std::collections::HashSet::new();

        // Loop to allow multiple replacements to chain (up to a limit to prevent infinite loops)
        while changed && loop_count < 10 {
            changed = false;
            let mut next_instructions = Vec::new();

            for current_inst in instructions {
                let mut replaced = false;

                // Check replacement effects
                for rep in &self.active_effects.replacement_effects {
                    if applied_ids.contains(&rep.id) {
                        continue;
                    }

                    // Match replacement effect description
                    if rep.description == "enters the battlefield tapped" {
                        if let SimInstruction::MoveCard { card_id, from, to, controller } = current_inst.clone() {
                            if to == Zone::Battlefield {
                                next_instructions.push(SimInstruction::MoveCard { card_id, from, to, controller });
                                next_instructions.push(SimInstruction::TapPermanent { card_id });
                                replaced = true;
                                applied_ids.insert(rep.id);
                                break;
                            }
                        }
                    } else if rep.description == "if you would draw a card, draw two cards instead" {
                        if let SimInstruction::DrawCard { player_id } = current_inst {
                            next_instructions.push(SimInstruction::DrawCard { player_id });
                            next_instructions.push(SimInstruction::DrawCard { player_id });
                            replaced = true;
                            applied_ids.insert(rep.id);
                            break;
                        }
                    } else if rep.description == "if you would gain life, gain twice that much life instead" {
                        if let SimInstruction::GainLife { player_id, amount } = current_inst {
                            next_instructions.push(SimInstruction::GainLife { player_id, amount: amount * 2 });
                            replaced = true;
                            applied_ids.insert(rep.id);
                            break;
                        }
                    } else if rep.description == "if a card would be put into a graveyard, exile it instead" {
                        if let SimInstruction::MoveCard { card_id, from, to, controller } = current_inst.clone() {
                            if to == Zone::Graveyard {
                                next_instructions.push(SimInstruction::MoveCard { card_id, from, to: Zone::Exile, controller });
                                replaced = true;
                                applied_ids.insert(rep.id);
                                break;
                            }
                        }
                    }
                }

                if replaced {
                    changed = true;
                    continue;
                }

                // Check prevention effects (Rule 615)
                if let SimInstruction::MarkDamage { card_id, amount } = current_inst {
                    let mut damage_remaining = amount;
                    // Find active prevention effects
                    for prev in &mut self.active_effects.prevention_effects {
                        if prev.amount_remaining > 0 && damage_remaining > 0 {
                            // Check scope
                            let applies = match prev.scope {
                                crate::effects::PreventionScope::AnyDamage => true,
                                crate::effects::PreventionScope::DamageToTarget(Target::Card(tid)) => tid == card_id,
                                _ => false,
                            };

                            if applies {
                                let prevent = damage_remaining.min(prev.amount_remaining);
                                damage_remaining -= prevent;
                                prev.amount_remaining -= prevent;
                                println!(
                                    "  -> \x1b[32mPrevention Shield Applied:\x1b[0m Prevented {} damage to Card ID {} using effect from Card ID {}. (Remaining shield: {})",
                                    prevent, card_id, prev.source, prev.amount_remaining
                                );
                            }
                        }
                    }

                    // Retain prevention effects with remaining capacity
                    self.active_effects.prevention_effects.retain(|p| p.amount_remaining > 0);

                    if damage_remaining < amount {
                        if damage_remaining > 0 {
                            next_instructions.push(SimInstruction::MarkDamage { card_id, amount: damage_remaining });
                        }
                        replaced = true;
                    }
                }

                if !replaced {
                    next_instructions.push(current_inst);
                }
            }

            instructions = next_instructions;
            loop_count += 1;
        }

        instructions
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
            // Find the player associated with this card (controller if on battlefield, else owner)
            let player_id_opt = if let Some(p) = self.zones.battlefield.permanents.iter().find(|p| p.id == card_id) {
                Some(p.controller)
            } else if let Some(zc) = self.card_registry.get(&card_id) {
                Some(zc.owner)
            } else {
                None
            };

            let applies = if !effect.conditions.is_empty() {
                effect.conditions.iter().all(|cond| {
                    self.evaluate_condition(cond, Some(&card), Some(card_id), player_id_opt, effect.source)
                })
            } else {
                // Fall back to simple default logic if no conditions are defined
                effect.source == card_id || effect.id % 2 == card_id % 2
            };

            if applies {
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
                    crate::effects::ContinuousEffectType::TextChange { ref from, ref to } => {
                        let attrs = get_card_attributes_mut(&mut card);
                        attrs.rules_text = attrs.rules_text.replace(from, to);
                    }
                    _ => {}
                }
            }
        }
        card
    }

    /// Evaluates a compiled continuous effect condition dynamically against the current game state and query context.
    pub fn evaluate_condition(
        &self,
        cond: &crate::effects::EffectCondition,
        card_opt: Option<&Card>,
        _card_id_opt: Option<CardId>,
        player_id_opt: Option<PlayerId>,
        source_card_id: CardId,
    ) -> bool {
        match cond {
            crate::effects::EffectCondition::HasType(expected_type) => {
                if let Some(card) = card_opt {
                    card.get_attributes().types.contains(expected_type)
                } else {
                    false
                }
            }
            crate::effects::EffectCondition::HasSubtype(expected_subtype) => {
                if let Some(card) = card_opt {
                    card.get_attributes().subtypes.contains(expected_subtype)
                } else {
                    false
                }
            }
            crate::effects::EffectCondition::HasColor(expected_color) => {
                if let Some(card) = card_opt {
                    if let Some(spell_attrs) = card.get_spell_attributes() {
                        spell_attrs.color.contains(expected_color)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            crate::effects::EffectCondition::IsOpponentOfSource => {
                if let Some(player_id) = player_id_opt {
                    let source_controller = if let Some(perm) = self.zones.battlefield.permanents.iter().find(|p| p.id == source_card_id) {
                        Some(perm.controller)
                    } else if let Some(zc) = self.card_registry.get(&source_card_id) {
                        Some(zc.owner)
                    } else {
                        None
                    };
                    if let Some(controller) = source_controller {
                        controller != player_id
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            crate::effects::EffectCondition::IsSourceController => {
                if let Some(player_id) = player_id_opt {
                    let source_controller = if let Some(perm) = self.zones.battlefield.permanents.iter().find(|p| p.id == source_card_id) {
                        Some(perm.controller)
                    } else if let Some(zc) = self.card_registry.get(&source_card_id) {
                        Some(zc.owner)
                    } else {
                        None
                    };
                    if let Some(controller) = source_controller {
                        controller == player_id
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            crate::effects::EffectCondition::IsSourceControllerTurn => {
                let source_controller = if let Some(perm) = self.zones.battlefield.permanents.iter().find(|p| p.id == source_card_id) {
                    Some(perm.controller)
                } else if let Some(zc) = self.card_registry.get(&source_card_id) {
                    Some(zc.owner)
                } else {
                    None
                };
                if let Some(controller) = source_controller {
                    self.active_player == controller
                } else {
                    false
                }
            }
            crate::effects::EffectCondition::IsPhase(expected_phase) => {
                self.current_phase == *expected_phase
            }
            crate::effects::EffectCondition::IsStackEmpty => {
                self.stack.items.is_empty()
            }
            crate::effects::EffectCondition::Not(nested) => {
                !self.evaluate_condition(nested, card_opt, _card_id_opt, player_id_opt, source_card_id)
            }
            crate::effects::EffectCondition::And(nested_list) => {
                nested_list.iter().all(|c| self.evaluate_condition(c, card_opt, _card_id_opt, player_id_opt, source_card_id))
            }
            crate::effects::EffectCondition::Or(nested_list) => {
                nested_list.iter().any(|c| self.evaluate_condition(c, card_opt, _card_id_opt, player_id_opt, source_card_id))
            }
        }
    }

    /// Evaluates a check instruction. Returns None if the check passes, or Some(error_message) if it fails.
    pub fn evaluate_check(&self, check: &SimInstruction) -> Option<String> {
        match check {
            SimInstruction::CheckType { card_id, expected_type } => {
                let card = self.card_registry.get(card_id).map(|zc| zc.card.clone())?;
                let card_effective = self.apply_active_effects(*card_id, card);
                let attrs = card_effective.get_attributes();
                if !attrs.types.contains(expected_type) {
                    Some(format!("CheckType Failed: Card ID {} is not of type {:?}", card_id, expected_type))
                } else {
                    None
                }
            }
            SimInstruction::CheckNotType { card_id, not_type } => {
                let card = self.card_registry.get(card_id).map(|zc| zc.card.clone())?;
                let card_effective = self.apply_active_effects(*card_id, card);
                let attrs = card_effective.get_attributes();
                if attrs.types.contains(not_type) {
                    Some(format!("CheckNotType Failed: Card ID {} is of type {:?}", card_id, not_type))
                } else {
                    None
                }
            }
            SimInstruction::CheckStackEmpty => {
                if !self.stack.items.is_empty() {
                    Some("CheckStackEmpty Failed: Stack is not empty".to_string())
                } else {
                    None
                }
            }
            SimInstruction::CheckPhase { expected_phase } => {
                if self.current_phase != *expected_phase {
                    Some(format!("CheckPhase Failed: Current phase is {:?}, expected {:?}", self.current_phase, expected_phase))
                } else {
                    None
                }
            }
            SimInstruction::CheckIsOpponent { player_id, source_card_id } => {
                let source_controller = if let Some(perm) = self.zones.battlefield.permanents.iter().find(|p| p.id == *source_card_id) {
                    Some(perm.controller)
                } else if let Some(zc) = self.card_registry.get(source_card_id) {
                    Some(zc.owner)
                } else {
                    None
                };
                if let Some(controller) = source_controller {
                    if controller == *player_id {
                        Some(format!("CheckIsOpponent Failed: Player {} is controller, not opponent", player_id))
                    } else {
                        None
                    }
                } else {
                    Some("CheckIsOpponent Failed: Source controller not found".to_string())
                }
            }
            SimInstruction::CheckIsSourceControllerTurn { source_card_id } => {
                let source_controller = if let Some(perm) = self.zones.battlefield.permanents.iter().find(|p| p.id == *source_card_id) {
                    Some(perm.controller)
                } else if let Some(zc) = self.card_registry.get(source_card_id) {
                    Some(zc.owner)
                } else {
                    None
                };
                if let Some(controller) = source_controller {
                    if self.active_player != controller {
                        Some(format!("CheckIsSourceControllerTurn Failed: Active player is {}, but source controller is {}", self.active_player, controller))
                    } else {
                        None
                    }
                } else {
                    Some("CheckIsSourceControllerTurn Failed: Source controller not found".to_string())
                }
            }
            _ => None,
        }
    }

    /// Checks if a player's activated abilities are suppressed by active ActionRestriction effects.
    pub fn is_activated_ability_suppressed(&self, player_id: PlayerId) -> bool {
        for effect in &self.active_effects.continuous_effects {
            if let crate::effects::ContinuousEffectType::ActionRestriction { restrict_instructions } = &effect.effect {
                let mut all_match = true;
                for cond in &effect.conditions {
                    if !self.evaluate_condition(cond, None, None, Some(player_id), effect.source) {
                        all_match = false;
                        break;
                    }
                }
                if all_match {
                    // Conditions match, now run the compiled restrict_instructions!
                    let mut restricted = true;
                    for inst in restrict_instructions {
                        // Dynamically bind player_id to the check being evaluated
                        let active_inst = match inst {
                            SimInstruction::CheckIsOpponent { source_card_id, .. } => {
                                SimInstruction::CheckIsOpponent { player_id, source_card_id: *source_card_id }
                            }
                            other => other.clone(),
                        };
                        if self.evaluate_check(&active_inst).is_some() {
                            // A check failed, so this restriction does not apply
                            restricted = false;
                            break;
                        }
                    }
                    if restricted {
                        return true; // Action is suppressed/restricted!
                    }
                }
            }
        }
        false
    }

    /// Run the Magic State-Based Action trigger-resolution loop (Rule 704).
    /// Loops continuously until a stable state is reached where no more SBAs trigger.
    #[allow(unused_must_use)]
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
        if !self.is_activated_ability_suppressed(player_id) {
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
        }

        // 3. Spells and other playable abilities (Rule 113) evaluated by decoupled engine
        let playable = crate::abilities::compute_all_playable_abilities(self, player_id);
        actions.extend(playable);

        actions
    }

    /// Executes a PriorityAction algorithmically on the game state and returns logs of any state-based actions that resolve.
    #[allow(unused_must_use)]
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
                let card_effective = self.apply_active_effects(card_id, card.clone());
                
                // Get and execute payment instructions
                if let Some(player_obj) = self.players.get(&player) {
                    if let Some(pay_insts) = crate::abilities::get_payment_instructions(&card_effective, card_id, player, &player_obj.mana_pool, self) {
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

                                if let Err(e) = self.execute_instructions(res_insts) {
                                    logs.push(format!("[SPELL FIZZLED] Resolution aborted: {}", e));
                                    
                                    // If a resolution is aborted, the card must go to the graveyard.
                                    // Check if the card is already in the graveyard or battlefield
                                    let is_in_graveyard = self.zones.graveyards.values().any(|g| g.cards.iter().any(|zc| zc.id == card_id));
                                    let is_on_battlefield = self.zones.battlefield.permanents.iter().any(|p| p.id == card_id);
                                    if !is_in_graveyard && !is_on_battlefield {
                                        let owner = self.card_registry[&card_id].owner;
                                        let _ = self.execute_instruction(SimInstruction::MoveCard {
                                            card_id,
                                            from: Zone::Stack,
                                            to: Zone::Graveyard,
                                            controller: owner,
                                        });
                                    }
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