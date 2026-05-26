use crate::effects::{CardId, PlayerId, Zone};

/// Represents any action a player can take when they have priority (Rule 117.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PriorityAction {
    /// Cast a spell (Rule 117.1a / 601.2).
    CastSpell {
        player: PlayerId,
        card_id: CardId,
        is_instant_speed: bool,
    },
    /// Activate an activated ability (Rule 117.1b / 602.2).
    ActivateAbility {
        player: PlayerId,
        ability_id: u32,
        source_id: CardId,
        is_instant_speed: bool,
        is_mana_ability: bool,
    },
    /// Take a special action (Rule 117.1c / 116).
    TakeSpecialAction {
        player: PlayerId,
        action: SpecialAction,
    },
    /// Pass priority to the next player (Rule 117.3d / 117.4).
    PassPriority {
        player: PlayerId,
    },
}

impl PriorityAction {
    /// Returns true if this action uses the stack (Rule 117.1 / 116.1 / 605.1a).
    pub fn uses_stack(&self) -> bool {
        match self {
            PriorityAction::CastSpell { .. } => true,
            PriorityAction::ActivateAbility { is_mana_ability, .. } => !is_mana_ability,
            PriorityAction::TakeSpecialAction { action, .. } => action.uses_stack(),
            PriorityAction::PassPriority { .. } => false,
        }
    }

    /// Returns true if this action requires the stack to be empty (Rule 307.1 / 116.2a).
    pub fn requires_empty_stack(&self) -> bool {
        match self {
            PriorityAction::CastSpell { is_instant_speed, .. } => !is_instant_speed,
            PriorityAction::ActivateAbility { is_instant_speed, .. } => !is_instant_speed,
            PriorityAction::TakeSpecialAction { action, .. } => action.requires_empty_stack(),
            PriorityAction::PassPriority { .. } => false,
        }
    }
}

/// Represents a Special Action a player may take that does not use the stack (Rule 116).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialAction {
    /// Play a land (Rule 116.2a / 305).
    PlayLand {
        card_id: CardId,
    },
    /// Turning a face-down creature face up (Rule 116.2b / 708).
    TurnFaceUp {
        card_id: CardId,
    },
    /// Take an action at a later time to end an effect or prevent a trigger (Rule 116.2c).
    PayToStopEffect {
        effect_id: u32,
    },
    /// Take an action to ignore the effect from a static ability for a duration (Rule 116.2d).
    PayToIgnoreStaticAbility {
        ability_id: u32,
    },
    /// Discard Circling Vultures "any time you could cast an instant" (Rule 116.2e).
    DiscardCirclingVultures {
        card_id: CardId,
    },
    /// Suspend a card from hand (Rule 116.2f / 702.62).
    SuspendCard {
        card_id: CardId,
        is_instant_speed: bool,
    },
    /// Pay {3} to put a chosen companion into hand from outside the game (Rule 116.2g / 702.139).
    RetrieveCompanion {
        card_id: CardId,
    },
    /// Pay {2} and exile a card with foretell face down (Rule 116.2h / 702.143).
    ForetellCard {
        card_id: CardId,
    },
    /// Exile a card from hand with plot (Rule 116.2k / 702.170).
    PlotCard {
        card_id: CardId,
    },
    /// Pay the unlock cost of a locked half of a Room card (Rule 116.2m / 709.5).
    UnlockRoomHalf {
        card_id: CardId,
        half_index: u8,
    },
}

impl SpecialAction {
    /// Returns true if this action uses the stack.
    /// Under Rule 116.1, special actions never use the stack.
    pub fn uses_stack(&self) -> bool {
        false
    }

    /// Returns true if this action requires the stack to be empty.
    pub fn requires_empty_stack(&self) -> bool {
        match self {
            SpecialAction::PlayLand { .. } => true,
            SpecialAction::TurnFaceUp { .. } => false,
            SpecialAction::PayToStopEffect { .. } => false,
            SpecialAction::PayToIgnoreStaticAbility { .. } => false,
            SpecialAction::DiscardCirclingVultures { .. } => false,
            SpecialAction::SuspendCard { is_instant_speed, .. } => !is_instant_speed,
            SpecialAction::RetrieveCompanion { .. } => true,
            SpecialAction::ForetellCard { .. } => false,
            SpecialAction::PlotCard { .. } => true,
            SpecialAction::UnlockRoomHalf { .. } => true,
        }
    }
}

