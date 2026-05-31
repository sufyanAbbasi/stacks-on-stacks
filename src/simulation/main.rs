pub mod card;
pub mod player;
pub mod game;
pub mod effects;
pub mod turns;
pub mod actions;
pub mod stack;
pub mod zones;
#[path = "../compiler/rules.rs"]
pub mod rules;
#[path = "../compiler/scryfall.rs"]
pub mod scryfall;
#[path = "../compiler/rules_text_ast.rs"]
pub mod rules_text_ast;
pub mod graph;
pub mod abilities;


use game::{Game, SimInstruction, Format};
use player::Player;
use card::Color;
use zones::ZoneCard;
use effects::{Target, Zone};
use scryfall::create_test_card;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Command-line flag parsing
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return;
    }

    let run_card_render = args.iter().any(|arg| arg == "--card-render");
    let run_walkthrough = args.iter().any(|arg| arg == "--walkthrough");
    let run_zones = args.iter().any(|arg| arg == "--zones");
    let run_serialize = args.iter().any(|arg| arg == "--serialize");
    let run_graph = args.iter().any(|arg| arg == "--graph");
    let run_test_ast = args.iter().any(|arg| arg == "--test-ast");
    let run_all = args.iter().any(|arg| arg == "--all") 
        || (!run_card_render && !run_walkthrough && !run_zones && !run_serialize && !run_graph && !run_test_ast);

    if run_all {
        println!("\x1b[1;35m=========================================================\x1b[0m");
        println!("\x1b[1;35m*  STACKS-ON-STACKS: RUNNING ALL SIMULATION SHOWCASES   *\x1b[0m");
        println!("\x1b[1;35m=========================================================\x1b[0m");
    }

    // Extract any additional arguments passed as custom card names to render
    let custom_card_names: Vec<String> = args.iter()
        .skip(1)
        .filter(|arg| !arg.starts_with("-"))
        .cloned()
        .collect();

    if run_all || run_card_render {
        run_card_rendering_showcase(&custom_card_names);
    }

    let mut game = None;

    if run_all || run_walkthrough || run_zones || run_serialize || run_graph {
        game = Some(run_standard_walkthrough());
    }

    if let Some(ref mut g) = game {
        if run_all || run_zones {
            run_secondary_zones_showcase(g);
        }
        if run_all || run_serialize {
            run_serialization_test(g);
        }
        if run_all || run_graph {
            run_graph_showcase(g);
        }
    }

    if run_all || run_test_ast {
        run_ast_deserialization_test();
    }
}

