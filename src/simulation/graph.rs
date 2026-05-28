use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::effects::PlayerId;
use crate::game::Game;
use crate::actions::PriorityAction;

/// Represents a node in our priority-state decision graph.
/// Each node is a distinct game state after state-based actions are evaluated
/// and specifies which player has priority (if any).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GameNode {
    pub id: String,
    pub description: String,
    pub state: Game,
    pub priority_player: Option<PlayerId>,
    pub possible_actions: Vec<PriorityAction>,
}

/// Represents an edge (a choice/transition) from one GameNode to another.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GameEdge {
    pub from_node: String,
    pub to_node: String,
    pub label: String,
    pub action: PriorityAction,
}

/// Represents the entire pre-initialized game state decision graph.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GameGraph {
    pub nodes: HashMap<String, GameNode>,
    pub edges: Vec<GameEdge>,
}

impl GameGraph {
    /// Creates a new, empty decision graph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Algorithmically builds the game priority-decision tree graph (using BFS expansion).
    pub fn build_algorithmic_graph(initial_state: &Game, max_nodes: usize) -> Self {
        use std::collections::VecDeque;

        let mut graph = Self::new();
        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        let mut node_id_counter = 1;
        let root_id = "Node_1_Start".to_string();

        let mut root_state = initial_state.clone();
        // Clear any previous passes or priority state to start fresh
        root_state.consecutive_passes = 0;
        root_state.priority_player = initial_state.priority_player.or_else(|| initial_state.turn_order.first().cloned());

        let root_serialized = serde_json::to_string(&root_state).unwrap();
        visited.insert(root_serialized, root_id.clone());

        let root_priority = root_state.priority_player;
        let root_actions = root_priority.map(|p| root_state.get_possible_actions(p)).unwrap_or_default();

        let initial_priority_id = root_priority.unwrap_or(root_state.active_player);
        let priority_player_name = root_state.players.get(&initial_priority_id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| format!("Player {}", initial_priority_id));
        let root_desc = format!("Initial State: {} has priority.", priority_player_name);

        graph.nodes.insert(
            root_id.clone(),
            GameNode {
                id: root_id.clone(),
                description: root_desc,
                state: root_state.clone(),
                priority_player: root_priority,
                possible_actions: root_actions,
            },
        );

        queue.push_back(root_id);

        while let Some(current_node_id) = queue.pop_front() {
            if graph.nodes.len() >= max_nodes {
                break;
            }

            let current_node = graph.nodes.get(&current_node_id).unwrap().clone();
            let _priority_player = match current_node.priority_player {
                Some(p) => p,
                None => continue, // Terminal node
            };

            for action in current_node.possible_actions {
                let mut next_state = current_node.state.clone();
                let logs = next_state.execute_action(action.clone());

                // Build a nice label for the edge
                let mut label = match &action {
                    PriorityAction::PassPriority { player } => {
                        let player_name = current_node.state.players.get(player).map(|p| p.name.as_str()).unwrap_or("Unknown");
                        format!("{} Passes", player_name)
                    }
                    PriorityAction::ActivateAbility { player, source_id, .. } => {
                        let land_name = current_node.state.get_registered_card_name(*source_id);
                        let player_name = current_node.state.players.get(player).map(|p| p.name.as_str()).unwrap_or("Unknown");
                        format!("{} Taps {}", player_name, land_name)
                    }
                    PriorityAction::CastSpell { player, card_id, .. } => {
                        let spell_name = current_node.state.get_registered_card_name(*card_id);
                        let player_name = current_node.state.players.get(player).map(|p| p.name.as_str()).unwrap_or("Unknown");
                        format!("{} Casts {}", player_name, spell_name)
                    }
                    _ => "Special Action".to_string(),
                };

                // Append any resolution or SBA logs to the edge label to make the transition narrative descriptive
                let key_events: Vec<String> = logs.iter()
                    .filter(|l| l.contains("resolves") || l.contains("countered") || l.contains("[SBA TRIGGERED]"))
                    .cloned()
                    .collect();
                if !key_events.is_empty() {
                    label.push_str(&format!(" ({})", key_events.join(", ").replace("[SBA TRIGGERED] ", "")));
                }

                let next_serialized = serde_json::to_string(&next_state).unwrap();
                let to_node_id = if let Some(existing_id) = visited.get(&next_serialized) {
                    existing_id.clone()
                } else {
                    node_id_counter += 1;
                    let new_id = format!("Node_{}", node_id_counter);
                    visited.insert(next_serialized, new_id.clone());

                    // Generate a descriptive, beautifully detailed state description
                    let mut desc = String::new();
                    
                    // 1. Describe Stack
                    if next_state.stack.items.is_empty() {
                        desc.push_str("Stack is empty.");
                    } else {
                        let stack_items: Vec<String> = next_state.stack.items.iter()
                            .map(|item| {
                                if let crate::stack::StackObject::Spell { card_id, caster, .. } = item.object {
                                    let caster_name = next_state.players.get(&caster).map(|p| p.name.as_str()).unwrap_or("Unknown");
                                    format!("{} (cast by {})", next_state.get_registered_card_name(card_id), caster_name)
                                } else {
                                    "Ability".to_string()
                                }
                            })
                            .collect();
                        desc.push_str(&format!("Stack: [{}].", stack_items.join(" -> ")));
                    }

                    // 2. Describe Mana Pools
                    let mut mana_descs = Vec::new();
                    let mut player_ids: Vec<PlayerId> = next_state.players.keys().cloned().collect();
                    player_ids.sort();
                    for pid in &player_ids {
                        if let Some(player) = next_state.players.get(pid) {
                            let mut colors_str = Vec::new();
                            if player.mana_pool.white > 0 { colors_str.push(format!("W: {}", player.mana_pool.white)); }
                            if player.mana_pool.blue > 0 { colors_str.push(format!("U: {}", player.mana_pool.blue)); }
                            if player.mana_pool.black > 0 { colors_str.push(format!("B: {}", player.mana_pool.black)); }
                            if player.mana_pool.red > 0 { colors_str.push(format!("R: {}", player.mana_pool.red)); }
                            if player.mana_pool.green > 0 { colors_str.push(format!("G: {}", player.mana_pool.green)); }
                            if player.mana_pool.colorless > 0 { colors_str.push(format!("C: {}", player.mana_pool.colorless)); }
                            
                            let pool_content = if colors_str.is_empty() {
                                "empty".to_string()
                            } else {
                                format!("{{{}}}", colors_str.join(", "))
                            };
                            mana_descs.push(format!("{}: {}", player.name, pool_content));
                        }
                    }
                    desc.push_str(&format!(" Mana pools: {}.", mana_descs.join("; ")));

                    // 3. Describe Battlefield Creatures & status
                    let mut creatures = Vec::new();
                    for perm in &next_state.zones.battlefield.permanents {
                        let name = crate::game::get_card_name(&perm.card);
                        if let crate::card::Card::Creature(_) = perm.card {
                            creatures.push(format!("{} (ID: {}, damage: {})", name, perm.id, perm.damage_marked));
                        }
                    }
                    if !creatures.is_empty() {
                        desc.push_str(&format!(" Battlefield: {}.", creatures.join(", ")));
                    } else {
                        desc.push_str(" Battlefield: No creatures.");
                    }

                    // 4. Describe Hand counts
                    let mut hand_descs = Vec::new();
                    for pid in &player_ids {
                        if let Some(player) = next_state.players.get(pid) {
                            let hand_count = next_state.zones.hands.get(pid).map(|h| h.cards.len()).unwrap_or(0);
                            hand_descs.push(format!("{}: {}", player.name, hand_count));
                        }
                    }
                    desc.push_str(&format!(" Hands: {}.", hand_descs.join(", ")));

                    let next_priority = next_state.priority_player;
                    let next_actions = next_priority.map(|p| next_state.get_possible_actions(p)).unwrap_or_default();

                    graph.nodes.insert(
                        new_id.clone(),
                        GameNode {
                            id: new_id.clone(),
                            description: desc,
                            state: next_state.clone(),
                            priority_player: next_priority,
                            possible_actions: next_actions,
                        },
                    );

                    queue.push_back(new_id.clone());
                    new_id
                };

                graph.edges.push(GameEdge {
                    from_node: current_node_id.clone(),
                    to_node: to_node_id,
                    label,
                    action,
                });
            }
        }

        graph
    }



