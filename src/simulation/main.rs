pub mod card;
pub mod player;
pub mod game;
pub mod effects;
pub mod turns;
pub mod actions;
pub mod stack;
#[path = "../compiler/rules.rs"]
pub mod rules;
#[path = "../compiler/scryfall.rs"]
pub mod scryfall;

fn main() {
    println!("Stacks-on-Stacks: Magic the Gathering Rule Interpreter");
    println!("---------------------------------------------------------");

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let name = &args[1];
        println!("Fetching and compiling: {}...", name);
        match scryfall::fetch_and_compile(name) {
            Ok(card) => {
                println!("SUCCESSFULLY COMPILED:");
                println!("{:#?}", card);
            }
            Err(e) => {
                println!("ERROR compiling card: {}", e);
            }
        }
    } else {
        // Run some default demo cases to verify everything works flawlessly
        let demo_cards = vec![
            "Delver of Secrets",
            "Counterspell",
            "Sol Ring",
            "Sylvan Library",
            "Chandra, Torch of Defiance",
            "Invasion of Ravnica",
            "Dryad of the Ilysian Grove",
        ];
        
        for name in demo_cards {
            println!("\nFetching and compiling: {}...", name);
            match scryfall::fetch_and_compile(name) {
                Ok(card) => {
                    println!("Compiled successfully!");
                    match card {
                        card::Card::Creature(ref attrs) => {
                            println!("  [Creature] {}: {}/{} (Cost: {:?}, Types: {:?}, Subtypes: {:?})", 
                                attrs.card.name, attrs.power, attrs.toughness, attrs.spell.cost, attrs.card.types, attrs.card.subtypes);
                        }
                        card::Card::Instant(ref attrs) => {
                            println!("  [Instant] {}: Cost: {:?} (Types: {:?})", 
                                attrs.card.name, attrs.spell.cost, attrs.card.types);
                        }
                        card::Card::Sorcery(ref attrs) => {
                            println!("  [Sorcery] {}: Cost: {:?} (Types: {:?})", 
                                attrs.card.name, attrs.spell.cost, attrs.card.types);
                        }
                        card::Card::Land(ref attrs) => {
                            println!("  [Land] {} (Types: {:?}, Subtypes: {:?})", 
                                attrs.card.name, attrs.card.types, attrs.card.subtypes);
                        }
                        card::Card::Artifact(ref attrs) => {
                            println!("  [Artifact] {}: Cost: {:?} (Types: {:?})", 
                                attrs.card.name, attrs.spell.cost, attrs.card.types);
                        }
                        card::Card::Enchantment(ref attrs) => {
                            println!("  [Enchantment] {}: Cost: {:?} (Types: {:?})", 
                                attrs.card.name, attrs.spell.cost, attrs.card.types);
                        }
                        card::Card::Planeswalker(ref attrs) => {
                            println!("  [Planeswalker] {}: Loyalty: {} (Types: {:?})", 
                                attrs.card.name, attrs.loyalty_counters, attrs.card.types);
                        }
                        card::Card::Battle(ref attrs) => {
                            println!("  [Battle] {}: Defense: {} (Types: {:?})", 
                                attrs.card.name, attrs.defense, attrs.card.types);
                        }
                        _ => {
                            println!("  [Other Card Type] {:#?}", card);
                        }
                    }
                }
                Err(e) => {
                    println!("  ERROR compiling card: {}", e);
                }
            }
        }
        println!("\nTip: Run `cargo run -- <card-name>` to compile any specific card!");
    }
}