/// Represents a State-Based Action automatically generated and executed by the game (Rule 704).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateBasedAction {
    /// 704.5a: If a player has 0 or less life, that player loses the game.
    LoseByLife { player: PlayerId },
    
    /// 704.5b: If a player attempted to draw from an empty library, they lose.
    LoseByEmptyDraw { player: PlayerId },
    
    /// 704.5c: If a player has ten or more poison counters, they lose.
    LoseByPoison { player: PlayerId },
    
    /// 704.5d: If a token is in a zone other than the battlefield, it ceases to exist.
    TokenCeasesToExist { token_id: CardId },
    
    /// 704.5e: A copy of a spell outside the stack, or a copy of a card outside stack/battlefield, ceases to exist.
    CopyCeasesToExist { copy_id: CardId },
    
    /// 704.5f: If a creature has toughness 0 or less, it's put into its owner's graveyard.
    CreatureZeroToughness { creature_id: CardId, owner: PlayerId },
    
    /// 704.5g: If a creature has toughness > 0, and total damage marked >= toughness, it is destroyed.
    DestroyLethalDamage { creature_id: CardId },
    
    /// 704.5h: If a creature was dealt deathtouch damage since state-based actions were last checked, it is destroyed.
    DestroyDeathtouch { creature_id: CardId },
    
    /// 704.5i: If a planeswalker has loyalty 0, it's put into its owner's graveyard.
    PlaneswalkerZeroLoyalty { planeswalker_id: CardId, owner: PlayerId },
    
    /// 704.5j: Legend Rule. Duplicate legendary permanents with the same name owned/controlled by the same player.
    LegendRule { player: PlayerId, keep_id: CardId, discard_ids: Vec<CardId> },
    
    /// 704.5k: World Rule. Multiple permanents with world supertype. All except newest are graveyard-bound.
    WorldRule { keep_id: CardId, discard_ids: Vec<CardId> },
    
    /// 704.5m: If an Aura is attached to an illegal object/player, or unattached, it goes to its owner's graveyard.
    AuraGraveyard { aura_id: CardId, owner: PlayerId },
    
    /// 704.5n: If an Equipment or Fortification is attached to an illegal permanent, it becomes unattached.
    EquipmentUnattached { attachment_id: CardId },
    
    /// 704.5p: If a battle, creature, or non-Aura/Equipment/Fortification permanent is attached, it becomes unattached.
    OtherAttachmentUnattached { attachment_id: CardId },
    
    /// 704.5q: If a permanent has both a +1/+1 and a -1/-1 counter, equal numbers of both are removed.
    CancelCounters { permanent_id: CardId, count: u32 },
    
    /// 704.5r: If a permanent exceeds its maximum limit of a certain counter kind, excess is removed.
    RemoveExcessCounters { permanent_id: CardId, counter_type: String, count_to_remove: u32 },
    
    /// 704.5s: If a Saga's lore counters >= final chapter number, the controller sacrifices it.
    SagaSacrifice { saga_id: CardId, controller: PlayerId },
    
    /// 704.5t: A completed dungeon is removed from the game.
    RemoveDungeon { dungeon_id: CardId, owner: PlayerId },
    
    /// 704.5u: Creatures without sector designations get assigned one (Space Sculptor).
    ChooseSector { player: PlayerId, creature_ids: Vec<CardId> },
    
    /// 704.5v: If a battle has defense 0, it's put into its owner's graveyard.
    BattleZeroDefense { battle_id: CardId, owner: PlayerId },
    
    /// 704.5w: Choose an appropriate player to protect a battle with no protector.
    BattleChooseProtector { battle_id: CardId, controller: PlayerId, protector: PlayerId },
    
    /// 704.5x: Choose an opponent to protect a Siege if controller is protector.
    SiegeChooseProtector { battle_id: CardId, controller: PlayerId, protector: PlayerId },
    
    /// 704.5y: Multiple Roles controlled by same player on same permanent (all except newest go to graveyard).
    RoleRule { permanent_id: CardId, keep_id: CardId, discard_ids: Vec<CardId> },
    
    /// 704.5z: Player controls a start your engines! permanent but has speed 0 (speed becomes 1).
    SetSpeed { player: PlayerId },

    // Commander Specific State-Based Actions (Rule 704.6c & 704.6d)
    
    /// 704.6c: A player dealt 21 or more combat damage by the same commander loses.
    LoseByCommanderDamage { player: PlayerId, commander_id: CardId },
    
    /// 704.6d: A commander in graveyard or exile is put into its owner's command zone.
    CommanderToCommandZone { commander_id: CardId, owner: PlayerId, current_zone: Zone },
}

/// Models who currently has priority and the stack pass status (Rule 117).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PriorityState {
    /// The player who currently has priority, or None if no one has priority.
    pub current_priority: Option<PlayerId>,
    /// Number of consecutive players who have passed priority since the last stack addition or resolution.
    /// When this equals the number of players, the top object of the stack resolves (Rule 117.4).
    pub consecutive_passes: u32,
}

impl PriorityState {
    /// Creates a new priority state representation.
    pub fn new() -> Self {
        Self {
            current_priority: None,
            consecutive_passes: 0,
        }
    }

    /// Resets consecutive passes (e.g., when a spell is cast, ability activated, or special action taken).
    pub fn reset_passes(&mut self) {
        self.consecutive_passes = 0;
    }

    /// Registers a pass by a player.
    pub fn pass(&mut self) {
        self.consecutive_passes += 1;
    }
}
