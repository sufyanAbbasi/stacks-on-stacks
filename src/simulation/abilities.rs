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
    } else if text.contains("target creature") || text.contains("target") {
        for perm in &game.zones.battlefield.permanents {
            if let Card::Creature(_) = perm.card {
                targets.push(Target::Card(perm.id));
            }
        }
    }
    targets
}

/// Returns the effective mana cost of a card/spell, accounting for continuous tax effects (Rule 613.10).
pub fn get_effective_cost(card: &Card, card_id: CardId, player_id: PlayerId, game: &Game) -> Vec<ManaSymbols> {
    let mut cost = if let Some(spell_cost) = card.get_spell_cost() {
        spell_cost.to_vec()
    } else {
        return Vec::new();
    };

    let mut tax = 0;
    for effect in &game.active_effects.continuous_effects {
        if let crate::effects::ContinuousEffectType::TaxSpells { cost_increase } = effect.effect {
            let applies = if !effect.conditions.is_empty() {
                effect.conditions.iter().all(|cond| {
                    game.evaluate_condition(cond, Some(card), Some(card_id), Some(player_id), effect.source)
                })
            } else {
                true
            };
            if applies {
                tax += cost_increase;
            }
        }
    }

    if tax > 0 {
        let mut generic_found = false;
        for sym in &mut cost {
            if let ManaSymbols::N(ref mut amt) = sym {
                *amt += tax as u8;
                generic_found = true;
                break;
            }
        }
        if !generic_found {
            cost.push(ManaSymbols::N(tax as u8));
        }
    }

    cost
}