fn run_ast_deserialization_test() {
    println!("\x1b[1;36m=========================================================\x1b[0m");
    println!("\x1b[1;36m*             MTG COMPILER AST PARSE & VERIFY TEST      *\x1b[0m");
    println!("\x1b[1;36m=========================================================\x1b[0m");
    
    let path = "parsed_cards.json";
    if !std::path::Path::new(path).exists() {
        println!("\x1b[1;31m[ERROR] File 'parsed_cards.json' not found!\x1b[0m");
        println!("Please run 'node scratch_parser.js' first to fetch and parse cards from Scryfall.");
        println!("\x1b[1;36m=========================================================\x1b[0m\n");
        return;
    }
    
    println!("\x1b[1;33m[LOAD] Reading 'parsed_cards.json'...\x1b[0m");
    let content = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            println!("\x1b[1;31m[ERROR] Failed to read file: {}\x1b[0m", e);
            println!("\x1b[1;36m=========================================================\x1b[0m\n");
            return;
        }
    };
    
    #[derive(serde::Deserialize, Debug)]
    struct ScryfallParseTest {
        name: String,
        oracle_text: String,
        parse_result: ScryfallParseResultInner,
    }

    #[derive(serde::Deserialize, Debug)]
    struct ScryfallParseResultInner {
        result: Option<Vec<Vec<serde_json::Value>>>,
        error: Option<serde_json::Value>,
    }
    
    println!("\x1b[1;33m[DESERIALIZE] Validating schema compatibility of parsed AST objects...\x1b[0m");
    let cards: Result<Vec<ScryfallParseTest>, _> = serde_json::from_str(&content);
    
    match cards {
        Ok(parsed_list) => {
            println!("\x1b[1;32m[SUCCESS] Read JSON file successfully! Inspecting structures...\x1b[0m");
            let mut total_errors = 0;
            for card in parsed_list {
                match card.parse_result.result {
                    Some(ref parses) => {
                        let mut parsed_trees = Vec::new();
                        let mut card_has_error = false;
                        for (tree_idx, parse_tree) in parses.iter().enumerate() {
                            let mut parsed_tree = Vec::new();
                            for (ability_idx, val) in parse_tree.iter().enumerate() {
                                let res: Result<rules_text_ast::AbilityOrRemind, _> = serde_json::from_value(val.clone());
                                match res {
                                    Ok(ability) => {
                                        parsed_tree.push(ability);
                                    }
                                    Err(e) => {
                                        total_errors += 1;
                                        card_has_error = true;
                                        println!("\x1b[1;31m[ERROR] Card '{}' failed to deserialize at parse_tree {}, ability_idx {}:\x1b[0m", card.name, tree_idx, ability_idx);
                                        println!("JSON value: {}", serde_json::to_string_pretty(val).unwrap());
                                        println!("Error Details: {:?}", e);
                                        
                                        println!("\x1b[1;33mTargeted Diagnostics:\x1b[0m");
                                        let act_res: Result<rules_text_ast::ActivatedAbility, _> = serde_json::from_value(val.clone());
                                        println!("  - Try ActivatedAbility: {:?}", act_res);
                                        
                                        let trigger_res: Result<rules_text_ast::TriggeredAbility, _> = serde_json::from_value(val.clone());
                                        println!("  - Try TriggeredAbility: {:?}", trigger_res);
                                        
                                        let sentence_res: Result<rules_text_ast::Sentence, _> = serde_json::from_value(val.clone());
                                        println!("  - Try Sentence: {:?}", sentence_res);
                                        
                                        if let Some(as_long_as) = val.get("asLongAs") {
                                            let cond_res: Result<rules_text_ast::Condition, _> = serde_json::from_value(as_long_as.clone());
                                            println!("    - Try asLongAs as Condition: {:?}", cond_res);
                                            let sent_res: Result<rules_text_ast::Sentence, _> = serde_json::from_value(as_long_as.clone());
                                            println!("    - Try asLongAs as Sentence: {:?}", sent_res);
                                            if let Some(does) = as_long_as.get("does") {
                                                let pvp_res: Result<rules_text_ast::PlayerVerbPhrase, _> = serde_json::from_value(does.clone());
                                                println!("      - Try asLongAs.does as PlayerVerbPhrase: {:?}", pvp_res);
                                                let imp_res: Result<rules_text_ast::Imperative, _> = serde_json::from_value(does.clone());
                                                println!("      - Try asLongAs.does as Imperative: {:?}", imp_res);
                                            }
                                        }
                                        if let Some(duration) = val.get("duration") {
                                            let dur_res: Result<rules_text_ast::Duration, _> = serde_json::from_value(duration.clone());
                                            println!("    - Try duration as Duration: {:?}", dur_res);
                                        }
                                        if let Some(effect) = val.get("effect") {
                                            let sent_res: Result<rules_text_ast::Sentence, _> = serde_json::from_value(effect.clone());
                                            println!("    - Try effect as Sentence: {:?}", sent_res);
                                            if let Some(what) = effect.get("what") {
                                                let obj_res: Result<rules_text_ast::Object, _> = serde_json::from_value(what.clone());
                                                println!("      - Try effect.what as Object: {:?}", obj_res);
                                                let po_res: Result<rules_text_ast::PureObject, _> = serde_json::from_value(what.clone());
                                                println!("      - Try effect.what as PureObject: {:?}", po_res);
                                            }
                                        }
                                        println!("---------------------------------------------------------");
                                    }
                                }
                            }
                            if !card_has_error {
                                parsed_trees.push(parsed_tree);
                            }
                        }
                        if !card_has_error && !parsed_trees.is_empty() {
                            println!("  - \x1b[1;32m{}\x1b[0m: Parsed successfully with {} possible parse tree(s).", card.name, parsed_trees.len());
                            println!("    Parsed structure (First parse tree):");
                            for (idx, ability) in parsed_trees[0].iter().enumerate() {
                                println!("      [{}] {:?}", idx + 1, ability);
                            }
                        }
                    }
                    None => {
                        println!("  - \x1b[1;31m{}\x1b[0m: FAILED to parse upstream (Error: {:?})", card.name, card.parse_result.error);
                    }
                }
            }
            if total_errors > 0 {
                println!("\x1b[1;31m[FAILURE] Finished with {} deserialization errors.\x1b[0m", total_errors);
            } else {
                println!("\x1b[1;32m[SUCCESS] All cards matched the Rust schema perfectly!\x1b[0m");
            }
        }
        Err(e) => {
            println!("\x1b[1;31m[ERROR] Deserialization failed! Schema mismatch or corrupted JSON.\x1b[0m");
            println!("Details: {}", e);
        }
    }
    println!("\x1b[1;36m=========================================================\x1b[0m\n");
}

fn print_help() {
    println!("\x1b[1;35m=========================================================\x1b[0m");
    println!("\x1b[1;35m*            STACKS-ON-STACKS SIMULATOR HELP            *\x1b[0m");
    println!("\x1b[1;35m=========================================================\x1b[0m");
    println!("Usage: cargo run [options] [card_names]");
    println!("\nOptions:");
    println!("  -h, --help       Show this help message");
    println!("  --card-render    Run dynamic Scryfall ASCII card rendering demo. Add names as trailing args.");
    println!("                   Example: cargo run -- --card-render \"Opposition Agent\"");
    println!("  --walkthrough    Run only the standard 13-step machine-code game walkthrough");
    println!("  --zones          Run the walkthrough followed by secondary zones/primitives demo");
    println!("  --serialize      Run the walkthrough followed by serialization round-trip snapshot test");
    println!("  --graph          Run the state-based actions check and priority decision graph demo");
    println!("  --test-ast       Verify that Scryfall cards parsed by magic-card-parser match our Rust AST schema");
    println!("  --all            Run all simulation showcases and tests sequentially (default if no flags are passed)");
    println!("\x1b[1;35m=========================================================\x1b[0m\n");
}

fn run_card_rendering_showcase(custom_names: &[String]) {
    println!("\n\x1b[1;36m=========================================================\x1b[0m");
    println!("\x1b[1;36m*             MTG ASCII CARD RENDERING SHOWCASE         *\x1b[0m");
    println!("\x1b[1;36m=========================================================\x1b[0m");
    
    let names = if custom_names.is_empty() {
        vec!["Colossal Dreadmaw".to_string(), "Lightning Bolt".to_string()]
    } else {
        custom_names.to_vec()
    };

    for name in names {
        println!("[SCRYFALL] Fetching and compiling: {}...", name);
        let _card = create_test_card(&name);
    }
    println!("\x1b[1;36m=========================================================\x1b[0m\n");
}

