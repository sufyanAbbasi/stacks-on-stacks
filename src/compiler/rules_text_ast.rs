use serde::{Serialize, Deserialize};

macro_rules! define_unit_deserializer {
    ($func_name:ident, $str_val:expr) => {
        fn $func_name<'de, D>(deserializer: D) -> Result<(), D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct Visitor;
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value = ();
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str(concat!("the string '", $str_val, "'"))
                }
                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if v == $str_val {
                        Ok(())
                    } else {
                        Err(E::custom(concat!("expected '", $str_val, "'")))
                    }
                }
            }
            deserializer.deserialize_any(Visitor)
        }
    };
}

define_unit_deserializer!(deserialize_take_extra_turn, "takeExtraTurn");
define_unit_deserializer!(deserialize_flip_coin, "flipCoin");
define_unit_deserializer!(deserialize_win_flip, "winFlip");
define_unit_deserializer!(deserialize_lose_flip, "loseFlip");
define_unit_deserializer!(deserialize_populate, "populate");
define_unit_deserializer!(deserialize_end_turn, "endTurn");
define_unit_deserializer!(deserialize_forage, "forage");
define_unit_deserializer!(deserialize_manifest_dread, "manifestDread");
define_unit_deserializer!(deserialize_investigate, "investigate");
define_unit_deserializer!(deserialize_ring_tempts, "ringTempts");


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum CounterKind {
    Simple(String),
    PT(PTModification),
}

/// Represents the top-level AST of a parsed MTG card's rules text.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CardAST {
    pub name: String,
    pub abilities: Vec<AbilityOrRemind>,
}

/// Represents an item in the card text, which can be a functional ability or reminder text.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum AbilityOrRemind {
    Ability(Ability),
    Reminder(String),
}

/// Represents all possible categories of functional abilities on an MTG card.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Ability {
    Activated(ActivatedAbility),
    Triggered(TriggeredAbility),
    StaticOrSpell(Sentence),
    StaticOrSpellList(Vec<Sentence>),
    Keyword(Vec<KeywordAbility>),
    AbilityWord {
        word: String,
        ability: Box<Ability>,
    },
    Modal(ModalAbility),
    AdditionalCost {
        additional_cost: Imperative,
    },
}

/// Represents either a single cost or multiple costs for activated abilities.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Costs {
    Single(Cost),
    Multiple(Vec<Cost>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum SentenceOrList {
    Single(Sentence),
    Multiple(Vec<Sentence>),
}

/// Represents an activated ability as parsed by the grammar.
/// Example: `{T}: Create a 1/1 white Human creature token.`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActivatedAbility {
    pub costs: Costs,
    pub activated_ability: SentenceOrList,
    pub instructions: Option<ActivationInstructions>,
}

/// Represents a triggered ability.
/// Example: `Whenever a creature enters the battlefield, you gain 1 life.`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TriggeredAbility {
    pub trigger: TriggerCondition,
    pub effect: SentenceOrList,
    pub if_clause: Option<Condition>,
}

/// Represents a modal ability.
/// Example: `Choose one — • Deal 3 damage; • Draw a card.`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModalAbility {
    pub quantifier: Vec<u32>,
    pub options: Vec<Sentence>,
}

/// Helper to specify who and how an activated ability may be triggered.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ActivationInstructions {
    Only {
        only: ActivationInstruction,
    },
    AnyPlayer,
}

/// Represents conditions on activating an ability.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ActivationInstruction {
    OnceATurn,
    SorceryOnly,
    Controls {
        actor: Player,
        controls: Object,
    },
    Condition(Condition),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ManaSymbol {
    Number(u32),
    Symbol(String),
}

/// Represents a cost requirement for activated abilities.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Cost {
    Tap(String), // Match "{t}" -> "tap"
    Mana {
        mana: Vec<ManaSymbol>,
    },
    Loyalty {
        loyalty: i8,
    },
    Sentence(Box<Sentence>),
    And {
        and: Vec<Cost>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TurnPhaseDetail {
    pub qualification: Option<TurnPhaseQualification>,
    pub part_of_turn: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TurnPhaseQualification {
    pub whose: Player,
}

/// Represents trigger conditions for triggered abilities.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum TriggerCondition {
    When {
        when: TriggerConditionInner,
        timing: Option<String>,
    },
    TurnPhase {
        #[serde(rename = "turnPhase")]
        turn_phase: TurnPhaseDetail,
        timing: Option<String>,
    },
}

/// Isomorphic representation of different sub-trigger clauses.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum TriggerConditionInner {
    Sentence(Sentence),
    TurnPhase {
        turn_phase: String,
    },
    EndCombat {
        turn_phase: String, // "endCombat"
    },
    GainsLife {
        actor: Player,
        does: String, // "gainsLife"
    },
    DealtDamage {
        what: Object,
        does: String, // "dealtDamage"
    },
    VerbPhrase {
        what: Object,
        does: ObjectVerbPhrase,
    },
}

