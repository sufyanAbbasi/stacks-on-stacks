use serde::{Serialize, Deserialize};
use crate::card::{Color, CardType, Subtype};

/// Unique identifier for a card instance in play or inside a zone.
pub type CardId = u32;

/// Unique identifier for a player.
pub type PlayerId = u32;

/// Monotonically increasing counter used to order continuous effects in layers (Rule 613.7).
pub type Timestamp = u64;

/// Represents the physical or logical zone an object is in (Section 400.1).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zone {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Stack,
    Exile,
    Command,
}

/// Represents any targetable game entity (Rule 115).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Player(PlayerId),
    Card(CardId),
    Spell(CardId), // A spell object on the stack
    Any,
}

/// Represents any standard game event that can be replaced or modified (Section 614).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    DrawCard { player: PlayerId },
    DealDamage { source: Target, target: Target, amount: u32, is_combat: bool },
    GainLife { player: PlayerId, amount: u32 },
    LoseLife { player: PlayerId, amount: u32 },
    ZoneTransition { card: CardId, from: Zone, to: Zone },
    SpellCast { player: PlayerId, card: CardId },
    AbilityActivated { player: PlayerId, ability_id: u32 },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TokenCreator {
    Predefined(crate::card::PredefinedToken),
    Custom {
        name: String,
        colors: Vec<crate::card::Color>,
        supertypes: Vec<crate::card::Supertype>,
        card_types: Vec<crate::card::CardType>,
        subtypes: Vec<crate::card::Subtype>,
        power: Option<String>,
        toughness: Option<String>,
        rules_text: String,
    },
}

/// --- SECTION 610: ONE-SHOT EFFECTS ---
/// An effect that does something once and has no duration (Rule 610.1).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum OneShotEffect {
    DealDamage { amount: u32, target: Target },
    DrawCards { player: PlayerId, count: u32 },
    Destroy { target: CardId, prevent_regeneration: bool },
    Exile { target: CardId },
    Sacrifice { player: PlayerId, target: CardId },
    CreateToken { player: PlayerId, creator: TokenCreator },
    GainLife { player: PlayerId, amount: u32 },
    LoseLife { player: PlayerId, amount: u32 },
    MoveZone { card: CardId, from: Zone, to: Zone },
    AddCounters { card: CardId, counter_type: String, count: u32 },
}

/// --- SECTION 611: CONTINUOUS EFFECTS (DURATIONS) ---
/// The period of time a continuous or replacement effect remains active (Rule 611.2).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum EffectDuration {
    /// Persists as long as the permanent with the static ability remains in the relevant zone (Rule 611.3).
    StaticAbility { source_card: CardId, zone: Zone },
    /// Created by a resolving spell or ability and lasts "until end of turn" (Rule 611.2a).
    UntilEndOfTurn,
    /// Lasts "until end of combat" (Rule 611.2a).
    UntilEndOfCombat,
    /// Lasts "for as long as you control [source_card]" (Rule 611.2b).
    WhileControlled { controller: PlayerId, source_card: CardId },
    /// Created by a resolving spell/ability and lasts indefinitely (Rule 611.2a).
    Indefinite,
}

