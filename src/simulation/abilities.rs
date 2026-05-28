use crate::game::{Game, SimInstruction};
use crate::effects::{PlayerId, CardId, Target, Zone};
use crate::actions::PriorityAction;
use crate::card::{Card, Color, ManaSymbols, Subtype, LandType};
use crate::player::ManaPool;

/// Rule 605: Mana Abilities
/// Returns any color of mana this card can produce.
/// An activated ability is a mana ability if:
/// 1. It doesn't require a target
/// 2. It could add mana to a player's mana pool when it resolves
/// 3. It's not a loyalty ability (planeswalker ability)
pub fn get_mana_abilities(card: &Card) -> Vec<Color> {
    let is_loyalty_ability = matches!(card, Card::Planeswalker(_));
    if is_loyalty_ability || requires_target(card) {
        return Vec::new();
    }

    let text = match card {
        Card::Artifact(attrs) => &attrs.card.rules_text,
        Card::Battle(attrs) => &attrs.card.rules_text,
        Card::Creature(attrs) => &attrs.card.rules_text,
        Card::Enchantment(attrs) => &attrs.card.rules_text,
        Card::Instant(attrs) => &attrs.card.rules_text,
        Card::Land(attrs) => &attrs.card.rules_text,
        Card::Planeswalker(attrs) => &attrs.card.rules_text,
        Card::Sorcery(attrs) => &attrs.card.rules_text,
        Card::Kindred(attrs) => &attrs.card.rules_text,
        Card::Leveler(attrs) => &attrs.card.rules_text,
        Card::Saga(attrs) => &attrs.card.rules_text,
        Card::Class(attrs) => &attrs.card.rules_text,
    };

    let mut colors = Vec::new();
    let text_lower = text.to_lowercase();

    // Parse rules text for standard activated mana abilities (e.g. "{T}: Add {G}")
    if text_lower.contains("{t}: add {g}") || text_lower.contains("add {g}") {
        colors.push(Color::G);
    }
    if text_lower.contains("{t}: add {r}") || text_lower.contains("add {r}") {
        colors.push(Color::R);
    }
    if text_lower.contains("{t}: add {u}") || text_lower.contains("add {u}") {
        colors.push(Color::U);
    }
    if text_lower.contains("{t}: add {b}") || text_lower.contains("add {b}") {
        colors.push(Color::B);
    }
    if text_lower.contains("{t}: add {w}") || text_lower.contains("add {w}") {
        colors.push(Color::W);
    }
    if text_lower.contains("{t}: add {c}") || text_lower.contains("add {c}") {
        colors.push(Color::C);
    }

    // Rule 305.6: Subtypes and intrinsic basic land mana abilities
    if colors.is_empty() {
        if let Card::Land(attrs) = card {
            for subtype in &attrs.card.subtypes {
                if let Subtype::Land(land_type) = subtype {
                    match land_type {
                        LandType::Forest => colors.push(Color::G),
                        LandType::Mountain => colors.push(Color::R),
                        LandType::Island => colors.push(Color::U),
                        LandType::Swamp => colors.push(Color::B),
                        LandType::Plains => colors.push(Color::W),
                        _ => {}
                    }
                }
            }
        }
    }

    colors
}

/// Checks if the rules text of a card indicates it requires a target
pub fn requires_target(card: &Card) -> bool {
    let text = match card {
        Card::Artifact(attrs) => &attrs.card.rules_text,
        Card::Battle(attrs) => &attrs.card.rules_text,
        Card::Creature(attrs) => &attrs.card.rules_text,
        Card::Enchantment(attrs) => &attrs.card.rules_text,
        Card::Instant(attrs) => &attrs.card.rules_text,
        Card::Land(attrs) => &attrs.card.rules_text,
        Card::Planeswalker(attrs) => &attrs.card.rules_text,
        Card::Sorcery(attrs) => &attrs.card.rules_text,
        Card::Kindred(attrs) => &attrs.card.rules_text,
        Card::Leveler(attrs) => &attrs.card.rules_text,
        Card::Saga(attrs) => &attrs.card.rules_text,
        Card::Class(attrs) => &attrs.card.rules_text,
    };
    text.to_lowercase().contains("target")
}