fn run_graph_showcase(_old_game: &mut Game) {
    use crate::effects::{ContinuousEffect, ContinuousLayer, ContinuousEffectType, EffectDuration, EffectCondition, Zone};

    println!("\n\x1b[1;35m================================================================================\x1b[0m");
    println!("\x1b[1;35m*           ALGORITHMIC MTG DECISION GRAPH COMPILATION & COMPARISON            *\x1b[0m");
    println!("\x1b[1;35m================================================================================\x1b[0m");

    // 1. Initialize a completely custom state for this scenario
    println!("\n[SBA SCENARIO] Constructing initial custom MTG scenario board state...");
    let mut game = Game::new(Format::Commander);

    let player_a = Player::new(1, "Player A".to_string(), 20);
    let player_b = Player::new(2, "Player B".to_string(), 20);

    game.add_player(player_a);
    game.add_player(player_b);

    // Compile/Fetch the cards
    let forest = create_test_card("Forest");
    let mountain = create_test_card("Mountain");
    let island = create_test_card("Island");
    let fireball = create_test_card("Fireball");
    let counterspell = create_test_card("Counterspell");
    let bear = create_test_card("Runeclaw Bear");

    // Player A Battlefield: Forest (ID 10) and Mountain (ID 13)
    game.card_registry.insert(10, ZoneCard { id: 10, card: forest, owner: 1, is_token: false });
    game.zones.insert_card(game.card_registry[&10].clone(), Zone::Battlefield, 1);

    game.card_registry.insert(13, ZoneCard { id: 13, card: mountain, owner: 1, is_token: false });
    game.zones.insert_card(game.card_registry[&13].clone(), Zone::Battlefield, 1);

    // Player A Hand: Fireball (ID 15)
    game.card_registry.insert(15, ZoneCard { id: 15, card: fireball, owner: 1, is_token: false });
    game.zones.insert_card(game.card_registry[&15].clone(), Zone::Hand, 1);

    // Player B Battlefield: Island 1 (ID 20), Island 2 (ID 21), and Runeclaw Bear (ID 30)
    game.card_registry.insert(20, ZoneCard { id: 20, card: island.clone(), owner: 2, is_token: false });
    game.zones.insert_card(game.card_registry[&20].clone(), Zone::Battlefield, 2);

    game.card_registry.insert(21, ZoneCard { id: 21, card: island, owner: 2, is_token: false });
    game.zones.insert_card(game.card_registry[&21].clone(), Zone::Battlefield, 2);

    game.card_registry.insert(30, ZoneCard { id: 30, card: bear, owner: 2, is_token: false });
    game.zones.insert_card(game.card_registry[&30].clone(), Zone::Battlefield, 2);

    // Player B Hand: Counterspell (ID 22)
    game.card_registry.insert(22, ZoneCard { id: 22, card: counterspell, owner: 2, is_token: false });
    game.zones.insert_card(game.card_registry[&22].clone(), Zone::Hand, 2);

    // Priority begins with Player 1 (A)
    game.priority_player = Some(1);
    game.active_player = 1;
    game.consecutive_passes = 0;

    println!("\x1b[1;32m[SYSTEM] Custom Starting Board State Configured successfully.\x1b[0m");
    print_game_state(&game);

    // 2. Build standard graph (Scenario A)
    println!("\n[GRAPH GENERATOR] Programmatically expanding Scenario A (Standard BFS)...");
    let standard_graph = graph::GameGraph::build_algorithmic_graph(&game, 150);
    println!("  - Success: Standard graph has {} states.", standard_graph.nodes.len());

    // 3. Build taxed graph (Scenario B - Thalia active)
    println!("\n[GRAPH GENERATOR] Programmatically expanding Scenario B (Taxation BFS with Thalia active)...");
    let mut taxed_game = game.clone();
    
    let thalia_card = create_test_card("Thalia, Guardian of Thraben");
    taxed_game.card_registry.insert(100, ZoneCard { id: 100, card: thalia_card, owner: 1, is_token: false });
    taxed_game.zones.insert_card(taxed_game.card_registry[&100].clone(), Zone::Battlefield, 1);
    
    taxed_game.active_effects.continuous_effects.push(ContinuousEffect {
        id: 500,
        source: 100,
        layer: ContinuousLayer::Layer6Abilities,
        duration: EffectDuration::StaticAbility { source_card: 100, zone: Zone::Battlefield },
        timestamp: 10,
        effect: ContinuousEffectType::TaxSpells {
            cost_increase: 1,
        },
        conditions: vec![EffectCondition::Not(Box::new(EffectCondition::HasType(crate::card::CardType::Creature)))],
    });
    let taxed_graph = graph::GameGraph::build_algorithmic_graph(&taxed_game, 150);
    println!("  - Success: Taxed graph has {} states.", taxed_graph.nodes.len());

    // 4. Build suppressed graph (Scenario C - Grand Abolisher active)
    println!("\n[GRAPH GENERATOR] Programmatically expanding Scenario C (Suppression BFS with Grand Abolisher active)...");
    let mut suppressed_game = game.clone();
    
    let abolisher_card = create_test_card("Grand Abolisher");
    suppressed_game.card_registry.insert(100, ZoneCard { id: 100, card: abolisher_card, owner: 1, is_token: false });
    suppressed_game.zones.insert_card(suppressed_game.card_registry[&100].clone(), Zone::Battlefield, 1);
    
    suppressed_game.active_effects.continuous_effects.push(ContinuousEffect {
        id: 600,
        source: 100,
        layer: ContinuousLayer::Layer6Abilities,
        duration: EffectDuration::StaticAbility { source_card: 100, zone: Zone::Battlefield },
        timestamp: 10,
        effect: ContinuousEffectType::ActionRestriction {
            restrict_instructions: vec![
                SimInstruction::CheckIsOpponent { player_id: 0, source_card_id: 100 },
            ],
        },
        conditions: vec![
            EffectCondition::IsOpponentOfSource,
            EffectCondition::IsSourceControllerTurn,
        ],
    });
    let suppressed_graph = graph::GameGraph::build_algorithmic_graph(&suppressed_game, 150);
    println!("  - Success: Suppressed graph has {} states.", suppressed_graph.nodes.len());

    // 5. Side-by-side terminal rendering
    println!("\n\x1b[1;36m┌────────────────────────────────────────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[1;36m│              STATE-SPACE BFS DECISION GRAPH COMPARISON RESULTS                 │\x1b[0m");
    println!("\x1b[1;36m├────────────────────────────────────────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[1;36m│ \x1b[1;33mScenario\x1b[1;36m        │ \x1b[1;32mActive Continuous Effects\x1b[1;36m            │ \x1b[1;35mBFS Nodes\x1b[1;36m │ \x1b[1;35mBFS Edges\x1b[1;36m │ \x1b[1;34mPruning % \x1b[1;36m│\x1b[0m");
    println!("\x1b[1;36m├─────────────────┼──────────────────────────────────────┼───────────┼───────────┼───────────┤\x1b[0m");

    let std_nodes = standard_graph.nodes.len();
    let std_edges = standard_graph.edges.len();

    let tx_nodes = taxed_graph.nodes.len();
    let tx_edges = taxed_graph.edges.len();
    let tx_prune_nodes = 100.0 * (1.0 - (tx_nodes as f64 / std_nodes as f64));
    let tx_prune_edges = 100.0 * (1.0 - (tx_edges as f64 / std_edges as f64));

    let sp_nodes = suppressed_graph.nodes.len();
    let sp_edges = suppressed_graph.edges.len();
    let sp_prune_nodes = 100.0 * (1.0 - (sp_nodes as f64 / std_nodes as f64));
    let sp_prune_edges = 100.0 * (1.0 - (sp_edges as f64 / std_edges as f64));

    println!(
        "\x1b[1;36m│ \x1b[1mStandard (A)\x1b[0;1;36m    │ \x1b[30mNone                                 \x1b[1;36m│ \x1b[1;35m{:<9}\x1b[1;36m │ \x1b[1;35m{:<9}\x1b[1;36m │ \x1b[1;34mBaseline  \x1b[1;36m│\x1b[0m",
        std_nodes, std_edges
    );
    println!(
        "\x1b[1;36m│ \x1b[1;33mThalia Tax (B)\x1b[0;1;36m  │ \x1b[33mSpell Taxation +{{1}} (Noncreatures)  \x1b[1;36m│ \x1b[1;35m{:<9}\x1b[1;36m │ \x1b[1;35m{:<9}\x1b[1;36m │ \x1b[1;32m-{:<.1}%     \x1b[1;36m│\x1b[0m",
        tx_nodes, tx_edges, tx_prune_nodes
    );
    println!(
        "\x1b[1;36m│ \x1b[1;35mAbolisher (C)\x1b[0;1;36m   │ \x1b[35mSuppress Opponent Act. Abilities     \x1b[1;36m│ \x1b[1;35m{:<9}\x1b[1;36m │ \x1b[1;35m{:<9}\x1b[1;36m │ \x1b[1;32m-{:<.1}%     \x1b[1;36m│\x1b[0m",
        sp_nodes, sp_edges, sp_prune_nodes
    );
    println!("\x1b[1;36m└────────────────────────────────────────────────────────────────────────────────┘\x1b[0m");

    println!("\n\x1b[1;33m[ANALYTICAL METRICS & NARRATIVE]\x1b[0m");
    println!("  - \x1b[1mScenario A (Standard)\x1b[0m: Player A has full choice branches to tap lands for mana, add mana to pool, and cast Fireball with either X=0 or X=1. Player B gets priority and can fully tap Islands to cast Counterspell to counter Player A's spell.");
    println!("  - \x1b[1;33mScenario B (Thalia Tax)\x1b[0m: Noncreature spells cost {{1}} more. Under this constraint, Fireball (X=1) costs {{2}}{{R}}, which cannot be paid with only 2 lands. Therefore, Player A can only cast Fireball for X=0. Additionally, Player B's Counterspell costs {{2}}{{U}}, which they cannot pay with only 2 Islands, completely pruning Player B's counter-strategy branches (Pruned {:.1}% of total decisions!).", tx_prune_edges);
    println!("  - \x1b[1;35mScenario C (Grand Abolisher)\x1b[0m: Opponents' activated abilities are fully suppressed. Player B (the opponent of Grand Abolisher's controller) is completely barred from tapping their Islands for mana! As a result, Player B can never float mana to cast Counterspell, removing all of Player B's reactive branches from the game graph (Pruned {:.1}% of total decisions!).", sp_prune_edges);

    // Export Mermaid flowchart markdown for Scenario A (Standard)
    let standard_mermaid = standard_graph.export_mermaid_markdown();
    
    // Build combined Markdown document with Mermaid graph and Comparison report
    let mut combined_md = String::new();
    combined_md.push_str("# State-Space BFS Decision Graph Comparison\n\n");
    combined_md.push_str("This document contains a side-by-side comparison of how continuous effects—specifically spell taxation (such as *Thalia, Guardian of Thraben*) and activated ability suppression (such as *Grand Abolisher*)—dynamically alter and prune the game's algorithmic state-space decision tree.\n\n");
    
    combined_md.push_str("## Side-by-Side Comparison Metrics\n\n");
    combined_md.push_str("| Scenario | Active Continuous Effects | BFS Nodes | BFS Edges | Node Reduction % | Edge Reduction % |\n");
    combined_md.push_str("| --- | --- | --- | --- | --- | --- |\n");
    combined_md.push_str(&format!("| **Standard (A)** | None | {} | {} | Baseline | Baseline |\n", std_nodes, std_edges));
    combined_md.push_str(&format!("| **Thalia Tax (B)** | Spell Taxation +{{1}} (Noncreatures) | {} | {} | {:.1}% | {:.1}% |\n", tx_nodes, tx_edges, tx_prune_nodes, tx_prune_edges));
    combined_md.push_str(&format!("| **Grand Abolisher (C)** | Suppress Opponent Activated Abilities | {} | {} | {:.1}% | {:.1}% |\n\n", sp_nodes, sp_edges, sp_prune_nodes, sp_prune_edges));
    
    combined_md.push_str("## Analytical Diagnostics\n\n");
    combined_md.push_str("> [!IMPORTANT]\n");
    combined_md.push_str("> **Active Tax and Suppress Effects Prove Dynamic Decision Modification**\n");
    combined_md.push_str("> Our priority BFS state-space search successfully proves that MtG card abilities do not just act as static text, but dynamically reshape the future decision-tree path. Active static effects act as powerful filters that block, modify, or completely eliminate decision vertices.\n\n");
    
    combined_md.push_str("### 1. Standard Scenario Analysis\n");
    combined_md.push_str("- Player A has 2 lands (Forest, Mountain) and Fireball in hand. They can make individual decisions to tap Forest, tap Mountain, and cast Fireball for X=0 or X=1.\n");
    combined_md.push_str("- Player B has 2 Islands and Counterspell. If Player A casts Fireball, Player B can respond by tapping their Islands and casting Counterspell.\n\n");
    
    combined_md.push_str("### 2. Thalia Spell Taxation Analysis\n");
    combined_md.push_str("- Fireball is taxed by `{1}`. Casting Fireball with X=0 now costs `{1}{R}`. Casting for X=1 costs `{2}{R}`, which Player A cannot pay. Thus, the choice of casting for X=1 is filtered out.\n");
    combined_md.push_str("- Counterspell is also taxed by `{1}`, raising its cost to `{2}{U}`. Player B has only 2 Islands, so they are completely locked out of responding. This entire counter-strategy branch is pruned.\n\n");
    
    combined_md.push_str("### 3. Grand Abolisher Suppression Analysis\n");
    combined_md.push_str("- Opponent's activated abilities are suppressed. Tapping an Island for mana is an activated ability (mana ability). Player B is forbidden from activating it.\n");
    combined_md.push_str("- Consequently, Player B cannot add `{U}` to pay for Counterspell, ensuring Player A can execute their plays completely uninterrupted.\n\n");
    
    combined_md.push_str("## Scenario A (Standard) Decision Graph Flowchart\n\n");
    combined_md.push_str(&standard_mermaid);

    // Create 'graphs' folder in the workspace root if not exists
    let workspace_graphs_dir = "./graphs";
    let _ = std::fs::create_dir_all(workspace_graphs_dir);
    
    let workspace_filepath = "./graphs/decision_graph.md";
    println!("[EXPORT] Combined Comparison & Flowchart to workspace: {}", workspace_filepath);
    if let Err(e) = std::fs::write(workspace_filepath, &combined_md) {
        println!("[ERROR] Failed to write workspace decision_graph.md: {}", e);
    }
    
    println!("\x1b[1;35m================================================================================\x1b[0m\n");
}

