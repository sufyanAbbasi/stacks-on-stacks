#[path = "mappings.rs"]
pub mod mappings;

use crate::card::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ScryfallCard {
    name: String,
    mana_cost: Option<String>,
    cmc: Option<f64>,
    type_line: Option<String>,
    oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
    defense: Option<String>,
    card_faces: Option<Vec<ScryfallCardFace>>,
}

#[derive(Debug, Deserialize)]
struct ScryfallCardFace {
    name: String,
    mana_cost: Option<String>,
    type_line: Option<String>,
    oracle_text: Option<String>,
    colors: Option<Vec<String>>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
    defense: Option<String>,
}

/// Parses a brace-delimited mana cost string (e.g. `{3}{G}{G}`) into a vector of `ManaSymbols`.
pub fn parse_mana_symbols(cost: &str) -> Vec<ManaSymbols> {
    let mut symbols = Vec::new();
    let mut chars = cost.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '{' {
            let mut sym_str = String::new();
            while let Some(&nc) = chars.peek() {
                if nc == '}' {
                    chars.next(); // consume '}'
                    break;
                } else {
                    sym_str.push(chars.next().unwrap());
                }
            }
            if let Some(sym) = map_single_mana_symbol(&sym_str) {
                symbols.push(sym);
            }
        }
    }
    symbols
}

fn map_single_mana_symbol(s: &str) -> Option<ManaSymbols> {
    let s_upper = s.to_uppercase();
    match s_upper.as_str() {
        "W" => Some(ManaSymbols::W),
        "U" => Some(ManaSymbols::U),
        "B" => Some(ManaSymbols::B),
        "R" => Some(ManaSymbols::R),
        "G" => Some(ManaSymbols::G),
        "C" => Some(ManaSymbols::C),
        "X" => Some(ManaSymbols::X),
        "S" => Some(ManaSymbols::S),
        "H" => Some(ManaSymbols::H),
        // Hybrid / Phyrexian
        "W/P" | "P/W" => Some(ManaSymbols::W_P),
        "U/P" | "P/U" => Some(ManaSymbols::U_P),
        "B/P" | "P/B" => Some(ManaSymbols::B_P),
        "R/P" | "P/R" => Some(ManaSymbols::R_P),
        "G/P" | "P/G" => Some(ManaSymbols::G_P),
        // Monocolor Hybrid (twobrid)
        "2/W" | "W/2" => Some(ManaSymbols::N_W(2)),
        "2/U" | "U/2" => Some(ManaSymbols::N_U(2)),
        "2/B" | "B/2" => Some(ManaSymbols::N_B(2)),
        "2/R" | "R/2" => Some(ManaSymbols::N_R(2)),
        "2/G" | "G/2" => Some(ManaSymbols::N_G(2)),
        // Colorless Hybrid
        "C/W" | "W/C" => Some(ManaSymbols::C_W),
        "C/U" | "U/C" => Some(ManaSymbols::C_U),
        "C/B" | "B/C" => Some(ManaSymbols::C_B),
        "C/R" | "R/C" => Some(ManaSymbols::C_R),
        "C/G" | "G/C" => Some(ManaSymbols::C_G),
        // Tri-Phyrexian / Hybrid Phyrexian
        "W/U/P" | "U/W/P" => Some(ManaSymbols::W_U_P),
        "W/B/P" | "B/W/P" => Some(ManaSymbols::W_B_P),
        "U/B/P" | "B/U/P" => Some(ManaSymbols::U_B_P),
        "U/R/P" | "R/U/P" => Some(ManaSymbols::U_R_P),
        "B/R/P" | "R/B/P" => Some(ManaSymbols::B_R_P),
        "B/G/P" | "G/B/P" => Some(ManaSymbols::B_G_P),
        "R/G/P" | "G/R/P" => Some(ManaSymbols::R_G_P),
        "R/W/P" | "W/R/P" => Some(ManaSymbols::R_W_P),
        "G/W/P" | "W/G/P" => Some(ManaSymbols::G_W_P),
        "G/U/P" | "U/G/P" => Some(ManaSymbols::G_U_P),
        // Standard Hybrids
        "W/U" | "U/W" => Some(ManaSymbols::W_U),
        "W/B" | "B/W" => Some(ManaSymbols::W_B),
        "U/B" | "B/U" => Some(ManaSymbols::U_B),
        "U/R" | "R/U" => Some(ManaSymbols::U_R),
        "B/R" | "R/B" => Some(ManaSymbols::B_R),
        "B/G" | "G/B" => Some(ManaSymbols::B_G),
        "R/G" | "G/R" => Some(ManaSymbols::R_G),
        "R/W" | "W/R" => Some(ManaSymbols::R_W),
        "G/W" | "W/G" => Some(ManaSymbols::G_W),
        "G/U" | "U/G" => Some(ManaSymbols::G_U),
        _ => {
            // Check if it is a generic/numerical mana symbol
            if let Ok(num) = s_upper.parse::<u8>() {
                Some(ManaSymbols::N(num))
            } else {
                None
            }
        }
    }
}