    /// Renders a beautifully styled terminal ASCII tree flowchart of the priority states.
    pub fn print_ascii_visualization(&self) {
        println!("\n\x1b[1;35m=========================================================\x1b[0m");
        println!("\x1b[1;35m*           MTG STATE-DECISION TREE VISUALIZATION       *\x1b[0m");
        println!("\x1b[1;35m=========================================================\x1b[0m");

        // We can render a retro hierarchical list of nodes and transitions
        for (node_id, node) in &self.nodes {
            println!("\n\x1b[1;34m[Node: {}]\x1b[0m", node_id);
            println!("  \x1b[1;32mState:\x1b[0m {}", node.description);
            if let Some(player) = node.priority_player {
                println!("  \x1b[1;33mPriority Player:\x1b[0m Player {}", player);
            } else {
                println!("  \x1b[1;33mPriority Player:\x1b[0m None (Phase Transition)");
            }
            println!("  \x1b[1;36mPossible Choices:\x1b[0m");
            for action in &node.possible_actions {
                println!("    - {:?}", action);
            }

            // Find outgoing edges
            let outgoing: Vec<&GameEdge> = self.edges.iter().filter(|e| e.from_node == *node_id).collect();
            if !outgoing.is_empty() {
                println!("  \x1b[1;35mTransitions / Branches:\x1b[0m");
                for edge in outgoing {
                    println!("    └── \x1b[33m\"{}\"\x1b[0m ===> \x1b[36m[{}]\x1b[0m", edge.label, edge.to_node);
                }
            }
        }
        println!("\n\x1b[1;35m=========================================================\x1b[0m\n");
    }