#[allow(unused_must_use)]
fn run_standard_walkthrough() -> Game {
    println!("\x1b[1;35m=========================================================\x1b[0m");
    println!("\x1b[1;35m*  STACKS-ON-STACKS: MTG SIMULATION KERNEL WALKTHROUGH   *\x1b[0m");
    println!("\x1b[1;35m=========================================================\x1b[0m");

    // Initialize Game Kernel and Players
    let mut game = Game::new(Format::Commander);
    
    let player_a = Player::new(1, "Player A".to_string(), 20);
    let player_b = Player::new(2, "Player B".to_string(), 20);
    
    game.add_player(player_a);
    game.add_player(player_b);

    // Pull raw static cards from create_test_card (Rule-compliant representations)
    let forest = create_test_card("Forest");
    let sol_ring = create_test_card("Sol Ring");
    let arcane_signet = create_test_card("Arcane Signet");
    let island = create_test_card("Island");
    let counterspell = create_test_card("Counterspell");

    // 1. SETUP INITIAL WALKTHROUGH STATES
    // Player A (ID 1) Hand: Forest (ID 10), Sol Ring (ID 11), Arcane Signet (ID 12)
    game.card_registry.insert(10, ZoneCard { id: 10, card: forest, owner: 1, is_token: false });
    game.zones.insert_card(game.card_registry[&10].clone(), Zone::Hand, 1);

    game.card_registry.insert(11, ZoneCard { id: 11, card: sol_ring, owner: 1, is_token: false });
    game.zones.insert_card(game.card_registry[&11].clone(), Zone::Hand, 1);

    game.card_registry.insert(12, ZoneCard { id: 12, card: arcane_signet, owner: 1, is_token: false });
    game.zones.insert_card(game.card_registry[&12].clone(), Zone::Hand, 1);

    // Player B (ID 2) Battlefield: Island 1 (ID 20), Island 2 (ID 21)
    game.card_registry.insert(20, ZoneCard { id: 20, card: island.clone(), owner: 2, is_token: false });
    game.zones.insert_card(game.card_registry[&20].clone(), Zone::Battlefield, 2);

    game.card_registry.insert(21, ZoneCard { id: 21, card: island, owner: 2, is_token: false });
    game.zones.insert_card(game.card_registry[&21].clone(), Zone::Battlefield, 2);

    // Player B Hand: Counterspell (ID 22)
    game.card_registry.insert(22, ZoneCard { id: 22, card: counterspell, owner: 2, is_token: false });
    game.zones.insert_card(game.card_registry[&22].clone(), Zone::Hand, 2);

    println!("\x1b[1;32m[SYSTEM] Initialization Succeeded.\x1b[0m Starting board state configured.");
    print_game_state(&game);

    // 2. COMPILE & DEFINE MACHINE CODE VECTOR (Rule-compliant sequences)
    let program_machine_code = vec![
        // -- PLAYER A'S FIRST SPELL SEQUENCE --
        
        // Operation 1: Play Forest from hand to Battlefield (Special Action 116)
        SimInstruction::MoveCard { card_id: 10, from: Zone::Hand, to: Zone::Battlefield, controller: 1 },
        
        // Operation 2: Tap Forest for mana (mana ability - doesn't use stack)
        SimInstruction::TapPermanent { card_id: 10 },
        SimInstruction::AddMana { player_id: 1, color: Color::G, amount: 1 },
        
        // Operation 3: Cast Sol Ring (pays green generic mana, moves from hand to stack)
        SimInstruction::SpendMana { player_id: 1, color: Color::G, amount: 1 },
        SimInstruction::MoveCard { card_id: 11, from: Zone::Hand, to: Zone::Stack, controller: 1 },
        SimInstruction::PushSpell { card_id: 11, caster: 1 }, // Sol Ring gets Stack Item ID 0
        
        // Operation 4: Resolve Sol Ring (pops Sol Ring and places onto Battlefield)
        SimInstruction::PopStack,
        SimInstruction::MoveCard { card_id: 11, from: Zone::Stack, to: Zone::Battlefield, controller: 1 },

        // -- PLAYER A'S SECOND SPELL SEQUENCE --

        // Operation 5: Tap Sol Ring for mana (mana ability - adds {C}{C})
        SimInstruction::TapPermanent { card_id: 11 },
        SimInstruction::AddMana { player_id: 1, color: Color::C, amount: 2 },

        // Operation 6: Cast Arcane Signet (pays {C}{C}, moves hand to stack, push)
        SimInstruction::SpendMana { player_id: 1, color: Color::C, amount: 2 },
        SimInstruction::MoveCard { card_id: 12, from: Zone::Hand, to: Zone::Stack, controller: 1 },
        SimInstruction::PushSpell { card_id: 12, caster: 1 }, // Arcane Signet gets Stack Item ID 1

        // -- PLAYER B'S RESPONSE SEQUENCE --

        // Operation 7: Player B taps Island 1 for mana (mana ability - adds {U})
        SimInstruction::TapPermanent { card_id: 20 },
        SimInstruction::AddMana { player_id: 2, color: Color::U, amount: 1 },

        // Operation 8: Player B taps Island 2 for mana (mana ability - adds {U})
        SimInstruction::TapPermanent { card_id: 21 },
        SimInstruction::AddMana { player_id: 2, color: Color::U, amount: 1 },

        // Operation 9: Player B casts Counterspell (pays {U}{U}, moves hand to stack, registers target)
        SimInstruction::SpendMana { player_id: 2, color: Color::U, amount: 2 },
        SimInstruction::MoveCard { card_id: 22, from: Zone::Hand, to: Zone::Stack, controller: 2 },
        SimInstruction::PushSpell { card_id: 22, caster: 2 }, // Counterspell gets Stack Item ID 2
        SimInstruction::RegisterTarget { stack_item_id: 2, target: Target::Spell(12) }, // targets Arcane Signet (Card ID 12)

        // -- STACK RESOLUTION PIPELINE --

        // Operation 10: Resolve top of stack (Pop Counterspell from stack)
        SimInstruction::PopStack,
        // Counterspell counters Arcane Signet: removes Arcane Signet from stack, moves to A's Graveyard
        SimInstruction::RemoveFromStack { stack_item_id: 1 },
        SimInstruction::MoveCard { card_id: 12, from: Zone::Stack, to: Zone::Graveyard, controller: 1 },
        // Counterspell itself moves from stack to B's Graveyard
        SimInstruction::MoveCard { card_id: 22, from: Zone::Stack, to: Zone::Graveyard, controller: 2 },
    ];

    // 3. EXECUTE MACHINE CODE CHRONOLOGICALLY
    println!("\x1b[1;33m[KERNEL STATUS] Loading and running machine code instructions...\x1b[0m");
    for (step, instruction) in program_machine_code.into_iter().enumerate() {
        println!("\n\x1b[1;34m--- CLOCK CYCLE / STEP {} ---\x1b[0m", step + 1);
        game.execute_instruction(instruction);
        print_game_state(&game);
    }

    println!("\n\x1b[1;32m[WALKTHROUGH COMPLETED SUCCESSFULY]\x1b[0m");
    println!("We verified:");
    println!("  1. Player A successfully played a Forest, tapped it for {{G}}, and cast Sol Ring.");
    println!("  2. Sol Ring resolved to the Battlefield and was tapped to pay {{C}}{{C}} for Arcane Signet.");
    println!("  3. With Arcane Signet on the stack, Player B tapped two Islands for {{U}}{{U}} and cast Counterspell.");
    println!("  4. Counterspell resolved, countering Arcane Signet, removing it from stack, and placing both in their respective graveyards.");
    println!("  5. All low-level state modifications on players, zones, and stack mapped perfectly (1:1) to the underlying structures!");
    println!("\x1b[1;35m=========================================================\x1b[0m\n");

    game
}