/// Parses an array of color abbreviations into a vector of `Color`s.
pub fn parse_colors(colors_arr: &[String]) -> Vec<Color> {
    let mut colors = Vec::new();
    for c in colors_arr {
        match c.to_uppercase().as_str() {
            "W" => colors.push(Color::W),
            "U" => colors.push(Color::U),
            "B" => colors.push(Color::B),
            "R" => colors.push(Color::R),
            "G" => colors.push(Color::G),
            "C" => colors.push(Color::C),
            _ => {}
        }
    }
    if colors.is_empty() {
        colors.push(Color::C);
    }
    colors
}

/// Parses a Scryfall `type_line` string into supertypes, card types, and subtypes.
pub fn parse_type_line(type_line: &str) -> (Vec<Supertype>, Vec<CardType>, Vec<Subtype>) {
    let mut supertypes = Vec::new();
    let mut card_types = Vec::new();
    let mut subtypes = Vec::new();

    // Split by the em-dash " — " or " - " (fallback)
    let parts: Vec<&str> = if type_line.contains(" — ") {
        type_line.split(" — ").collect()
    } else if type_line.contains(" - ") {
        type_line.split(" - ").collect()
    } else {
        vec![type_line]
    };

    // Parse supertypes and card types (left part)
    let left_part = parts[0].trim();
    let left_words: Vec<&str> = left_part.split_whitespace().collect();
    for word in left_words {
        if let Some(st) = mappings::parse_supertype(word) {
            supertypes.push(st);
        } else if let Some(ct) = mappings::parse_cardtype(word) {
            card_types.push(ct);
        }
    }

    // Parse subtypes (right part)
    if parts.len() > 1 {
        let right_part = parts[1].trim();
        let words: Vec<&str> = right_part.split_whitespace().collect();
        
        let mut i = 0;
        while i < words.len() {
            if i + 1 < words.len() && words[i].to_lowercase() == "time" && words[i+1].to_lowercase() == "lord" {
                if let Some(sub) = mappings::parse_subtype("Time Lord", &card_types) {
                    subtypes.push(sub);
                }
                i += 2;
            } else {
                if let Some(sub) = mappings::parse_subtype(words[i], &card_types) {
                    subtypes.push(sub);
                }
                i += 1;
            }
        }
    }

    (supertypes, card_types, subtypes)
}