    /// Exporters a beautiful markdown file with a styled, interactive Mermaid.js diagram.
    pub fn export_mermaid_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# MTG Decision Graph & Priority Flowchart\n\n");
        md.push_str("This document contains the auto-generated **Mermaid.js flowchart** of our priority-state decision graph.\n\n");
        md.push_str("```mermaid\nflowchart TD\n");

        // Styling and themes
        md.push_str("    %% Node styles and definitions\n");
        md.push_str("    classDef apColor fill:#2ecc71,stroke:#27ae60,stroke-width:2px,color:#fff;\n");
        md.push_str("    classDef napColor fill:#3498db,stroke:#2980b9,stroke-width:2px,color:#fff;\n");
        md.push_str("    classDef systemColor fill:#9b59b6,stroke:#8e44ad,stroke-width:2px,color:#fff;\n\n");

        // Define nodes
        for (node_id, node) in &self.nodes {
            let label = format!("\"**{}**<br/>{}\"", node_id.replace("_", " "), node.description);
            if let Some(player) = node.priority_player {
                if player == node.state.active_player {
                    md.push_str(&format!("    {}({})\n", node_id, label));
                    md.push_str(&format!("    class {} apColor;\n", node_id));
                } else {
                    md.push_str(&format!("    {}({})\n", node_id, label));
                    md.push_str(&format!("    class {} napColor;\n", node_id));
                }
            } else {
                md.push_str(&format!("    {}[{}]\n", node_id, label));
                md.push_str(&format!("    class {} systemColor;\n", node_id));
            }
        }

        md.push_str("\n    %% Transitions / Edges\n");
        for edge in &self.edges {
            md.push_str(&format!("    {} -->|\"{}\"| {}\n", edge.from_node, edge.label, edge.to_node));
        }

        md.push_str("```\n");
        md
    }
}