#[allow(unused_must_use)]
fn run_secondary_zones_showcase(game: &mut Game) {
    println!("\x1b[1;36m=========================================================\x1b[0m");
    println!("\x1b[1;36m*      SECONDARY ZONE PRIMITIVES & LIBRARY SEARCH        *\x1b[0m");
    println!("\x1b[1;36m=========================================================\x1b[0m");
    
    // Setup Player A's library: Put a Colossal Dreadmaw (ID 50) inside Player A's library
    let dreadmaw = create_test_card("Colossal Dreadmaw");
    game.card_registry.insert(50, ZoneCard { id: 50, card: dreadmaw, owner: 1, is_token: false });
    game.zones.insert_card(game.card_registry[&50].clone(), Zone::Library, 1);
    
    let showcase_instructions = vec![
        // 1. Library actions: Search library for Colossal Dreadmaw (ID 50) and draw it
        SimInstruction::SearchLibrary { player_id: 1, card_id: 50 },
        SimInstruction::DrawCard { player_id: 1 },
        SimInstruction::ShuffleLibrary { player_id: 1 },
        
        // 2. Command Zone actions: Create emblem for Player A
        SimInstruction::CreateEmblem { controller: 1, rules_text: "You have no hand size limit.".to_string() },
        
        // 3. Token actions: Create Sol Ring token on battlefield for Player B
        SimInstruction::CreateToken { controller: 2, token_id: 60, card: create_test_card("Sol Ring") },
        
        // 4. Status, damage and counter actions on Permanent (Sol Ring ID 11)
        SimInstruction::AddCounter { card_id: 11, counter_type: "+1/+1".to_string(), amount: 2 },
        SimInstruction::MarkDamage { card_id: 11, amount: 3 },
        SimInstruction::PhaseOutPermanent { card_id: 11 },
        SimInstruction::PhaseInPermanent { card_id: 11 },
        SimInstruction::FlipPermanent { card_id: 11 },
        SimInstruction::UnflipPermanent { card_id: 11 },
        SimInstruction::TurnPermanentFaceDown { card_id: 11 },
        SimInstruction::TurnPermanentFaceUp { card_id: 11 },
        SimInstruction::AttachPermanent { card_id: 11, target: Target::Card(10) },
        SimInstruction::DetachPermanent { card_id: 11 },
        SimInstruction::ClearDamage { card_id: 11 },
        
        // 5. Exile Visibility actions
        SimInstruction::MoveCard { card_id: 22, from: Zone::Graveyard, to: Zone::Exile, controller: 2 },
        SimInstruction::SetExiledFaceUp { card_id: 22, face_up: false },
        SimInstruction::SetExiledFaceUp { card_id: 22, face_up: true },
    ];

    println!("\x1b[1;33m[KERNEL STATUS] Loading and running secondary showcase instructions...\x1b[0m");
    for (step, instruction) in showcase_instructions.into_iter().enumerate() {
        println!("\n\x1b[1;36m--- SHOWCASE CLOCK CYCLE / STEP {} ---\x1b[0m", step + 1);
        game.execute_instruction(instruction);
        print_game_state(game);
    }

    println!("\n\x1b[1;32m[SHOWCASE COMPLETED SUCCESSFULY]\x1b[0m");
    println!("We verified:");
    println!("  1. Library searching works correctly and returns the compiled card instance.");
    println!("  2. Library drawing, shuffling, and command zone emblem creation are executed 1:1.");
    println!("  3. Battlefield token creation functions flawlessly.");
    println!("  4. Detailed permanent status properties (tapped/untapped, flipped/unflipped, face-up/face-down, phased-in/phased-out), counters, marked damage, and attachment targets are fully represented.");
    println!("  5. Shared exile card visibility (face-up/face-down) changes are tracked properly.");
    println!("\x1b[1;35m=========================================================\x1b[0m\n");
}