/// Fetches a card by name from the Scryfall API and compiles it into a type-safe `Card`.
pub fn fetch_and_compile(name: &str) -> Result<Card, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("stacks-on-stacks/0.1.0")
        .build()?;
    
    let url = format!("https://api.scryfall.com/cards/named?exact={}", urlencoding::encode(name));
    let response = client.get(&url).send()?;
    
    if !response.status().is_success() {
        return Err(format!("Scryfall API returned status: {}", response.status()).into());
    }

    let scry_card: ScryfallCard = response.json()?;

    // Resolve front face properties
    let (face_name, face_cost, face_type_line, face_oracle, face_colors, face_power, face_toughness, face_loyalty, face_defense) = {
        if let Some(ref faces) = scry_card.card_faces {
            if !faces.is_empty() {
                let f = &faces[0];
                (
                    f.name.clone(),
                    f.mana_cost.clone().unwrap_or_default(),
                    f.type_line.clone().unwrap_or_default(),
                    f.oracle_text.clone().unwrap_or_default(),
                    f.colors.clone().unwrap_or_else(|| scry_card.colors.clone().unwrap_or_default()),
                    f.power.clone(),
                    f.toughness.clone(),
                    f.loyalty.clone(),
                    f.defense.clone(),
                )
            } else {
                (
                    scry_card.name.clone(),
                    scry_card.mana_cost.clone().unwrap_or_default(),
                    scry_card.type_line.clone().unwrap_or_default(),
                    scry_card.oracle_text.clone().unwrap_or_default(),
                    scry_card.colors.clone().unwrap_or_default(),
                    scry_card.power.clone(),
                    scry_card.toughness.clone(),
                    scry_card.loyalty.clone(),
                    scry_card.defense.clone(),
                )
            }
        } else {
            (
                scry_card.name.clone(),
                scry_card.mana_cost.clone().unwrap_or_default(),
                scry_card.type_line.clone().unwrap_or_default(),
                scry_card.oracle_text.clone().unwrap_or_default(),
                scry_card.colors.clone().unwrap_or_default(),
                scry_card.power.clone(),
                scry_card.toughness.clone(),
                scry_card.loyalty.clone(),
                scry_card.defense.clone(),
            )
        }
    };

    let (supertypes, card_types, subtypes) = parse_type_line(&face_type_line);

    let card_attr = CardAttributes {
        name: face_name,
        types: card_types.clone(),
        supertypes: supertypes.clone(),
        subtypes: subtypes.clone(),
        rules_text: face_oracle.clone(),
    };

    let spell_attr = SpellAttributes {
        color: parse_colors(&face_colors),
        cost: parse_mana_symbols(&face_cost),
        cmc: scry_card.cmc.unwrap_or(0.0).round() as u32,
    };

    let mut perm_types = Vec::new();
    for t in &card_types {
        match t {
            CardType::Artifact => perm_types.push(PermanentType::Artifact),
            CardType::Battle => perm_types.push(PermanentType::Battle),
            CardType::Creature => perm_types.push(PermanentType::Creature),
            CardType::Enchantment => perm_types.push(PermanentType::Enchantment),
            CardType::Land => perm_types.push(PermanentType::Land),
            CardType::Planeswalker => perm_types.push(PermanentType::Planeswalker),
            _ => {}
        }
    }
    let perm_attr = PermanentAttributes {
        types: perm_types,
        status: PermanentStatus::default(),
    };

    // Construct appropriate Card variant based on priorities
    if card_types.contains(&CardType::Planeswalker) {
        let loyalty_counters = face_loyalty
            .as_deref()
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        Ok(Card::Planeswalker(PlaneswalkerAttributes {
            card: card_attr,
            permanent: perm_attr,
            loyalty_counters,
        }))
    } else if card_types.contains(&CardType::Creature) {
        if face_oracle.to_lowercase().contains("level up") {
            Ok(Card::Leveler(LevelerAttributes {
                card: card_attr,
                permanent: perm_attr,
                spell: spell_attr,
                level_counters: 0,
            }))
        } else {
            let p = face_power.unwrap_or_else(|| "0".to_string());
            let t = face_toughness.unwrap_or_else(|| "0".to_string());
            Ok(Card::Creature(CreatureAttributes {
                card: card_attr,
                permanent: perm_attr,
                spell: spell_attr,
                power: p,
                toughness: t,
                damage_marked: 0,
            }))
        }
    } else if card_types.contains(&CardType::Enchantment)
        && subtypes.iter().any(|s| matches!(s, Subtype::Enchantment(EnchantmentType::Saga)))
    {
        Ok(Card::Saga(SagaAttributes {
            card: card_attr,
            permanent: perm_attr,
            spell: spell_attr,
            lore_counters: 0,
        }))
    } else if card_types.contains(&CardType::Enchantment)
        && subtypes.iter().any(|s| matches!(s, Subtype::Enchantment(EnchantmentType::Class)))
    {
        Ok(Card::Class(ClassAttributes {
            card: card_attr,
            permanent: perm_attr,
            spell: spell_attr,
            class_level: 1,
        }))
    } else if card_types.contains(&CardType::Enchantment) {
        Ok(Card::Enchantment(EnchantmentAttributes {
            card: card_attr,
            permanent: perm_attr,
            spell: spell_attr,
        }))
    } else if card_types.contains(&CardType::Artifact) {
        Ok(Card::Artifact(ArtifactAttributes {
            card: card_attr,
            permanent: perm_attr,
            spell: spell_attr,
        }))
    } else if card_types.contains(&CardType::Land) {
        Ok(Card::Land(LandAttributes {
            card: card_attr,
            permanent: perm_attr,
        }))
    } else if card_types.contains(&CardType::Battle) {
        let defense = face_defense
            .as_deref()
            .unwrap_or("0")
            .parse::<u32>()
            .unwrap_or(0);
        Ok(Card::Battle(BattleAttributes {
            card: card_attr,
            permanent: perm_attr,
            spell: spell_attr,
            defense,
        }))
    } else if card_types.contains(&CardType::Instant) {
        Ok(Card::Instant(InstantAttributes {
            card: card_attr,
            spell: spell_attr,
        }))
    } else if card_types.contains(&CardType::Sorcery) {
        Ok(Card::Sorcery(SorceryAttributes {
            card: card_attr,
            spell: spell_attr,
        }))
    } else if card_types.contains(&CardType::Kindred) {
        Ok(Card::Kindred(KindredAttributes {
            card: card_attr,
            spell: spell_attr,
        }))
    } else {
        if !perm_attr.types.is_empty() {
            Ok(Card::Artifact(ArtifactAttributes {
                card: card_attr,
                permanent: perm_attr,
                spell: spell_attr,
            }))
        } else {
            Ok(Card::Sorcery(SorceryAttributes {
                card: card_attr,
                spell: spell_attr,
            }))
        }
    }
}
