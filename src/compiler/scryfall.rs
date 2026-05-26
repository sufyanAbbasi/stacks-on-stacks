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

fn format_subtype(s: &Subtype) -> String {
    let s_str = format!("{:?}", s);
    if let Some(start) = s_str.find('(') {
        if let Some(end) = s_str.rfind(')') {
            return s_str[start+1..end].to_string();
        }
    }
    s_str
}

fn format_mana_symbol(m: &ManaSymbols) -> String {
    match m {
        ManaSymbols::W => "W".to_string(),
        ManaSymbols::U => "U".to_string(),
        ManaSymbols::B => "B".to_string(),
        ManaSymbols::R => "R".to_string(),
        ManaSymbols::G => "G".to_string(),
        ManaSymbols::C => "C".to_string(),
        ManaSymbols::X => "X".to_string(),
        ManaSymbols::S => "S".to_string(),
        ManaSymbols::H => "H".to_string(),
        ManaSymbols::N(n) => n.to_string(),
        ManaSymbols::W_U => "W/U".to_string(),
        ManaSymbols::W_B => "W/B".to_string(),
        ManaSymbols::U_B => "U/B".to_string(),
        ManaSymbols::U_R => "U/R".to_string(),
        ManaSymbols::B_R => "B/R".to_string(),
        ManaSymbols::B_G => "B/G".to_string(),
        ManaSymbols::R_G => "R/G".to_string(),
        ManaSymbols::R_W => "R/W".to_string(),
        ManaSymbols::G_W => "G/W".to_string(),
        ManaSymbols::G_U => "G/U".to_string(),
        ManaSymbols::N_W(n) => format!("{}/W", n),
        ManaSymbols::N_U(n) => format!("{}/U", n),
        ManaSymbols::N_B(n) => format!("{}/B", n),
        ManaSymbols::N_R(n) => format!("{}/R", n),
        ManaSymbols::N_G(n) => format!("{}/G", n),
        ManaSymbols::C_W => "C/W".to_string(),
        ManaSymbols::C_U => "C/U".to_string(),
        ManaSymbols::C_B => "C/B".to_string(),
        ManaSymbols::C_R => "C/R".to_string(),
        ManaSymbols::C_G => "C/G".to_string(),
        ManaSymbols::W_P => "W/P".to_string(),
        ManaSymbols::U_P => "U/P".to_string(),
        ManaSymbols::B_P => "B/P".to_string(),
        ManaSymbols::R_P => "R/P".to_string(),
        ManaSymbols::G_P => "G/P".to_string(),
        ManaSymbols::W_U_P => "W/U/P".to_string(),
        ManaSymbols::W_B_P => "W/B/P".to_string(),
        ManaSymbols::U_B_P => "U/B/P".to_string(),
        ManaSymbols::U_R_P => "U/R/P".to_string(),
        ManaSymbols::B_R_P => "B/R/P".to_string(),
        ManaSymbols::B_G_P => "B/G/P".to_string(),
        ManaSymbols::R_G_P => "R/G/P".to_string(),
        ManaSymbols::R_W_P => "R/W/P".to_string(),
        ManaSymbols::G_W_P => "G/W/P".to_string(),
        ManaSymbols::G_U_P => "G/U/P".to_string(),
    }
}

fn format_mana_cost(cost: &[ManaSymbols]) -> String {
    cost.iter().map(|m| format!("{{{}}}", format_mana_symbol(m))).collect()
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut current_line = String::new();
        for word in paragraph.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }
    }
    lines
}