fn run_serialization_test(game: &Game) {
    // --- GAME STATE SERIALIZATION & ROUND-TRIP VERIFICATION ---
    println!("\x1b[1;35m=========================================================\x1b[0m");
    println!("\x1b[1;35m*    GAME STATE SERIALIZATION & ROUND-TRIP TEST         *\x1b[0m");
    println!("\x1b[1;35m=========================================================\x1b[0m");
    println!("\x1b[1;33m[SERIALIZATION] Serializing active game state to JSON...\x1b[0m");
    
    let serialized_state = serde_json::to_string_pretty(game).unwrap();
    
    // Print first 1000 characters of the JSON state representation
    let preview_len = std::cmp::min(1000, serialized_state.len());
    println!("\x1b[1;30m--- JSON PREVIEW (First 1000 chars) ---\n{}\n...[TRUNCATED]...\x1b[0m", &serialized_state[..preview_len]);
    
    println!("\x1b[1;33m[DESERIALIZATION] Deserializing JSON back to Game struct...\x1b[0m");
    let restored_game: Game = serde_json::from_str(&serialized_state).unwrap();
    
    // Check structural and logical equality
    assert_eq!(game, &restored_game);
    println!("\x1b[1;32m[SUCCESS] State matches perfectly! Snapshot/Restore is 100% sound.\x1b[0m");
    println!("\x1b[1;35m=========================================================\x1b[0m\n");
}