/// Isomorphic representation of sentences or effects parsed by the grammar.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Sentence {
    Simple(Box<Imperative>),
    ObjectVerbPhrase {
        what: Object,
        does: Box<ObjectVerbPhrase>,
    },
    IsWhat {
        is: IsWhat,
    },
    PlayerVerbPhrase {
        actor: Player,
        does: Box<PlayerVerbPhrase>,
    },
    ConditionEffect {
        condition: Condition,
        effect: Box<Sentence>,
    },
    WouldInstead {
        what: Object,
        does: ObjectVerbPhrase,
        instead: Box<Sentence>,
    },
    AsLongAs {
        as_long_as: Condition,
        effect: Box<Sentence>,
    },
    Duration {
        duration: Duration,
        effect: Box<Sentence>,
    },
    ForEach {
        for_each: Object,
        effect: Box<Sentence>,
    },
    EqualCharacteristic {
        what: Possessive,
        characteristic: String,
        set_to: NumberDefinition,
    },
    AsSentence {
        as_clause: Box<Sentence>,
        does: Box<Sentence>,
    },
    Instead {
        instead: Box<Sentence>,
    },
    MaxHandSize {
        whose: Possessive,
        hand_size: String,
        amount: NumberDefinition,
    },
    And {
        and: Vec<Sentence>,
    },
    Otherwise {
        otherwise: Box<Sentence>,
    },
    RatherThan {
        does: Box<Sentence>,
        rather_than: Box<Sentence>,
    },
    AtPartOfTurn {
        does: Box<Sentence>,
        at: PartOfTurn,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum AddOneOfValue {
    Simple(Vec<String>),
    Nested(Vec<Vec<String>>),
}

/// Represents commands and imperatives (actions taken upon spell or ability resolution).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Imperative {
    Sacrifice {
        sacrifice: Object,
    },
    Destroy {
        destroy: Object,
    },
    Detain {
        detain: Object,
    },
    Discard {
        discard: Object,
        #[serde(default)]
        random: bool,
    },
    Return {
        returns: Object,
        from: Option<Zone>,
        to: Zone,
        #[serde(default)]
        tapped: bool,
        attached: Option<Object>,
    },
    Exile {
        exile: Object,
        #[serde(default)]
        face_down: bool,
        until: Option<UntilClause>,
    },
    Create {
        create: TokenDescription,
    },
    Copy {
        copy: Object,
        times: Option<CountableCount>,
    },
    LoseLife {
        lose_life: NumberDefinition,
    },
    Mill {
        mill: NumberDefinition,
    },
    GainLife {
        gain_life: NumberDefinition,
    },
    GainsControlOf {
        gains_control_of: Object,
        until: Option<UntilClause>,
    },
    RemoveCounters {
        count: CountableCount,
        remove_counters_from: Object,
        counter_kind: Option<CounterKind>,
    },
    PutCounters {
        amount: NumberDefinition,
        counter_kind: CounterKind,
        put_on: Object,
    },
    Cast {
        cast: Object,
        #[serde(default)]
        without_paying: bool,
        duration: Option<Duration>,
        only_during: Option<PartOfTurn>,
        each: Option<PartOfTurn>,
    },
    Play {
        play: Object,
        #[serde(default)]
        without_paying: bool,
        duration: Option<Duration>,
        only_during: Option<PartOfTurn>,
        each: Option<PartOfTurn>,
    },
    Surveil {
        surveil: NumberDefinition,
    },
    Search {
        search: Zone,
        criteria: Option<Object>,
    },
    Choose {
        choose: ChooseAction,
    },
    Draw {
        draw: DrawCount,
    },
    Shuffle {
        shuffle: Zone,
    },
    ShuffleInto {
        shuffle: ObjectOrZone,
        into: Zone,
    },
    Counter {
        counter: Object,
    },
    Tap {
        tap: Object,
    },
    TapOrUntap {
        does: TapOrUntapAction,
        to: Object,
    },
    Untap {
        tap: Object,
        #[serde(default)]
        goad: bool,
        when: Option<PartOfTurn>,
    },
    #[serde(deserialize_with = "deserialize_take_extra_turn")]
    TakeExtraTurn,
    Scry {
        scry: NumberDefinition,
    },
    Pay {
        pay: Box<Cost>,
        rather_than_cost_of: Option<Object>,
    },
    AddManaOneOf {
        add_one_of: AddOneOfValue,
        #[serde(default)]
        amount: Option<NumberDefinition>,
    },
    AddManaCombination {
        add_combination_of: Vec<String>,
        amount: NumberDefinition,
    },
    PreventDamage {
        amount: NumberDefinition,
        prevent: String,
        to: Option<PreventTarget>,
        duration: Option<Duration>,
    },
    LookAtTop {
        look_at_top: NumberDefinition,
        from: Zone,
        #[serde(default)]
        any_order: bool,
    },
    LookAt {
        look_at: Object,
    },
    Reveal {
        reveal: ObjectOrZone,
        #[serde(default)]
        random: bool,
        from: Option<Zone>,
    },
    PutIntoZone {
        put: Object,
        into: Zone,
        #[serde(default)]
        tapped: bool,
        control: Option<Possessive>,
    },
    May {
        may: Box<Sentence>,
        if_do: Option<Box<Sentence>>,
    },
    HaveProperty {
        have: Object,
        property: ObjectProperty,
    },
    HaveActor {
        actor: Player,
        does: Box<PlayerVerbPhrase>,
    },
    HaveLifeTotal {
        life_total_becomes: NumberDefinition,
    },
    HaveComparison {
        comparison: NumericalComparison,
        value: NumberDefinition,
    },
    ForEachImperative {
        does: Box<Imperative>,
        for_each: Object,
    },
    Unless {
        does: Box<Imperative>,
        unless: Box<Sentence>,
    },
    ChooseNewTargets {
        choose: NewTargetsChoice,
    },
    SwitchPowerToughness {
        switch_power_toughness: Object,
        until: UntilClause,
    },
    DoSameFor {
        do_same_for: Object,
    },
    SpendManaAsAnyType {
        spend_mana_as_any_type_for: Object,
    },
    Transform {
        transform: Object,
    },
    #[serde(deserialize_with = "deserialize_flip_coin")]
    FlipCoin,
    #[serde(deserialize_with = "deserialize_win_flip")]
    WinFlip,
    #[serde(deserialize_with = "deserialize_lose_flip")]
    LoseFlip,
    Regenerate {
        regenerate: Object,
    },
    Bolster {
        bolster: NumberDefinition,
    },
    #[serde(deserialize_with = "deserialize_populate")]
    Populate,
    Support {
        support: NumberDefinition,
    },
    Attach {
        attach: Object,
        to: Object,
    },
    #[serde(deserialize_with = "deserialize_end_turn")]
    EndTurn,
    CastComparison {
        cast: CastComparisonDetail,
    },
    #[serde(deserialize_with = "deserialize_forage")]
    Forage,
    #[serde(deserialize_with = "deserialize_manifest_dread")]
    ManifestDread,
    Blight {
        blight: NumberDefinition,
    },
    Airbend {
        airbend: Object,
    },
    Earthbend {
        earthbend: NumberDefinition,
    },
    Waterbend {
        waterbend: Cost,
    },
    Harness {
        harness: Object,
    },
    Discover {
        discover: NumberDefinition,
    },
    Incubate {
        incubate: NumberDefinition,
    },
    CollectEvidence {
        collect_evidence: NumberDefinition,
    },
    Suspect {
        suspect: Object,
    },
    Level {
        level: NumberDefinition,
    },
    PlayWithRevealed {
        play_with_revealed: Object,
    },
    #[serde(deserialize_with = "deserialize_investigate")]
    Investigate,
    #[serde(deserialize_with = "deserialize_ring_tempts")]
    RingTempts,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ObjectProperty {
    Infinitive(ObjectInfinitive),
    VerbPhrase(ObjectVerbPhrase),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ObjectInfinitive {
    BePut {
        enter: Zone,
        duration: Duration,
    },
    BeCreatedUnderYourControl {
        reference: CreateControlReference,
        does: String, // "create"
    },
    Fight {
        fight: Object,
    },
    Deal {
        deal: DealsWhat,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateControlReference {
    pub actor: String, // "you"
    pub does: String, // "control"
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CantClause {
    Simple(String), // "attack", "blocked", "countered", "regenerate"
    Block {
        block: Option<Object>,
    },
    Or {
        or: Vec<CantClauseAction>,
    },
    ActionWithSuffix {
        does: String,
        suffix: String, // "alone"
    },
    BlockedBy {
        #[serde(rename = "blockedBy")]
        blocked_by: BlockedByCount,
    },
    Enchanted {
        does: String, // "enchant"
        what: Option<Object>,
    },
    ObjectVerbPhrase(Box<ObjectVerbPhrase>),
    WithUnless {
        cant: Box<CantClause>,
        unless: Condition,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum CantClauseAction {
    Simple(String),
    ActionWithSuffix {
        does: String,
        suffix: String, // "alone"
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockedByCount {
    pub gt: NumberDefinition,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum PreventTarget {
    Recipient(Entity),
    Any(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ObjectOrZone {
    Object(Box<Object>),
    Zone(Zone),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TapOrUntapAction {
    pub or: Vec<String>, // ["tap", "untap"]
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NewTargetsChoice {
    pub new_targets: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CastComparisonDetail {
    pub comparison: NumericalComparison,
    pub what: String,
    pub duration: Duration,
}

/// Represents player-specific actions or properties.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PlayerVerbPhrase {
    LifeGain {
        life_gain: NumberDefinition,
    },
    LifeGainEqual {
        life_gain: LifeGainEqualDetail,
    },
    ForEach {
        does: Box<PlayerVerbPhrase>,
        for_each: Object,
    },
    FirstTime {
        does: Box<PlayerVerbPhrase>,
        reference: String, // "firstTime"
        duration: Duration,
    },
    Controls {
        controls: Object,
    },
    NotControls {
        not: ControlsNegation,
    },
    Owns {
        owns: Object,
    },
    DoesntEmpty(String), // "doesntEmpty"
    PutsIntoZone {
        what: Object,
        enters: Zone,
    },
    Surveil(String), // "surveil"
    LifeTotalBecomes {
        life_total_becomes: NumberDefinition,
    },
    Attack {
        does: String, // "attack"
        who: Option<Player>,
        creatures: Option<NumberDefinition>,
        duration: Option<Duration>,
    },
    Imperative(Imperative),
    Then {
        and: Vec<PlayerVerbPhrase>,
    },
    Cant {
        cant: Imperative,
    },
    NotDo {
        not: String, // "do"
    },
    DoAction(String), // "do"
    LoseGame(String), // "lose"
    IfCondition {
        does: Box<PlayerVerbPhrase>,
        condition: Condition,
    },
    ThisWay {
        does: Box<PlayerVerbPhrase>,
        reference: String, // "thisWay"
    },
    GetEmblem {
        emblem: Condition,
    },
    Each {
        each: Box<PlayerVerbPhrase>,
    },
    Cycle {
        cycle: Object,
    },
    HasNoCardsInHand {
        not: HasInHandNegation,
    },
    HasVerb {
        what: Object,
        does: ObjectVerbPhrase,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LifeGainEqualDetail {
    pub whose: Possessive,
    pub value: String, // e.g. "power", "toughness"
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlsNegation {
    pub controls: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HasInHandNegation {
    pub has: HasInHandDetail,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HasInHandDetail {
    pub what: String, // "card"
    pub in_zone: String, // "hand"
}

/// Represents characteristics/actions applied to permanents or spells.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ObjectVerbPhrase {
    Is {
        is: Box<IsWhat>,
    },
    HasAbility {
        have_ability: AcquiredAbility,
        as_long_as: Option<Condition>,
    },
    BasePowerToughness {
        base_power_toughness: PT,
    },
    GainsAbility {
        gains: AcquiredAbility,
    },
    GetsPT {
        power_toughness_mod: PTModification,
        for_each: Option<Object>,
        gains: Option<AcquiredAbility>,
        until: Option<UntilClause>,
        as_long_as: Option<Condition>,
    },
    EntersBattlefieldCounters {
        enters_with: EntersWithCountersDetail,
    },
    EntersBattlefieldComplex {
        enter: String, // "battlefield"
        #[serde(default)]
        tapped: bool,
        #[serde(default)]
        prepared: bool,
        control: Option<Possessive>,
        from: Option<Zone>,
        #[serde(rename = "with")]
        with_counters: Option<EntersWithCountersSimple>,
    },
    LeavesBattlefield {
        leaves: String, // "battlefield"
    },
    Dies(String), // "die"
    PutIntoZone {
        enter: Zone,
        from: Option<Zone>,
    },
    Cant {
        cant: CantClause,
    },
    DealsDamage {
        deal: DealsWhat,
    },
    Blocks(String), // "blocks"
    UntapsDuring {
        untap: PartOfTurn,
    },
    BlocksOrBlockedBy {
        or: Vec<BlockRelation>,
    },
    CounteredThisWay {
        reference: String, // "thisWay"
        does: String, // "countered"
    },
    Fights {
        fights: Object,
    },
    Targets {
        targets: Object,
    },
    LosesKeyword {
        loses: String,
    },
    CostReduction {
        cost_reduction: CostReductionDetail,
    },
    Ignores {
        ignores: String,
    },
    BlockAdditional {
        block_additional: Object,
    },
    DoSo(String), // "do"
    RemainsExiled {
        remain: String, // "exile"
    },
    Becomes {
        #[serde(rename = "become")]
        become_action: BecomesWhat,
    },
    Attacks {
        must_attack: Option<String>, // "this" or "each"
    },
    AttacksSimple(String), // "attacks"
    LosesAllAbilities {
        loses: String, // "allAbilities"
        until: Option<UntilClause>,
    },
    Created(String), // "created"
    CausesPlayer {
        cause: CausePlayerDetail,
    },
    ForEach {
        does: Box<ObjectVerbPhrase>,
        for_each: Object,
    },
    Duration {
        does: Box<ObjectVerbPhrase>,
        duration: Duration,
    },
    IfCondition {
        does: Box<ObjectVerbPhrase>,
        condition: Condition,
    },
    Kicked(String), // "kicked"
    MilledThisWay {
        reference: String, // "thisWay"
        does: String, // "milled"
    },
    CastFromGraveyard {
        does: String, // "cast"
        from: String, // "graveyard"
    },
    CantBeCountered {
        cant: String, // "countered"
    },
    CantPreventDamage {
        cant_prevent: String, // "damage"
    },
    CantAttack {
        cant_attack: Duration,
    },
    CantBeBlocked {
        cant: BlockNegationDetail,
    },
    CostIncrease {
        cost_increase: Cost,
        action: String, // "cast" or "activate"
    },
    AsObject {
        as_clause: Object,
        #[serde(default)]
        in_addition: bool,
    },
    AssignCombatDamageAsNotBlocked {
        damage: DamageAsNegationDetail,
    },
    RemainsTapped {
        remains: String, // "tapped"
    },
    AndOr {
        and: Option<Vec<ObjectVerbPhrase>>,
        or: Option<Vec<ObjectVerbPhrase>>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum BlockRelation {
    Blocks { blocks: Object },
    BlockedBy { blocked_by: Object },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CostReductionDetail {
    pub mana: Vec<ManaSymbol>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CausePlayerDetail {
    pub actor: Player,
    pub does: Box<PlayerVerbPhrase>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockNegationDetail {
    pub blocked_by: BlockedByTarget,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum BlockedByTarget {
    Recipient(Object),
    Any(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DamageAsNegationDetail {
    pub as_clause: NotBlockedNegation,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NotBlockedNegation {
    pub not: String, // "blocked"
}

/// Helper structures representing power and toughness, modifications, and counters.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntersWithCountersDetail {
    pub amount: NumberDefinition,
    pub counter_kind: CounterKind,
    pub for_each: Option<Object>,
    #[serde(default)]
    pub additional: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EntersWithCountersSimple {
    pub amount: NumberDefinition,
    pub counter_kind: CounterKind,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PT {
    pub power: i32,
    pub toughness: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PTModification {
    pub power_mod: i32,
    pub toughness_mod: i32,
}

/// Represents what properties a permanent or spell has or becomes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum IsWhat {
    Color {
        color: String,
    },
    ObjectAndAddition {
        object: Box<Object>,
        #[serde(default)]
        in_addition: bool,
    },
    InZone {
        in_zone: Zone,
    },
    Still {
        still: Box<Object>,
    },
    Status(String), // e.g. "turnedFaceUp", "attacking", "blocking"
    ComplexStatus {
        or: Vec<String>, // ["attacking", "blocking"]
    },
    Condition(Box<Condition>),
}

/// Represents target / parameter descriptions for coin flips, choices, etc.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum ChooseAction {
    Object(Object),
    Type {
        #[serde(rename = "type")]
        choose_type: CardType,
    },
    NotImperative {
        not: Box<Imperative>,
    },
    Named(String), // "cardName"
    Color(String), // "color"
}

/// Represents what properties a permanent becomes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum BecomesWhat {
    Status(String), // "tap"
    NotStatus {
        not: String, // "tap"
    },
    Unattached {
        not: Option<UnattachedDetail>,
    },
    Copy {
        copy_of: Object,
        except: Option<String>,
    },
    ComplexBecome {
        #[serde(rename = "type")]
        become_type: CardType,
        color: Option<String>,
        size: Option<PT>,
        with_ability: Option<AcquiredAbility>,
        #[serde(default)]
        in_addition: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UnattachedDetail {
    pub does: String, // "attached"
    pub to: Object,
}

/// Represents the amount/count for drawing cards.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum DrawCount {
    Fixed(u32),
    Variable(NumberDefinition),
}

/// Represents the count for things like counting/removing counters.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CountableCount {
    Fixed(u32),
    Eq { eq: u32 },
    AtLeast { at_least: u32 },
    FewerThan { fewer_than: u32 },
    UpTo { up_to: u32 },
    AnyNumber(String), // "anyNumber"
    All(String),       // "all"
    Both(String),      // "both"
}

/// Represents any numeric property or calculation (could be X, Y, or fixed numbers).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum NumberDefinition {
    Fixed(u32),
    Variable(String), // "x", "y", "z"
    Complex {
        reference: String, // "that"
        what: String,      // "amount"
    },
}

/// Represents numerical comparisons (e.g. at least, or greater, or less, gt, lte).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum NumericalComparison {
    Gte { gte: NumberDefinition },
    Lte { lte: NumberDefinition },
    Gt { gt: NumberDefinition },
    Lt { lt: NumberDefinition },
    Simple(NumberDefinition),
}

/// Represents target specifications (including complex referencing prefixes/suffixes).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Object {
    Referenced {
        reference: String,
        object: Box<Object>,
    },
    Simple(String), // "it", "them", "they", "rest", "emblem"
    Condition {
        object: Box<Object>,
        condition: Box<IsWhat>,
    },
    Connected {
        and: Option<Vec<Object>>,
        or: Option<Vec<Object>>,
    },
    Pure(Box<PureObject>),
    Each {
        each: Box<Object>,
    },
    TopCards {
        top_cards: NumberDefinition,
        from: Zone,
    },
    TopCardSimple {
        top_cards: u32, // 1
        whose: Possessive,
        what: String, // "libary"
    },
    CountersOn {
        counter_type: Option<String>,
        counters_on: Box<Object>,
    },
}

/// High-fidelity structural specifications for cards, tokens, and spells.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum PureObject {
    WithSuffix {
        object: Box<PureObject1>,
        suffix: Box<Suffix>,
    },
    Simple(PureObject1),
}

/// Represents types and connected lists of types in card text (like Creature, or Human Creature).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum CardType {
    Simple(String),
    And {
        and: Vec<CardType>,
    },
    Or {
        or: Vec<CardType>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PureObject1 {
    pub object: Option<PureObjectInner>,
    #[serde(rename = "type")]
    pub object_type: Option<CardType>,
    pub prefixes: Option<Vec<Prefix>>,
    pub suffix: Option<Box<Suffix>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PureObjectInner {
    Copy {
        copy_of: Option<Box<Object>>,
    },
    Simple(String), // "card", "spell", "type", "ability", "commander", "token", "target", etc.
    WithoutAbility {
        object: Box<PureObject>,
        without: String, // keyword name
    },
    WithCondition {
        object: Box<PureObject>,
        condition: Suffix,
    },
    NamedCard(String), // Card Name
}

/// Represents referencing suffixes used in card abilities.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Suffix {
    Ownership {
        actor: Player,
        does: OwnershipRelation,
    },
    Location {
        in_zone: Zone,
    },
    Negated {
        not: Box<Suffix>,
    },
    ThisWay {
        reference: String, // "thisWay"
        does: String,      // "reveal", "sacrifice", "tap", etc.
    },
    FromSource {
        from: Box<ObjectOrZone>,
    },
    CastStatus(String), // "youCast"
    DidAction {
        did_action: String,
        when: Option<Duration>,
    },
    TargetsOnly {
        only_targets: Box<Object>,
    },
    TargetsRecipient {
        targets: Box<Entity>,
    },
    Status(String), // "tappedThisWay", "amongThem", "youveCastBeforeThisTurn"
    ChoiceSelection {
        #[serde(rename = "type")]
        choice_type: CardType,
        actor: Player,
        does: String, // "choose"
    },
    TargetableBy {
        could_target: Box<Object>,
    },
    Blockable {
        can_block: Box<Object>,
    },
    ConvokedBy {
        convoked: Box<Object>,
    },
    Named {
        named: String,
    },
    NotNamed {
        not: NamedTarget,
    },
    AttachedTo {
        attached_to: Box<Object>,
    },
    TargetRelation {
        what: String, // "it"
        does: String, // "targets"
    },
    OtherThan {
        not: Box<Object>,
    },
    BasePT {
        base_power_toughness: PT,
    },
    TotalCharacteristic {
        total: CharacteristicComparison,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OwnershipRelation {
    pub not: Option<String>, // Option for negated control/ownership
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NamedTarget {
    pub named: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum CharacteristicComparison {
    Power(NumberDefinition),
    Toughness(NumberDefinition),
}

/// Represents qualifying prefixes for MTG entities.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Prefix {
    Simple(String), // "enchanted", "the", "first", "attached", "equipped", "historic", "exiled", "revealed", "token", "faceDown", "tapped", "attacking", "blocking", "other"
    Negated {
        not: Box<Prefix>,
    },
    NegatedProperty {
        not: String, // e.g. "token" -> nontoken
    },
    AbilityType {
        ability_type: String, // "activated", "triggered"
    },
    Color {
        color: String,
    },
    Size {
        size: PT,
    },
    Connected {
        and: Option<Vec<Prefix>>,
        or: Option<Vec<Prefix>>,
    },
}

/// Represents players or player groupings in card rules.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Player {
    You(String), // "you"
    They(String), // "they"
    Referenced {
        references: Vec<String>,
        player: PurePlayer,
    },
    Opponents(String), // "opponent" or "opponents"
    DefendingPlayer(String), // "defendingPlayer"
    Ownership {
        whose: Box<Possessive>,
        does: String, // "control" or "own"
    },
    Each {
        each: Box<Player>,
    },
    Team(String), // "team"
    Connected {
        and: Option<Vec<Player>>,
        or: Option<Vec<Player>>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum PurePlayer {
    Player(String), // "player"
    Opponents(String), // "opponents"
    NoOne(String), // "noone"
}

/// Isomorphic representation of game zones (library, hand, battlefield, graveyard, exile).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Zone {
    Simple(String), // "exile", "battlefield", "it", "anywhere"
    Owned {
        owner: Possessive,
        zone: OwnedZone,
    },
    Multiple(Vec<Zone>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum OwnedZone {
    Simple(String), // "graveyard", "library", "hand"
    Connected {
        and: Option<Vec<OwnedZone>>,
        or: Option<Vec<OwnedZone>>,
    },
}

/// Represents either a Player or an Object.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Entity {
    Player(Player),
    Object(Object),
}

/// Represents durational qualifiers for continuous effects.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Duration {
    Simple(String), // "thisTurn"
    Complex {
        reference: String,
        what: String, // "turn", "step", "phase"
    },
    Until(UntilClause),
    AsLongAs {
        as_long_as: Box<Condition>,
    },
    Each {
        each: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum UntilValue {
    Simple(String),
    Sentence(Box<Sentence>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UntilClause {
    pub until: UntilValue,
}

/// Represents conditional qualifiers used across rules text.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Condition {
    Simple(String), // "yourTurn", "untapped"
    DoneDuration {
        done: String, // "scried", "surveilled"
        duration: Box<Duration>,
    },
    NotTurn {
        not_turn_of: Possessive,
    },
    HasCounter {
        object: Object,
        count: CountableCount,
        has_counter: Option<String>,
    },
    Comparison {
        number: NumberDefinition,
        is: NumericalComparison,
    },
    ManaSpentOn {
        mana_spent_on: Object,
    },
    PairedWith {
        paired_with: Box<Object>,
    },
    HasChosenName {
        what: Object,
        has: ChosenNameDetail,
    },
    ManaSpentCondition {
        condition: Box<Condition>,
        value: ManaSpentValue,
    },
    KickedWith {
        what: Object,
        kicked: KickedDetail,
    },
    HasCounterDetails {
        what: Object,
        has: CounterQuantityDetail,
    },
    Complex(Box<Sentence>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChosenNameDetail {
    pub reference: String, // "chosen"
    pub what: String, // "name"
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ManaSpentValue {
    pub what: String, // "mana"
    pub reference: ReferencingAction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReferencingAction {
    pub does: String, // "spent"
    pub reference: String, // "this"
    pub what: String, // "spell"
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct KickedDetail {
    pub with: KickedWithMana,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct KickedWithMana {
    pub manacost: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CounterQuantityDetail {
    pub amount: NumberDefinition,
    pub counter_kind: CounterKind,
}

/// Represents possessive descriptions (ownership).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Possessive {
    Simple(String), // "your", "their"
    Player(Box<Player>),
}

/// Represents description of tokens to be created.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TokenDescription {
    pub amount: NumberDefinition,
    #[serde(rename = "type")]
    pub token_type: Option<CardType>,
    pub size: Option<PT>,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MayhemDetail {
    pub mayhem: Option<Cost>,
}

/// Represents keyword abilities, both parameterized and basic.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum KeywordAbility {
    Basic(String), // e.g. "flying", "haste", "vigilance", "first strike", "flash"
    Cycling { cycling: Cost },
    Enchant { enchant: Entity },
    Equip { equip: Cost },
    CumulativeUpkeep { cumulative_upkeep: Cost },
    Escape { escape: Cost },
    Spectacle { spectacle: Cost },
    Afterlife { afterlife: u32 },
    Afflict { afflict: u32 },
    Eternalize { eternalize: Cost },
    Embalm { embalm: Cost },
    Fabricate { fabricate: u32 },
    Crew { crew: u32 },
    Escalate { escalate: Cost },
    Emerge { emerge: Cost },
    Surge { surge: Cost },
    Awaken { awaken: Cost },
    Renown { renown: u32 },
    Dash { dash: Cost },
    Outlast { outlast: Cost },
    Tribute { tribute: u32 },
    Mutate { mutate: Cost },
    Bestow { bestow: Cost },
    Scavenge { scavenge: Cost },
    Overload { overload: Cost },
    Buyback { buyback: Cost },
    Rampage { rampage: u32 },
    Echo { echo: Cost },
    Fading { fading: u32 },
    Kicker { kicker: Cost },
    Flashback { flashback: Cost },
    Madness { madness: Cost },
    Morph { morph: Cost },
    Amplify { amplify: u32 },
    Entwine { entwine: Cost },
    Modular { modular: u32 },
    Bushido { bushido: u32 },
    Ninjutsu { ninjutsu: Cost },
    Dredge { dredge: u32 },
    Transmute { transmute: Cost },
    Bloodthirsty { bloodthirsty: u32 },
    Replicate { replicate: Cost },
    Graft { graft: u32 },
    Recover { recover: Cost },
    Ripple { ripple: u32 },
    Suspend { suspend: u32, cost: Cost },
    Vanishing { vanishing: u32 },
    Absorb { absorb: u32 },
    Fortify { fortify: Cost },
    Frenzy { frenzy: u32 },
    Poisonous { poisonous: u32 },
    Evoke { evoke: Cost },
    Devour { devour: u32 },
    Unearth { unearth: Cost },
    Annihilator { annihilator: u32 },
    LevelUp { level_up: Cost },
    Miracle { miracle: Cost },
    Megamorph { megamorph: Cost },
    AffinityFor { affinity_for: Object },
    Partner(String), // "partner"
    PartnerWith { partner_with: String },
    Offering { offering: Object },
    Soulshift { soulshift: u32 },
    SpliceOnto { splice_onto: Object, cost: Cost },
    Forecast { forecast: ActivatedAbility },
    Champion { champion: Object },
    ProtectionFrom { protection_from: Entity },
    Prowl { prowl: Cost },
    Reinforce { reinforce: u32, cost: Cost },
    Transfigure { transfigure: Cost },
    BandsWith { bands_with: String },
    Landwalk { landwalk: Suffix }, // walk or nonbasic walk
    AuraSwap { aura_swap: Cost },
    Foretell { foretell: Cost },
    Boast { boast: Cost },
    Disturb { disturb: Cost },
    Cleave { cleave: Cost },
    Squad { squad: Cost },
    Prototype { prototype: Cost },
    Reconfigure { reconfigure: Cost },
    Blitz { blitz: Cost },
    Casualty { casualty: u32 },
    Toxic { toxic: u32 },
    Backup { backup: u32 },
    Craft { craft: Cost },
    Disguise { disguise: Cost },
    Ward { ward: Cost },
    Plot { plot: Cost },
    Saddle { saddle: u32 },
    Freerunning { freerunning: Cost },
    Gift { gift: String },
    Offspring { offspring: Cost },
    Impending { impending: u32, cost: Cost },
    Exhaust { exhaust: ActivatedAbility },
    MaxSpeed { max_speed: Box<Ability> },
    Harmonize { harmonize: Cost },
    Mobilize { mobilize: u32 },
    Warp { warp: Cost },
    Mayhem(MayhemDetail),
    WebSlinging { web_slinging: Cost },
    Firebending { firebending: u32 },
    Sneak { sneak: Cost },
}

/// Represents phases, steps, or components of a turn.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged, rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum PartOfTurn {
    Simple(String), // "combat"
    Complex {
        qualification: String,
        part_of_turn: String,
    },
}

/// Represents detailed elements of a game's structure or characteristics.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AcquiredAbility {
    pub ability: Box<Ability>,
}

/// Represents referencing descriptions.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DealsWhat {
    pub amount: NumberDefinition,
    pub damage_to: Option<DamageToTarget>,
    pub divide_among: Option<Vec<Object>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum DamageToTarget {
    Recipient(Entity),
    Any(String), // "anyTarget"
    SelfTarget(String), // "self"
}

/// Represents activated abilities passive properties or descriptors.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActivatedAbilities {
    pub whose: Option<Possessive>,
    pub activated_abilities: String, // "any" or "true"
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ActivatedAbilitiesVP {
    pub cant: String, // "activatedAbilities"
    pub unless: Option<String>, // "manaAbility"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one_of_value() {
        let json_str = r#"[["c", "c"]]"#;
        let res: Result<AddOneOfValue, _> = serde_json::from_str(json_str);
        println!("AddOneOfValue result: {:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_direct_struct() {
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct TestAddManaOneOf {
            add_one_of: AddOneOfValue,
            #[serde(default)]
            amount: Option<NumberDefinition>,
        }
        let json_str = r#"{
            "addOneOf": [
                [
                    "c",
                    "c"
                ]
            ]
        }"#;
        let res: Result<TestAddManaOneOf, _> = serde_json::from_str(json_str);
        println!("TestAddManaOneOf result: {:?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_add_mana_one_of() {
        let json_camel = r#"{
            "addOneOf": [
                [
                    "c",
                    "c"
                ]
            ]
        }"#;
        let res_camel: Result<Imperative, _> = serde_json::from_str(json_camel);
        println!("Imperative camel result: {:?}", res_camel);

        let json_snake = r#"{
            "add_one_of": [
                [
                    "c",
                    "c"
                ]
            ]
        }"#;
        let res_snake: Result<Imperative, _> = serde_json::from_str(json_snake);
        println!("Imperative snake result: {:?}", res_snake);

        assert!(res_camel.is_ok() || res_snake.is_ok());
    }

    #[test]
    fn test_sentence_add_mana_one_of() {
        let json_str = r#"{
            "addOneOf": [
                [
                    "c",
                    "c"
                ]
            ]
        }"#;
        let res: Result<Sentence, _> = serde_json::from_str(json_str);
        println!("Sentence result: {:?}", res);
        assert!(res.is_ok());
    }
}