/// --- SECTION 613: INTERACTION OF CONTINUOUS EFFECTS (LAYERS) ---
/// System of layers/sublayers used to apply continuous effects in a precise order (Rule 613.1).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContinuousLayer {
    /// Layer 1: Rules and effects that modify copiable values (Rule 613.1a).
    Layer1CopiableValues,
    /// Layer 2: Control-changing effects (Rule 613.1b).
    Layer2Control,
    /// Layer 3: Text-changing effects (Rule 613.1c).
    Layer3Text,
    /// Layer 4: Type-changing effects (Rule 613.1d).
    Layer4Type,
    /// Layer 5: Color-changing effects (Rule 613.1e).
    Layer5Color,
    /// Layer 6: Ability-adding or ability-removing effects (Rule 613.1f).
    Layer6Abilities,
    /// Layer 7a: Effects from characteristic-defining abilities that define power/toughness (Rule 613.4a).
    Layer7aCharacteristicDefiningPT,
    /// Layer 7b: Effects that set power and/or toughness to a specific number (Rule 613.4b).
    Layer7bSetPT,
    /// Layer 7c: Effects and counters that modify power/toughness but don't set them (Rule 613.4c).
    Layer7cModifyPT,
    /// Layer 7d: Power and/or toughness switching effects (Rule 613.4d).
    Layer7dSwitchPT,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum EffectCondition {
    /// Matches if the card being evaluated has the specified CardType (Rule 300)
    HasType(crate::card::CardType),
    /// Matches if the card being evaluated has the specified Subtype
    HasSubtype(crate::card::Subtype),
    /// Matches if the card being evaluated has the specified Color
    HasColor(crate::card::Color),
    /// Matches if the player initiating the query is an opponent of the effect's source controller (Rule 102.2)
    IsOpponentOfSource,
    /// Matches if the player initiating the query is the controller of the effect's source
    IsSourceController,
    /// Matches if it is currently the turn of the player who controls the source of this effect
    IsSourceControllerTurn,
    /// Matches if the current phase is the specified Phase (Section 500)
    IsPhase(crate::turns::Phase),
    /// Matches if the stack is empty (Rule 117.4)
    IsStackEmpty,
    /// Negation of a condition
    Not(Box<EffectCondition>),
    /// Matches if all nested conditions are true
    And(Vec<EffectCondition>),
    /// Matches if any nested condition is true
    Or(Vec<EffectCondition>),
}

/// Represents the actual modification applied by a continuous effect.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ContinuousEffectType {
    AddAbility(String),
    RemoveAbility(String),
    ModifyPowerToughness { power_offset: i32, toughness_offset: i32 },
    SetPowerToughness { power_base: u32, toughness_base: u32 },
    ChangeControl(PlayerId),
    ChangeType(CardType),
    AddSubtype(Subtype),
    RemoveSubtype(Subtype),
    ChangeColor(Color),
    TextChange { from: String, to: String },
    TaxSpells { cost_increase: u32 },
    ActionRestriction { restrict_instructions: Vec<crate::game::SimInstruction> },
}

/// A continuous effect active in the game state (Section 611).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ContinuousEffect {
    pub id: u32,
    pub source: CardId,
    pub layer: ContinuousLayer,
    pub duration: EffectDuration,
    pub timestamp: Timestamp,
    pub effect: ContinuousEffectType,
    #[serde(default)]
    pub conditions: Vec<EffectCondition>,
}

/// --- SECTION 614: REPLACEMENT EFFECTS ---
/// An effect that watches for a game event and replaces it with a different event (Rule 614.1).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReplacementEffect {
    pub id: u32,
    pub source: CardId,
    pub duration: EffectDuration,
    pub timestamp: Timestamp,
    /// Brief text or enum matching rule logic, e.g., "enters the battlefield tapped"
    pub description: String,
}

/// --- SECTION 615: PREVENTION EFFECTS ---
/// A specialized continuous effect that prevents some amount of damage (Rule 615.1).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PreventionEffect {
    pub id: u32,
    pub source: CardId,
    pub duration: EffectDuration,
    pub timestamp: Timestamp,
    /// Maximum damage this shield can prevent before expiring (Rule 615.3).
    pub amount_remaining: u32,
    /// Scope of damage prevented (e.g. any damage, combat damage, specific target).
    pub scope: PreventionScope,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PreventionScope {
    AnyDamage,
    CombatDamageOnly,
    DamageToTarget(Target),
    DamageFromSource(CardId),
}

/// --- RULE 603.7: DELAYED TRIGGERED ABILITIES ---
/// Wait for a future event to occur, trigger once, and then resolve (Rule 603.7).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DelayedTrigger {
    pub id: u32,
    pub source: CardId,
    /// Duration of the trigger's validity, e.g., "at the beginning of the next end step" (Rule 603.7b).
    pub trigger_duration: EffectDuration,
    pub event_to_watch: String, // String representation of the trigger condition
    pub effect_to_execute: OneShotEffect,
}