fn get_card_info(card: &Card) -> (&CardAttributes, Option<&SpellAttributes>, Option<String>) {
    match card {
        Card::Artifact(attrs) => (&attrs.card, Some(&attrs.spell), None),
        Card::Battle(attrs) => (&attrs.card, Some(&attrs.spell), Some(format!("DF:{}", attrs.defense))),
        Card::Creature(attrs) => (&attrs.card, Some(&attrs.spell), Some(format!("{}/{}", attrs.power, attrs.toughness))),
        Card::Enchantment(attrs) => (&attrs.card, Some(&attrs.spell), None),
        Card::Instant(attrs) => (&attrs.card, Some(&attrs.spell), None),
        Card::Land(attrs) => (&attrs.card, None, None),
        Card::Planeswalker(attrs) => (&attrs.card, None, Some(format!("LY:{}", attrs.loyalty_counters))),
        Card::Sorcery(attrs) => (&attrs.card, Some(&attrs.spell), None),
        Card::Kindred(attrs) => (&attrs.card, Some(&attrs.spell), None),
        Card::Leveler(attrs) => (&attrs.card, Some(&attrs.spell), Some(format!("LV:{}", attrs.level_counters))),
        Card::Saga(attrs) => (&attrs.card, Some(&attrs.spell), Some(format!("SG:{}", attrs.lore_counters))),
        Card::Class(attrs) => (&attrs.card, Some(&attrs.spell), Some(format!("CL:{}", attrs.class_level))),
    }
}

fn colorize_symbols(s: &str) -> String {
    let mut result = s.to_string();
    result = result.replace("{W}", "\x1b[1;37m{W}\x1b[0m");
    result = result.replace("{U}", "\x1b[1;34m{U}\x1b[0m");
    result = result.replace("{B}", "\x1b[1;30m{B}\x1b[0m");
    result = result.replace("{R}", "\x1b[1;31m{R}\x1b[0m");
    result = result.replace("{G}", "\x1b[1;32m{G}\x1b[0m");
    result = result.replace("{C}", "\x1b[1;36m{C}\x1b[0m");
    result = result.replace("{X}", "\x1b[1;35m{X}\x1b[0m");
    result = result.replace("{T}", "\x1b[1;33m{T}\x1b[0m");
    result = result.replace("{Q}", "\x1b[1;33m{Q}\x1b[0m");
    
    // Replace numerical generic costs {0} to {20}
    for n in 0..=20 {
        let from = format!("{{{}}}", n);
        let to = format!("\x1b[1;36m{{{}}}\x1b[0m", n);
        result = result.replace(&from, &to);
    }
    result
}

fn get_styled_stat_box(stat: &str) -> (String, usize) {
    if stat.is_empty() {
        return ("".to_string(), 0);
    }
    
    if stat.starts_with("DF:") {
        let val = &stat[3..];
        let text = format!("[ ◆ {} ]", val);
        let colored = format!("\x1b[1;31m[ \x1b[31m◆\x1b[1;31m {} ]\x1b[0m", val);
        (colored, text.chars().count())
    } else if stat.starts_with("LY:") {
        let val = &stat[3..];
        let text = format!("[ ✦ {} ]", val);
        let colored = format!("\x1b[1;33m[ \x1b[33m✦\x1b[1;33m {} ]\x1b[0m", val);
        (colored, text.chars().count())
    } else if stat.starts_with("SG:") {
        let val = &stat[3..];
        let roman = match val {
            "1" => "I",
            "2" => "II",
            "3" => "III",
            "4" => "IV",
            "5" => "V",
            _ => val,
        };
        let text = format!("[ Chapter {} ]", roman);
        let colored = format!("\x1b[1;35m[ Chapter {} ]\x1b[0m", roman);
        (colored, text.chars().count())
    } else if stat.starts_with("CL:") {
        let val = &stat[3..];
        let text = format!("[ Lvl {} ]", val);
        let colored = format!("\x1b[1;36m[ Lvl {} ]\x1b[0m", val);
        (colored, text.chars().count())
    } else if stat.starts_with("LV:") {
        let val = &stat[3..];
        let text = format!("[ Lvl {} ]", val);
        let colored = format!("\x1b[1;36m[ Lvl {} ]\x1b[0m", val);
        (colored, text.chars().count())
    } else {
        let text = format!("[ {} ]", stat);
        let colored = format!("\x1b[1m[ {} ]\x1b[0m", stat);
        (colored, text.chars().count())
    }
}

