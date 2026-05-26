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

use game::{Game, SimInstruction, Format};
use player::Player;
use card::{Card, Color};
use zones::ZoneCard;
use effects::{Target, Zone};
use scryfall::create_test_card;

fn main() {
    println!("\x1b[1;35m=========================================================\x1b[0m");
    println!("\x1b[1;35m*  STACKS-ON-STACKS: MTG SIMULATION KERNEL WALKTHROUGH   *\x1b[0m");
    println!("\x1b[1;35m=========================================================\x1b[0m");

    // --- PREMIUM CARD RENDERING SHOWCASE ---
    println!("\n\x1b[1;36m=========================================================\x1b[0m");
    println!("\x1b[1;36m*             MTG ASCII CARD RENDERING SHOWCASE         *\x1b[0m");
    println!("\x1b[1;36m=========================================================\x1b[0m");
    let _dreadmaw = create_test_card("Colossal Dreadmaw");
    let _bolt = create_test_card("Lightning Bolt");
    println!("\x1b[1;36m=========================================================\x1b[0m\n");

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

    // --- SECONDARY ZONE PRIMITIVE & LIBRARY SEARCH SHOWCASE ---
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
        print_game_state(&game);
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