/// Evaluates if a player's mana pool is able to pay for the spell cost, inclusive of taxes.
pub fn can_pay_cost(card: &Card, card_id: CardId, player_id: PlayerId, player_mana: &ManaPool, game: &Game) -> bool {
    if card.get_spell_cost().is_none() {
        return true;
    }
    let spell_cost = get_effective_cost(card, card_id, player_id, game);
    let mut req = ManaPool::default();
    let mut generic = 0;
    for sym in &spell_cost {
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
}

/// Translates the cost payment into atomic SimInstructions, inclusive of taxes
pub fn get_payment_instructions(card: &Card, card_id: CardId, player_id: PlayerId, mana_pool: &ManaPool, game: &Game) -> Option<Vec<SimInstruction>> {
    if card.get_spell_cost().is_none() {
        return Some(vec![]);
    }
    let spell_cost = get_effective_cost(card, card_id, player_id, game);
    let mut insts = Vec::new();
    let mut req = ManaPool::default();
    let mut generic = 0;
    for sym in &spell_cost {
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
}

/// Translates spell/ability resolution into primitive VM instructions
pub fn get_resolution_instructions(
    card: &Card,
    self_id: CardId,
    target: Option<Target>,
    caster: PlayerId,
    game: &Game,
) -> (Vec<SimInstruction>, String) {
    let card_effective = game.apply_active_effects(self_id, card.clone());
    let attrs = card_effective.get_attributes();
    let n = &attrs.name;
    let rules_text = &attrs.rules_text;

    let mut insts = vec![SimInstruction::PopStack];
    
    let is_permanent = match &card_effective {
        Card::Artifact(_) | Card::Battle(_) | Card::Creature(_) | Card::Enchantment(_) | Card::Planeswalker(_) | Card::Land(_) | Card::Leveler(_) | Card::Saga(_) | Card::Class(_) => true,
        _ => false,
    };

    let mut log = format!("{} resolves.", n);

    if !is_permanent {
        let layout = get_parser_layout(&card_effective);
        if let Ok(abilities) = parse_rules_text_dynamic(n, rules_text, layout) {
            let (compiled, parsed_log) = compile_ast_to_instructions(&abilities, self_id, target, caster, game);
            if !compiled.is_empty() {
                insts.extend(compiled);
                log = parsed_log;
                insts.push(SimInstruction::MoveCard {
                    card_id: self_id,
                    from: Zone::Stack,
                    to: Zone::Graveyard,
                    controller: caster,
                });
                return (insts, log);
            }
        }

        // Fallback to substring matching if dynamic parser fails or compiles to nothing
        let text_lower = rules_text.to_lowercase();
        if text_lower.contains("counter target") {
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
        } else if text_lower.contains("deals 2 damage") {
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
                if can_pay_cost(&card_effective, hand_card.id, player_id, &player.mana_pool, game) {
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

/// Helper to determine the layout string expected by the magic-card-parser.
pub fn get_parser_layout(card: &Card) -> &'static str {
    match card {
        Card::Saga(_) => "saga",
        Card::Class(_) => "class",
        Card::Leveler(_) => "leveler",
        _ => {
            let attrs = card.get_attributes();
            if attrs.rules_text.to_lowercase().contains("case of the") {
                "case"
            } else {
                "normal"
            }
        }
    }
}

/// Dynamically invokes the NodeJS natural language parser to parse a card's (potentially modified) rules text on-the-fly.
pub fn parse_rules_text_dynamic(name: &str, oracle_text: &str, layout: &str) -> Result<Vec<crate::rules_text_ast::AbilityOrRemind>, Box<dyn std::error::Error>> {
    use std::process::{Command, Stdio};
    use std::io::Write;

    #[derive(serde::Serialize)]
    struct Payload<'a> {
        name: &'a str,
        oracle_text: &'a str,
        layout: &'a str,
    }
    let payload = Payload { name, oracle_text, layout };
    let json_input = serde_json::to_string(&payload)?;

    let mut child = Command::new("node")
        .arg("parse_card.js")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(json_input.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        let err_str = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Node parser failed with status {}: {}", output.status, err_str).into());
    }

    #[derive(serde::Deserialize)]
    struct ParseOutput {
        result: Option<Vec<Vec<serde_json::Value>>>,
        error: Option<serde_json::Value>,
    }

    let parsed: ParseOutput = serde_json::from_slice(&output.stdout)?;
    if let Some(err) = parsed.error {
        return Err(format!("Nearley parser error: {:?}", err).into());
    }

    if let Some(trees) = parsed.result {
        if !trees.is_empty() {
            let mut abilities = Vec::new();
            for val in &trees[0] {
                let ability: crate::rules_text_ast::AbilityOrRemind = serde_json::from_value(val.clone())?;
                abilities.push(ability);
            }
            return Ok(abilities);
        }
    }

    Err("No parse trees returned from parser".into())
}

pub fn compile_ast_to_instructions(
    abilities: &[crate::rules_text_ast::AbilityOrRemind],
    self_id: CardId,
    target: Option<Target>,
    caster: PlayerId,
    game: &Game,
) -> (Vec<SimInstruction>, String) {
    let mut insts = Vec::new();
    let mut log = String::new();

    for ability_or_remind in abilities {
        if let crate::rules_text_ast::AbilityOrRemind::Ability(ability) = ability_or_remind {
            match ability {
                crate::rules_text_ast::Ability::StaticOrSpell(sentence) => {
                    compile_sentence(sentence, self_id, target, caster, game, &mut insts, &mut log);
                }
                crate::rules_text_ast::Ability::StaticOrSpellList(sentences) => {
                    for sentence in sentences {
                        compile_sentence(sentence, self_id, target, caster, game, &mut insts, &mut log);
                    }
                }
                _ => {}
            }
        }
    }

    (insts, log)
}

fn compile_sentence(
    sentence: &crate::rules_text_ast::Sentence,
    self_id: CardId,
    target: Option<Target>,
    caster: PlayerId,
    game: &Game,
    insts: &mut Vec<SimInstruction>,
    log: &mut String,
) {
    match sentence {
        crate::rules_text_ast::Sentence::Simple(imperative) => {
            compile_imperative(imperative, self_id, target, caster, game, insts, log);
        }
        crate::rules_text_ast::Sentence::ObjectVerbPhrase { what, does } => {
            compile_object_verb_phrase(what, does, self_id, target, caster, game, insts, log);
        }
        crate::rules_text_ast::Sentence::PlayerVerbPhrase { actor, does } => {
            compile_player_verb_phrase(actor, does, self_id, target, caster, game, insts, log);
        }
        _ => {}
    }
}

fn compile_imperative(
    imperative: &crate::rules_text_ast::Imperative,
    self_id: CardId,
    target: Option<Target>,
    caster: PlayerId,
    game: &Game,
    insts: &mut Vec<SimInstruction>,
    log: &mut String,
) {
    let card_name = game.get_registered_card_name(self_id);
    match imperative {
        crate::rules_text_ast::Imperative::Counter { counter: _ } => {
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
                    *log = format!("{} resolves. {} is countered and put into owner's graveyard.", card_name, target_name);
                } else {
                    *log = format!("{} resolves, but its target was not found on the stack.", card_name);
                }
            } else {
                *log = format!("{} resolves, but has no valid target.", card_name);
            }
        }
        crate::rules_text_ast::Imperative::Draw { draw } => {
            let count = match draw {
                crate::rules_text_ast::DrawCount::Fixed(c) => *c,
                _ => 1,
            };
            for _ in 0..count {
                insts.push(SimInstruction::DrawCard { player_id: caster });
            }
            let caster_name = game.players.get(&caster).map(|p| p.name.as_str()).unwrap_or("Player");
            *log = format!("{} resolves. {} draws {} card(s).", card_name, caster_name, count);
        }
        crate::rules_text_ast::Imperative::GainLife { gain_life } => {
            let amount = match gain_life {
                crate::rules_text_ast::NumberDefinition::Fixed(a) => *a,
                _ => 1,
            };
            insts.push(SimInstruction::GainLife { player_id: caster, amount });
            let caster_name = game.players.get(&caster).map(|p| p.name.as_str()).unwrap_or("Player");
            *log = format!("{} resolves. {} gains {} life.", card_name, caster_name, amount);
        }
        crate::rules_text_ast::Imperative::LoseLife { lose_life } => {
            let amount = match lose_life {
                crate::rules_text_ast::NumberDefinition::Fixed(a) => *a,
                _ => 1,
            };
            insts.push(SimInstruction::LoseLife { player_id: caster, amount });
            let caster_name = game.players.get(&caster).map(|p| p.name.as_str()).unwrap_or("Player");
            *log = format!("{} resolves. {} loses {} life.", card_name, caster_name, amount);
        }
        crate::rules_text_ast::Imperative::Destroy { destroy: _ } => {
            if let Some(Target::Card(target_id)) = target {
                let owner = game.card_registry[&target_id].owner;
                insts.push(SimInstruction::MoveCard {
                    card_id: target_id,
                    from: Zone::Battlefield,
                    to: Zone::Graveyard,
                    controller: owner,
                });
                let target_name = game.get_registered_card_name(target_id);
                *log = format!("{} resolves, destroying {}.", card_name, target_name);
            } else {
                *log = format!("{} resolves, but has no valid target.", card_name);
            }
        }
        _ => {}
    }
}

fn compile_object_verb_phrase(
    _what: &crate::rules_text_ast::Object,
    does: &crate::rules_text_ast::ObjectVerbPhrase,
    self_id: CardId,
    target: Option<Target>,
    _caster: PlayerId,
    game: &Game,
    insts: &mut Vec<SimInstruction>,
    log: &mut String,
) {
    let card_name = game.get_registered_card_name(self_id);
    match does {
        crate::rules_text_ast::ObjectVerbPhrase::DealsDamage { deal } => {
            let amount = match &deal.amount {
                crate::rules_text_ast::NumberDefinition::Fixed(a) => *a,
                _ => 1,
            };
            if let Some(Target::Card(target_id)) = target {
                insts.push(SimInstruction::MarkDamage { card_id: target_id, amount });
                let target_name = game.get_registered_card_name(target_id);
                *log = format!("{} resolves, dealing {} damage to {}.", card_name, amount, target_name);
            } else {
                *log = format!("{} resolves, but has no valid target.", card_name);
            }
        }
        _ => {}
    }
}

fn compile_player_verb_phrase(
    _actor: &crate::rules_text_ast::Player,
    does: &crate::rules_text_ast::PlayerVerbPhrase,
    self_id: CardId,
    _target: Option<Target>,
    caster: PlayerId,
    game: &Game,
    insts: &mut Vec<SimInstruction>,
    log: &mut String,
) {
    let card_name = game.get_registered_card_name(self_id);
    match does {
        crate::rules_text_ast::PlayerVerbPhrase::LifeGain { life_gain } => {
            let amount = match life_gain {
                crate::rules_text_ast::NumberDefinition::Fixed(a) => *a,
                _ => 1,
            };
            insts.push(SimInstruction::GainLife { player_id: caster, amount });
            let caster_name = game.players.get(&caster).map(|p| p.name.as_str()).unwrap_or("Player");
            *log = format!("{} resolves. {} gains {} life.", card_name, caster_name, amount);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rules_text_dynamic() {
        let abilities = parse_rules_text_dynamic("Runeclaw Bear", "Flying", "normal").unwrap();
        assert!(!abilities.is_empty());
        if let crate::rules_text_ast::AbilityOrRemind::Ability(crate::rules_text_ast::Ability::Keyword(keywords)) = &abilities[0] {
            assert_eq!(keywords.len(), 1);
            if let crate::rules_text_ast::KeywordAbility::Basic(name) = &keywords[0] {
                assert_eq!(name, "flying");
            } else {
                panic!("Expected Basic keyword");
            }
        } else {
            panic!("Expected Keyword ability, got: {:?}", abilities[0]);
        }
    }

    #[test]
    fn test_replacement_effects() {
        use crate::game::Format;
        use crate::player::Player;
        use crate::effects::{ReplacementEffect, EffectDuration};

        let mut game = Game::new(Format::Commander);
        let player_a = Player::new(1, "Player A".to_string(), 20);
        game.add_player(player_a);

        game.active_effects.replacement_effects.push(ReplacementEffect {
            id: 1,
            source: 100,
            duration: EffectDuration::UntilEndOfTurn,
            timestamp: 1,
            description: "if you would gain life, gain twice that much life instead".to_string(),
        });

        game.execute_instruction(SimInstruction::GainLife { player_id: 1, amount: 5 });
        assert_eq!(game.players[&1].life_total, 30); // 20 + 5 * 2 = 30
    }

    #[test]
    fn test_prevention_effects() {
        use crate::game::Format;
        use crate::effects::{PreventionEffect, PreventionScope, EffectDuration, Zone};
        use crate::zones::ZoneCard;
        use crate::card::{CardAttributes, CreatureAttributes, SpellAttributes, PermanentAttributes, PermanentStatus};

        let mut game = Game::new(Format::Commander);
        
        let target_card = Card::Creature(CreatureAttributes {
            card: CardAttributes {
                name: "Target Creature".to_string(),
                types: vec![],
                supertypes: vec![],
                subtypes: vec![],
                rules_text: "".to_string(),
                faces: None,
            },
            permanent: PermanentAttributes {
                types: vec![],
                status: PermanentStatus::default(),
            },
            spell: SpellAttributes { color: vec![], cost: vec![], cmc: 1 },
            power: "2".to_string(),
            toughness: "2".to_string(),
            damage_marked: 0,
        });
        game.card_registry.insert(10, ZoneCard { id: 10, card: target_card, owner: 1, is_token: false });
        game.zones.insert_card(game.card_registry[&10].clone(), Zone::Battlefield, 1);

        game.active_effects.prevention_effects.push(PreventionEffect {
            id: 1,
            source: 100,
            duration: EffectDuration::UntilEndOfTurn,
            timestamp: 1,
            amount_remaining: 3,
            scope: PreventionScope::DamageToTarget(Target::Card(10)),
        });

        game.execute_instruction(SimInstruction::MarkDamage { card_id: 10, amount: 5 });
        
        let perm = game.zones.battlefield.permanents.iter().find(|p| p.id == 10).unwrap();
        assert_eq!(perm.damage_marked, 2); // 5 - 3 = 2
    }

    #[test]
    fn test_text_changing_effects() {
        use crate::game::Format;
        use crate::player::Player;
        use crate::effects::{ContinuousEffect, ContinuousLayer, ContinuousEffectType, EffectDuration, Zone};
        use crate::zones::ZoneCard;
        use crate::card::{CardAttributes, CreatureAttributes, InstantAttributes, SpellAttributes, PermanentAttributes, PermanentStatus};

        let mut game = Game::new(Format::Commander);
        let player_a = Player::new(1, "Player A".to_string(), 20);
        game.add_player(player_a);

        // Target Creature
        let target_card = Card::Creature(CreatureAttributes {
            card: CardAttributes {
                name: "Target Creature".to_string(),
                types: vec![],
                supertypes: vec![],
                subtypes: vec![],
                rules_text: "".to_string(),
                faces: None,
            },
            permanent: PermanentAttributes {
                types: vec![],
                status: PermanentStatus::default(),
            },
            spell: SpellAttributes { color: vec![], cost: vec![], cmc: 1 },
            power: "2".to_string(),
            toughness: "2".to_string(),
            damage_marked: 0,
        });
        game.card_registry.insert(1, ZoneCard { id: 1, card: target_card, owner: 1, is_token: false });
        game.zones.insert_card(game.card_registry[&1].clone(), Zone::Battlefield, 1);

        // Spell with "deals 2 damage" text
        let spell_card = Card::Instant(InstantAttributes {
            card: CardAttributes {
                name: "Mock Spell".to_string(),
                types: vec![],
                supertypes: vec![],
                subtypes: vec![],
                rules_text: "Mock Spell deals 2 damage to target creature.".to_string(),
                faces: None,
            },
            spell: SpellAttributes { color: vec![], cost: vec![], cmc: 1 },
        });

        // Add Text Change effect replacing "deals 2 damage" with "deals 3 damage"
        game.active_effects.continuous_effects.push(ContinuousEffect {
            id: 100,
            source: 2,
            layer: ContinuousLayer::Layer3Text,
            duration: EffectDuration::UntilEndOfTurn,
            timestamp: 1,
            effect: ContinuousEffectType::TextChange {
                from: "deals 2 damage".to_string(),
                to: "deals 3 damage".to_string(),
            },
            conditions: vec![],
        });

        // Resolve spell targeting Card ID 1
        let (insts, _log) = get_resolution_instructions(&spell_card, 2, Some(Target::Card(1)), 1, &game);
        
        // Assert that compiled instructions now deal 3 damage instead of 2!
        let mut found_mark_damage_3 = false;
        for inst in insts {
            if let SimInstruction::MarkDamage { card_id, amount } = inst {
                if card_id == 1 && amount == 3 {
                    found_mark_damage_3 = true;
                }
            }
        }
        assert!(found_mark_damage_3, "Expected to find MarkDamage instruction dealing 3 damage due to TextChange!");
    }

    #[test]
    fn test_cost_taxation() {
        use crate::game::Format;
        use crate::player::Player;
        use crate::effects::{ContinuousEffect, ContinuousLayer, ContinuousEffectType, EffectDuration};
        use crate::card::{CardAttributes, CreatureAttributes, InstantAttributes, SpellAttributes, PermanentAttributes, PermanentStatus};

        let mut game = Game::new(Format::Commander);
        let player_a = Player::new(1, "Player A".to_string(), 20);
        game.add_player(player_a);

        // 1. Noncreature spell with printed cost {1}{U}
        let noncreature_card = Card::Instant(InstantAttributes {
            card: CardAttributes {
                name: "Mock Instant".to_string(),
                types: vec![crate::card::CardType::Instant],
                supertypes: vec![],
                subtypes: vec![],
                rules_text: "".to_string(),
                faces: None,
            },
            spell: SpellAttributes {
                color: vec![Color::U],
                cost: vec![ManaSymbols::N(1), ManaSymbols::U],
                cmc: 2,
            },
        });

        // 2. Creature spell with printed cost {1}{G}
        let creature_card = Card::Creature(CreatureAttributes {
            card: CardAttributes {
                name: "Mock Creature".to_string(),
                types: vec![crate::card::CardType::Creature],
                supertypes: vec![],
                subtypes: vec![],
                rules_text: "".to_string(),
                faces: None,
            },
            permanent: PermanentAttributes {
                types: vec![],
                status: PermanentStatus::default(),
            },
            spell: SpellAttributes {
                color: vec![Color::G],
                cost: vec![ManaSymbols::N(1), ManaSymbols::G],
                cmc: 2,
            },
            power: "2".to_string(),
            toughness: "2".to_string(),
            damage_marked: 0,
        });

        // 3. Register both card templates/instances
        game.card_registry.insert(10, crate::zones::ZoneCard { id: 10, card: noncreature_card.clone(), owner: 1, is_token: false });
        game.card_registry.insert(11, crate::zones::ZoneCard { id: 11, card: creature_card.clone(), owner: 1, is_token: false });

        // Verify printed cost first (without taxes)
        let cost_nc = get_effective_cost(&noncreature_card, 10, 1, &game);
        assert_eq!(cost_nc, vec![ManaSymbols::N(1), ManaSymbols::U]);

        let cost_cre = get_effective_cost(&creature_card, 11, 1, &game);
        assert_eq!(cost_cre, vec![ManaSymbols::N(1), ManaSymbols::G]);

        // 4. Add Thalia spell taxation effect: noncreatures cost {1} more
        game.active_effects.continuous_effects.push(ContinuousEffect {
            id: 200,
            source: 50, // Thalia card id
            layer: ContinuousLayer::Layer6Abilities,
            duration: EffectDuration::StaticAbility { source_card: 50, zone: Zone::Battlefield },
            timestamp: 10,
            effect: ContinuousEffectType::TaxSpells {
                cost_increase: 1,
            },
            conditions: vec![crate::effects::EffectCondition::Not(Box::new(crate::effects::EffectCondition::HasType(crate::card::CardType::Creature)))],
        });

        // Verify tax applies to noncreature but NOT creature
        let taxed_cost_nc = get_effective_cost(&noncreature_card, 10, 1, &game);
        assert_eq!(taxed_cost_nc, vec![ManaSymbols::N(2), ManaSymbols::U]); // 1 + 1 = 2

        let taxed_cost_cre = get_effective_cost(&creature_card, 11, 1, &game);
        assert_eq!(taxed_cost_cre, vec![ManaSymbols::N(1), ManaSymbols::G]); // Untaxed

        // Test payment checks
        let mut player_pool = {
            let player = game.players.get(&1).unwrap();
            player.mana_pool.clone()
        };

        // Player only has {1}{U} in mana pool
        player_pool.white = 0;
        player_pool.blue = 1;
        player_pool.colorless = 1; // pool has {1}{U}

        // Player CANNOT cast noncreature spell (which costs {2}{U})
        assert!(!can_pay_cost(&noncreature_card, 10, 1, &player_pool, &game));

        // Give player {2}{U}
        player_pool.colorless = 2;
        assert!(can_pay_cost(&noncreature_card, 10, 1, &player_pool, &game));
    }

    #[test]
    fn test_ability_suppression() {
        use crate::game::Format;
        use crate::player::Player;
        use crate::effects::{ContinuousEffect, ContinuousLayer, ContinuousEffectType, EffectDuration, Zone};
        use crate::zones::{ZoneCard, Permanent};
        use crate::card::{CardAttributes, LandAttributes, CreatureAttributes, SpellAttributes, PermanentAttributes, PermanentStatus};

        let mut game = Game::new(Format::Commander);
        let player_a = Player::new(1, "Player A".to_string(), 20);
        let player_b = Player::new(2, "Player B".to_string(), 20);
        game.add_player(player_a);
        game.add_player(player_b);

        // 1. Give Player B an Forest land permanent (which usually has mana ability)
        let forest = Card::Land(LandAttributes {
            card: CardAttributes {
                name: "Forest".to_string(),
                types: vec![],
                supertypes: vec![],
                subtypes: vec![],
                rules_text: "{T}: Add {G}".to_string(),
                faces: None,
            },
            permanent: PermanentAttributes {
                types: vec![],
                status: PermanentStatus::default(),
            },
        });
        game.card_registry.insert(30, ZoneCard { id: 30, card: forest.clone(), owner: 2, is_token: false });
        game.zones.battlefield.permanents.push(Permanent {
            id: 30,
            card: forest,
            owner: 2,
            controller: 2,
            status: PermanentStatus::default(),
            timestamp: 1,
            counters: std::collections::HashMap::new(),
            damage_marked: 0,
            attached_to: None,
            is_token: false,
        });

        // 2. Add Grand Abolisher suppression effect: Opponents' activated abilities are suppressed
        // Source card ID 50 is controlled by Player 1
        game.active_effects.continuous_effects.push(ContinuousEffect {
            id: 300,
            source: 50,
            layer: ContinuousLayer::Layer6Abilities,
            duration: EffectDuration::StaticAbility { source_card: 50, zone: Zone::Battlefield },
            timestamp: 2,
            effect: ContinuousEffectType::ActionRestriction {
                restrict_instructions: vec![
                    SimInstruction::CheckIsOpponent { player_id: 0, source_card_id: 50 },
                ],
            },
            conditions: vec![
                crate::effects::EffectCondition::IsOpponentOfSource,
                crate::effects::EffectCondition::IsSourceControllerTurn,
            ],
        });

        // Register the source card so we can resolve its owner/controller
        let grand_abolisher = Card::Creature(CreatureAttributes {
            card: CardAttributes {
                name: "Grand Abolisher".to_string(),
                types: vec![],
                supertypes: vec![],
                subtypes: vec![],
                rules_text: "".to_string(),
                faces: None,
            },
            permanent: PermanentAttributes {
                types: vec![],
                status: PermanentStatus::default(),
            },
            spell: SpellAttributes { color: vec![], cost: vec![], cmc: 2 },
            power: "2".to_string(),
            toughness: "2".to_string(),
            damage_marked: 0,
        });
        game.card_registry.insert(50, ZoneCard { id: 50, card: grand_abolisher, owner: 1, is_token: false });

        // 3. Verify Player 2's abilities are suppressed but Player 1's are NOT
        assert!(game.is_activated_ability_suppressed(2));
        assert!(!game.is_activated_ability_suppressed(1));

        // 4. Verify that Player 2 has NO activated abilities in possible actions
        let p2_actions = game.get_possible_actions(2);
        let has_activated = p2_actions.iter().any(|act| matches!(act, crate::actions::PriorityAction::ActivateAbility { .. }));
        assert!(!has_activated, "Expected Player 2's activated abilities to be suppressed/filtered out!");
    }
}