fn print_ascii_card(card: &Card) {
    let (card_attr, spell_attr, stat_box) = get_card_info(card);

    let name = &card_attr.name;
    let cost_str = if let Some(sa) = spell_attr {
        format_mana_cost(&sa.cost)
    } else {
        "".to_string()
    };
    
    let name_cost_line = if cost_str.is_empty() {
        format!("{:<36}", name)
    } else {
        let name_len = name.len();
        let cost_len = cost_str.len();
        if name_len + 1 + cost_len <= 36 {
            let spaces = 36 - name_len - cost_len;
            format!("{}{}{}", name, " ".repeat(spaces), cost_str)
        } else {
            let truncated_name = if name_len > 20 { &name[0..17] } else { name };
            let spaces = 36 - truncated_name.len() - cost_len;
            format!("{}{}{}", truncated_name, " ".repeat(spaces), cost_str)
        }
    };

    let super_types_str: Vec<String> = card_attr.supertypes.iter().map(|s| format!("{:?}", s)).collect();
    let card_types_str: Vec<String> = card_attr.types.iter().map(|t| format!("{:?}", t)).collect();
    let subtypes_str: Vec<String> = card_attr.subtypes.iter().map(|s| format_subtype(s)).collect();
    
    let left_types = if super_types_str.is_empty() {
        card_types_str.join(" ")
    } else {
        format!("{} {}", super_types_str.join(" "), card_types_str.join(" "))
    };
    
    let mut left_types_plain = left_types.clone();
    let mut left_types_colored = left_types.clone();
    let reset_ansi = "\x1b[0m";
    
    if let Some(sa) = spell_attr {
        if sa.cost.is_empty() && !sa.color.is_empty() {
            left_types_plain = format!("● {}", left_types);
            let circle_color = if sa.color.len() > 1 {
                "\x1b[38;5;178m" // Gold
            } else if sa.color.contains(&Color::U) {
                "\x1b[1;34m"
            } else if sa.color.contains(&Color::G) {
                "\x1b[1;32m"
            } else if sa.color.contains(&Color::R) {
                "\x1b[1;31m"
            } else if sa.color.contains(&Color::W) {
                "\x1b[1;37m"
            } else if sa.color.contains(&Color::B) {
                "\x1b[1;30m"
            } else {
                "\x1b[1;36m"
            };
            left_types_colored = format!("{}{}●\x1b[0m {}", circle_color, reset_ansi, left_types);
        }
    }
    
    let full_type_plain = if subtypes_str.is_empty() {
        left_types_plain
    } else {
        format!("{} — {}", left_types_plain, subtypes_str.join(" "))
    };
    
    let full_type_colored = if subtypes_str.is_empty() {
        left_types_colored
    } else {
        format!("{} — {}", left_types_colored, subtypes_str.join(" "))
    };
    
    let set_plain = "[STK]";
    let set_colored = "\x1b[1;33m[STK]\x1b[0m"; // Gold set expansion symbol
    
    let type_line_len = full_type_plain.chars().count();
    let set_len = set_plain.len();
    
    let formatted_type_line = if type_line_len + 1 + set_len <= 36 {
        let spaces = 36 - type_line_len - set_len;
        format!("{}{}{}", full_type_colored, " ".repeat(spaces), set_colored)
    } else {
        let max_len = 36 - 1 - set_len;
        let truncated_plain = if type_line_len > max_len {
            full_type_plain.chars().take(max_len).collect::<String>()
        } else {
            full_type_plain.clone()
        };
        let spaces = 36 - truncated_plain.chars().count() - set_len;
        format!("{}{}{}", truncated_plain, " ".repeat(spaces), set_colored)
    };

    let mut rules_lines = wrap_text(&card_attr.rules_text, 36);
    while rules_lines.len() < 4 {
        rules_lines.push("".to_string());
    }

    let has_stat = stat_box.is_some();
    let stat_str = stat_box.unwrap_or_default();
    let (stat_colored, stat_visible_len) = get_styled_stat_box(&stat_str);
    
    let color_ansi = if card_attr.types.contains(&CardType::Land) {
        "\x1b[1;33m" // Bold Yellow-brown for Lands
    } else if let Some(sa) = spell_attr {
        if sa.color.len() > 1 {
            "\x1b[38;5;178m" // Bold Gold for Multicolored cards
        } else if sa.color.contains(&Color::U) {
            "\x1b[1;34m" // Bold Blue
        } else if sa.color.contains(&Color::G) {
            "\x1b[1;32m" // Bold Green
        } else if sa.color.contains(&Color::R) {
            "\x1b[1;31m" // Bold Red
        } else if sa.color.contains(&Color::W) {
            "\x1b[1;37m" // Bold White
        } else if sa.color.contains(&Color::B) {
            "\x1b[1;30m" // Bold Black (Dark Gray)
        } else {
            "\x1b[1;36m" // Bold Cyan for colorless
        }
    } else {
        "\x1b[1;36m" // Bold Cyan default
    };

    let colorized_name_cost = colorize_symbols(&name_cost_line);

    // Render beautiful cards using Unicode rounded-corner box-drawing borders.
    // 38 horizontal lines corresponds exactly to 36-characters content + 2-characters interior space padding.
    println!("  {}╭──────────────────────────────────────╮{}", color_ansi, reset_ansi);
    println!("  {}│\x1b[0m {} {}│{}", color_ansi, colorized_name_cost, color_ansi, reset_ansi);
    println!("  {}├──────────────────────────────────────┤{}", color_ansi, reset_ansi);
    println!("  {}│\x1b[0m {} {}│{}", color_ansi, formatted_type_line, color_ansi, reset_ansi);
    println!("  {}├──────────────────────────────────────┤{}", color_ansi, reset_ansi);
    
    for (i, line) in rules_lines.iter().enumerate() {
        if i == rules_lines.len() - 1 && has_stat {
            let line_visible_len = line.chars().count();
            if line_visible_len + 2 + stat_visible_len <= 36 {
                let spaces_count = 36 - line_visible_len - stat_visible_len;
                let colorized_line = colorize_symbols(line);
                println!(
                    "  {}│\x1b[0m {}{}{} {}\x1b[0m{}│{}", 
                    color_ansi, colorized_line, " ".repeat(spaces_count), stat_colored, " ", color_ansi, reset_ansi
                );
            } else {
                let colorized_line = colorize_symbols(&format!("{:<36}", line));
                println!("  {}│\x1b[0m {} {}│{}", color_ansi, colorized_line, color_ansi, reset_ansi);
                
                let spaces_count = 36 - stat_visible_len;
                println!(
                    "  {}│\x1b[0m {}{} {}\x1b[0m{}│{}", 
                    color_ansi, " ".repeat(spaces_count), stat_colored, " ", color_ansi, reset_ansi
                );
            }
        } else {
            let padded_line = format!("{:<36}", line);
            let colorized_line = colorize_symbols(&padded_line);
            println!("  {}│\x1b[0m {} {}│{}", color_ansi, colorized_line, color_ansi, reset_ansi);
        }
    }
    
    println!("  {}╰──────────────────────────────────────╯{}", color_ansi, reset_ansi);
}

/// Dynamically fetches card details from the Scryfall API, constructs, prints an ASCII representation, and returns the Card.
pub fn create_test_card(name: &str) -> Card {
    println!("\x1b[36m[SCRYFALL FETCH]\x1b[0m Fetching '{}' from Scryfall API...", name);
    match fetch_and_compile(name) {
        Ok(card) => {
            print_ascii_card(&card);
            card
        }
        Err(e) => panic!("Failed to compile card '{}' from Scryfall: {}", name, e),
    }
}