/// Computes valid targets for a card based on current game state/stack
pub fn get_valid_targets(card: &Card, game: &Game) -> Vec<Target> {
    let text = match card {
        Card::Artifact(attrs) => &attrs.card.rules_text,
        Card::Battle(attrs) => &attrs.card.rules_text,
        Card::Creature(attrs) => &attrs.card.rules_text,
        Card::Enchantment(attrs) => &attrs.card.rules_text,
        Card::Instant(attrs) => &attrs.card.rules_text,
        Card::Land(attrs) => &attrs.card.rules_text,
        Card::Planeswalker(attrs) => &attrs.card.rules_text,
        Card::Sorcery(attrs) => &attrs.card.rules_text,
        Card::Kindred(attrs) => &attrs.card.rules_text,
        Card::Leveler(attrs) => &attrs.card.rules_text,
        Card::Saga(attrs) => &attrs.card.rules_text,
        Card::Class(attrs) => &attrs.card.rules_text,
    }.to_lowercase();

    let mut targets = Vec::new();
    if text.contains("target spell") {
        for item in &game.stack.items {
            if let crate::stack::StackObject::Spell { card_id, .. } = item.object {
                targets.push(Target::Spell(card_id));
            }
        }
    } else if text.contains("target creature") {
        for perm in &game.zones.battlefield.permanents {
            if let Card::Creature(_) = perm.card {
                targets.push(Target::Card(perm.id));
            }
        }
    }
    targets
}

/// Evaluates if a player's mana pool is able to pay for the printed spell cost.
/// Currently does not take into account alternate or additional paying costs
/// (like sacrificing a creature), or modifiers due to effects.
pub fn can_pay_cost(card: &Card, player_mana: &ManaPool) -> bool {
    if let Some(spell_cost) = card.get_spell_cost() {
        let mut req = ManaPool::default();
        let mut generic = 0;
        for sym in spell_cost {
            match sym {
                ManaSymbols::W => req.white += 1,
                ManaSymbols::U => req.blue += 1,
                ManaSymbols::B => req.black += 1,
                ManaSymbols::R => req.red += 1,
                ManaSymbols::G => req.green += 1,
                ManaSymbols::C => req.colorless += 1,
                ManaSymbols::N(amt) => generic += *amt as u32,
                _ => {}
            }
        }
        player_mana.can_pay(req) && player_mana.total() >= req.total() + generic
    } else {
        true
    }
}

/// Translates the cost payment into atomic SimInstructions
pub fn get_payment_instructions(card: &Card, player_id: PlayerId, mana_pool: &ManaPool) -> Option<Vec<SimInstruction>> {
    if let Some(spell_cost) = card.get_spell_cost() {
        let mut insts = Vec::new();
        let mut req = ManaPool::default();
        let mut generic = 0;
        for sym in spell_cost {
            match sym {
                ManaSymbols::W => req.white += 1,
                ManaSymbols::U => req.blue += 1,
                ManaSymbols::B => req.black += 1,
                ManaSymbols::R => req.red += 1,
                ManaSymbols::G => req.green += 1,
                ManaSymbols::C => req.colorless += 1,
                ManaSymbols::N(amt) => generic += *amt as u32,
                _ => {}
            }
        }
        for _ in 0..req.white { insts.push(SimInstruction::SpendMana { player_id, color: Color::W, amount: 1 }); }
        for _ in 0..req.blue { insts.push(SimInstruction::SpendMana { player_id, color: Color::U, amount: 1 }); }
        for _ in 0..req.black { insts.push(SimInstruction::SpendMana { player_id, color: Color::B, amount: 1 }); }
        for _ in 0..req.red { insts.push(SimInstruction::SpendMana { player_id, color: Color::R, amount: 1 }); }
        for _ in 0..req.green { insts.push(SimInstruction::SpendMana { player_id, color: Color::G, amount: 1 }); }
        for _ in 0..req.colorless { insts.push(SimInstruction::SpendMana { player_id, color: Color::C, amount: 1 }); }
        
        let mut temp_pool = mana_pool.clone();
        let _ = temp_pool.spend(req);
        for _ in 0..generic {
            let spent_color = if temp_pool.colorless >= 1 {
                temp_pool.colorless -= 1;
                Color::C
            } else if temp_pool.green >= 1 {
                temp_pool.green -= 1;
                Color::G
            } else if temp_pool.red >= 1 {
                temp_pool.red -= 1;
                Color::R
            } else if temp_pool.black >= 1 {
                temp_pool.black -= 1;
                Color::B
            } else if temp_pool.blue >= 1 {
                temp_pool.blue -= 1;
                Color::U
            } else if temp_pool.white >= 1 {
                temp_pool.white -= 1;
                Color::W
            } else {
                Color::C
            };
            insts.push(SimInstruction::SpendMana { player_id, color: spent_color, amount: 1 });
        }
        Some(insts)
    } else {
        Some(vec![])
    }
}