/// --- RULE 609.4: "AS THOUGH" EFFECTS ---
/// Allows a player to perform an action as if some condition were true (Rule 609.4).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AsThoughEffect {
    pub id: u32,
    pub source: CardId,
    pub duration: EffectDuration,
    pub player: PlayerId,
    pub as_though_type: AsThoughType,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AsThoughType {
    /// Play/cast card from non-hand zone, e.g. "as though it were in your hand"
    CastFromZone { card: CardId, zone: Zone },
    /// Spend mana of any color to pay costs, e.g. "as though it were mana of any color"
    SpendManaAnyColor { cost_or_card: Target },
    /// Attack as though it didn't have defender
    AttackAsThoughNoDefender { creature: CardId },
}

/// --- ACTIVE EFFECTS REGISTRY ---
/// Tracks all active, long-lived effects in the game simulation state.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct ActiveEffects {
    pub continuous_effects: Vec<ContinuousEffect>,
    pub replacement_effects: Vec<ReplacementEffect>,
    pub prevention_effects: Vec<PreventionEffect>,
    pub delayed_triggers: Vec<DelayedTrigger>,
    pub as_though_effects: Vec<AsThoughEffect>,
    next_effect_id: u32,
}

impl ActiveEffects {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a continuous effect to the registry, assigning it a unique ID.
    pub fn add_continuous(&mut self, mut effect: ContinuousEffect) -> u32 {
        let id = self.next_effect_id;
        self.next_effect_id += 1;
        effect.id = id;
        self.continuous_effects.push(effect);
        id
    }

    /// Adds a replacement effect to the registry.
    pub fn add_replacement(&mut self, mut effect: ReplacementEffect) -> u32 {
        let id = self.next_effect_id;
        self.next_effect_id += 1;
        effect.id = id;
        self.replacement_effects.push(effect);
        id
    }

    /// Adds a prevention effect to the registry.
    pub fn add_prevention(&mut self, mut effect: PreventionEffect) -> u32 {
        let id = self.next_effect_id;
        self.next_effect_id += 1;
        effect.id = id;
        self.prevention_effects.push(effect);
        id
    }

    /// Adds a delayed trigger to the registry.
    pub fn add_delayed_trigger(&mut self, mut trigger: DelayedTrigger) -> u32 {
        let id = self.next_effect_id;
        self.next_effect_id += 1;
        trigger.id = id;
        self.delayed_triggers.push(trigger);
        id
    }

    /// Adds an "as though" effect to the registry.
    pub fn add_as_though(&mut self, mut effect: AsThoughEffect) -> u32 {
        let id = self.next_effect_id;
        self.next_effect_id += 1;
        effect.id = id;
        self.as_though_effects.push(effect);
        id
    }

    /// Removes all expired continuous, replacement, prevention, or as-though effects
    /// that are no longer valid (e.g. ended at turn's end).
    pub fn clear_expired_turn_effects(&mut self) {
        self.continuous_effects.retain(|e| e.duration != EffectDuration::UntilEndOfTurn);
        self.replacement_effects.retain(|e| e.duration != EffectDuration::UntilEndOfTurn);
        self.prevention_effects.retain(|e| e.duration != EffectDuration::UntilEndOfTurn);
        self.as_though_effects.retain(|e| e.duration != EffectDuration::UntilEndOfTurn);
    }

    /// Returns the continuous effects sorted precisely by layer, then by timestamp order
    /// as required by the interaction system rules (Rule 613.1 and 613.7).
    pub fn get_sorted_continuous_effects(&self) -> Vec<ContinuousEffect> {
        let mut sorted = self.continuous_effects.clone();
        // Sort by layer ascending (Layer1 to Layer7d), then by timestamp ascending (timestamp order)
        sorted.sort_by(|a, b| {
            match a.layer.cmp(&b.layer) {
                std::cmp::Ordering::Equal => a.timestamp.cmp(&b.timestamp),
                other => other,
            }
        });
        sorted
    }
}