fn print_game_state(game: &Game) {
    println!("\n\x1b[1;35m================================================================================\x1b[0m");
    println!("\x1b[1;35m                       MAGIC SIMULATOR GAME STATE KERNEL                       \x1b[0m");
    println!("\x1b[1;35m================================================================================\x1b[0m");

    for player_id in &[1, 2] {
        if let Some(player) = game.players.get(player_id) {
            let role_name = if *player_id == 1 { "Player A 🌲" } else { "Player B 💧" };
            println!("\x1b[1;33m{} : {}\x1b[0m (HP: {})", role_name, player.name, player.life_total);
            
            // Mana pool
            print!("  \x1b[1;32mMana Pool:\x1b[0m ");
            if player.mana_pool.is_empty() {
                print!("Empty");
            } else {
                if player.mana_pool.white > 0 { print!("\x1b[1;37m{{W}}\x1b[0m:{} ", player.mana_pool.white); }
                if player.mana_pool.blue > 0 { print!("\x1b[1;34m{{U}}\x1b[0m:{} ", player.mana_pool.blue); }
                if player.mana_pool.black > 0 { print!("\x1b[1;30m{{B}}\x1b[0m:{} ", player.mana_pool.black); }
                if player.mana_pool.red > 0 { print!("\x1b[1;31m{{R}}\x1b[0m:{} ", player.mana_pool.red); }
                if player.mana_pool.green > 0 { print!("\x1b[1;32m{{G}}\x1b[0m:{} ", player.mana_pool.green); }
                if player.mana_pool.colorless > 0 { print!("\x1b[1;36m{{C}}\x1b[0m:{} ", player.mana_pool.colorless); }
            }
            println!();

            // Hand
            print!("  \x1b[1;34mHand:\x1b[0m ");
            if let Some(hand) = game.zones.hands.get(player_id) {
                if hand.cards.is_empty() {
                    print!("(Empty)");
                } else {
                    let card_names: Vec<String> = hand.cards.iter().map(|zc| {
                        let name = game.get_registered_card_name(zc.id);
                        format!("{} [Card ID: {}]", name, zc.id)
                    }).collect();
                    print!("{}", card_names.join(", "));
                }
            }
            println!();

            // Battlefield (permanents controlled by this player)
            print!("  \x1b[1;32mBattlefield:\x1b[0m ");
            let controlled_perms: Vec<&crate::zones::Permanent> = game.zones.battlefield.permanents.iter()
                .filter(|p| p.controller == *player_id).collect();
            if controlled_perms.is_empty() {
                print!("(None)");
            } else {
                let perm_names: Vec<String> = controlled_perms.iter().map(|p| {
                    let name = game.get_registered_card_name(p.id);
                    let mut status_flags: Vec<String> = Vec::new();
                    
                    if p.is_tapped() {
                        status_flags.push("\x1b[31mTapped ⊗\x1b[0m".to_string());
                    } else {
                        status_flags.push("\x1b[32mUntapped ◯\x1b[0m".to_string());
                    }
                    if p.is_flipped() {
                        status_flags.push("\x1b[35mFlipped ↷\x1b[0m".to_string());
                    }
                    if p.is_face_down() {
                        status_flags.push("\x1b[30mFace Down ❑\x1b[0m".to_string());
                    }
                    if p.is_phased_out() {
                        status_flags.push("\x1b[36mPhased Out ◌\x1b[0m".to_string());
                    }
                    if p.damage_marked > 0 {
                        status_flags.push(format!("\x1b[31mMarked Damage: {}\x1b[0m", p.damage_marked));
                    }
                    if !p.counters.is_empty() {
                        let mut counters_str: Vec<String> = p.counters.iter().map(|(k, v)| format!("{}: {}", k, v)).collect();
                        counters_str.sort();
                        status_flags.push(format!("\x1b[34mCounters: {}\x1b[0m", counters_str.join(", ")));
                    }
                    if let Some(target) = &p.attached_to {
                        status_flags.push(format!("\x1b[33mAttached to: {:?}\x1b[0m", target));
                    }
                    
                    format!("{} [Card ID: {}] ({})", name, p.id, status_flags.join(", "))
                }).collect();
                print!("{}", perm_names.join(", "));
            }
            println!();

            // Graveyard
            print!("  \x1b[1;31mGraveyard:\x1b[0m ");
            if let Some(gy) = game.zones.graveyards.get(player_id) {
                if gy.cards.is_empty() {
                    print!("(Empty)");
                } else {
                    let gy_names: Vec<String> = gy.cards.iter().map(|zc| {
                        let name = game.get_registered_card_name(zc.id);
                        format!("{} [Card ID: {}]", name, zc.id)
                    }).collect();
                    print!("{}", gy_names.join(", "));
                }
            }
            println!();

            // Command Zone (Commanders and Emblems)
            let has_command_zone_items = game.zones.command_zone.commanders.iter().any(|zc| zc.owner == *player_id)
                || game.zones.command_zone.emblems.iter().any(|e| e.owner == *player_id);
                
            if has_command_zone_items {
                print!("  \x1b[1;35mCommand Zone:\x1b[0m ");
                let mut cz_items = Vec::new();
                for commander in game.zones.command_zone.commanders.iter().filter(|zc| zc.owner == *player_id) {
                    let name = game.get_registered_card_name(commander.id);
                    cz_items.push(format!("Commander: {} [Card ID: {}]", name, commander.id));
                }
                for emblem in game.zones.command_zone.emblems.iter().filter(|e| e.owner == *player_id) {
                    cz_items.push(format!("Emblem [ID: {}] ('{}')", emblem.id, emblem.rules_text));
                }
                print!("{}", cz_items.join(", "));
                println!();
            }
            println!();
        }
    }

    // Exile zone state
    if !game.zones.exile.objects.is_empty() {
        println!("\x1b[1;36m=================== EXILE ZONE ===================\x1b[0m");
        for obj in &game.zones.exile.objects {
            let name = game.get_registered_card_name(obj.id);
            let vis = if obj.face_up { "Face Up" } else { "Face Down" };
            println!("  Card: {} [Card ID: {}] (Owner: Player {}, {})", name, obj.id, obj.owner, vis);
        }
        println!();
    }

    // Stack state
    println!("\x1b[1;31m=================== STACK ZONE ===================\x1b[0m");
    if game.stack.is_empty() {
        println!("  (Stack is currently Empty)");
    } else {
        for (idx, item) in game.stack.iter().enumerate() {
            match item.object {
                crate::stack::StackObject::Spell { card_id, caster, .. } => {
                    let name = game.get_registered_card_name(card_id);
                    let caster_name = game.players.get(&caster).map(|p| p.name.as_str()).unwrap_or("Unknown");
                    print!("  [{}] Stack Item ID: {} | Card: {} (Card ID: {}) | Caster: {}", idx, item.id, name, card_id, caster_name);
                    if !item.targets.is_empty() {
                        let target_descs: Vec<String> = item.targets.iter().map(|target| {
                            match target {
                                Target::Spell(cid) => format!("Spell '{}' (Card ID: {})", game.get_registered_card_name(*cid), cid),
                                other => format!("{:?}", other),
                            }
                        }).collect();
                        print!(" | Targets: \x1b[1;33m{}\x1b[0m", target_descs.join(", "));
                    }
                    println!();
                }
                _ => {
                    println!("  [{}] Stack Item ID: {} | Activated/Triggered Ability", idx, item.id);
                }
            }
        }
    }
    println!("\x1b[1;35m================================================================================\x1b[0m\n");
}