/// Translates spell/ability resolution into primitive VM instructions
pub fn get_resolution_instructions(
    card: &Card,
    self_id: CardId,
    target: Option<Target>,
    caster: PlayerId,
    game: &Game,
) -> (Vec<SimInstruction>, String) {
    let n = card.name();
    let mut insts = vec![SimInstruction::PopStack];
    
    let text = match card {
        Card::Artifact(attrs) => &attrs.card.rules_text,
        Card::Battle(attrs) => &attrs.card.rules_text,
        Card::Creature(attrs) => &attrs.card.rules_text,
        Card::Enchantment(attrs) => &attrs.card.rules_text,
        Card::Instant(attrs) => &attrs.card.rules_text,
        Card::Land(attrs) => &attrs.card.rules_text,
        Card::Planeswalker(attrs) => &attrs.card.rules_text,
        Card::Sorcery(attrs) => &attrs.card.rules_text,
        Card::Kindred(attrs) => &attrs.card.rules_text,
        Card::Leveler(attrs) => &attrs.card.rules_text,
        Card::Saga(attrs) => &attrs.card.rules_text,
        Card::Class(attrs) => &attrs.card.rules_text,
    }.to_lowercase();

    let is_permanent = match card {
        Card::Artifact(_) | Card::Battle(_) | Card::Creature(_) | Card::Enchantment(_) | Card::Planeswalker(_) | Card::Land(_) | Card::Leveler(_) | Card::Saga(_) | Card::Class(_) => true,
        _ => false,
    };

    let mut log = format!("{} resolves.", n);

    if !is_permanent {
        if text.contains("counter target") {
            if let Some(Target::Spell(target_card_id)) = target {
                if let Some(pos) = game.stack.items.iter().position(|item| {
                    if let crate::stack::StackObject::Spell { card_id: c_id, .. } = item.object {
                        c_id == target_card_id
                    } else {
                        false
                    }
                }) {
                    let target_item_id = game.stack.items[pos].id;
                    let target_owner = game.card_registry[&target_card_id].owner;
                    insts.push(SimInstruction::RemoveFromStack { stack_item_id: target_item_id });
                    insts.push(SimInstruction::MoveCard {
                        card_id: target_card_id,
                        from: Zone::Stack,
                        to: Zone::Graveyard,
                        controller: target_owner,
                    });
                    let target_name = game.get_registered_card_name(target_card_id);
                    log = format!("{} resolves. {} is countered and put into owner's graveyard.", n, target_name);
                } else {
                    log = format!("{} resolves, but its target was not found on the stack.", n);
                }
            } else {
                log = format!("{} resolves, but has no valid target.", n);
            }
        } else if text.contains("deals 2 damage") {
            if let Some(Target::Card(target_id)) = target {
                insts.push(SimInstruction::MarkDamage { card_id: target_id, amount: 2 });
                let target_name = game.get_registered_card_name(target_id);
                log = format!("{} resolves, dealing 2 damage to {}.", n, target_name);
            } else {
                log = format!("{} resolves, but has no valid target.", n);
            }
        }

        insts.push(SimInstruction::MoveCard {
            card_id: self_id,
            from: Zone::Stack,
            to: Zone::Graveyard,
            controller: caster,
        });
    } else {
        insts.push(SimInstruction::MoveCard {
            card_id: self_id,
            from: Zone::Stack,
            to: Zone::Battlefield,
            controller: caster,
        });
    }

    (insts, log)
}

/// Rule 605: Mana Abilities
/// Computes all mana abilities that a specific player has access to on a permanent.
pub fn compute_all_mana_abilities(game: &Game, player_id: PlayerId, permanent_id: CardId) -> Vec<Color> {
    if let Some(zc) = game.card_registry.get(&permanent_id) {
        // Ensure the player controls the permanent on the battlefield
        let is_controlled = game.zones.battlefield.permanents.iter()
            .any(|p| p.id == permanent_id && p.controller == player_id);
        if is_controlled {
            // Apply continuous/static effects in layer order to the permanent's card before querying its abilities!
            let card_effective = game.apply_active_effects(permanent_id, zc.card.clone());
            return get_mana_abilities(&card_effective);
        }
    }
    Vec::new()
}

/// Rule 113: Playable/Castable Abilities
pub fn compute_all_playable_abilities(game: &Game, player_id: PlayerId) -> Vec<PriorityAction> {
    let mut actions = Vec::new();

    if let Some(hand) = game.zones.hands.get(&player_id) {
        for hand_card in &hand.cards {
            let card = &hand_card.card;
            
            // Apply active effects to compile characteristics of the card in hand
            let card_effective = game.apply_active_effects(hand_card.id, card.clone());
            
            if let Some(player) = game.players.get(&player_id) {
                if can_pay_cost(&card_effective, &player.mana_pool) {
                    // Timing permissions (can be modified by continuous/replacement effects in the future)
                    let is_instant = match card_effective {
                        Card::Instant(_) => true,
                        _ => false,
                    };

                    if requires_target(&card_effective) {
                        let valid_targets = get_valid_targets(&card_effective, game);
                        for target in valid_targets {
                            actions.push(PriorityAction::CastSpell {
                                player: player_id,
                                card_id: hand_card.id,
                                is_instant_speed: is_instant,
                                target: Some(target),
                            });
                        }
                    } else {
                        actions.push(PriorityAction::CastSpell {
                            player: player_id,
                            card_id: hand_card.id,
                            is_instant_speed: is_instant,
                            target: None,
                        });
                    }
                }
            }
        }
    }

    actions
}
