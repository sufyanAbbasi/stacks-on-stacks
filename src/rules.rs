struct Condition {
    precondition: Option<Condition>,
    rule: Rule,
    check: fn(),
}

enum Rule {
    // 101.1. Whenever a card’s text directly contradicts these rules, the card takes precedence. The card overrides only the rule that applies to that specific situation. The only exception is that a player can concede the game at any time (see rule 104.3a).
    CARDS_TAKE_PRECEDENCE,
    // 101.2. When a rule or effect allows or directs something to happen, and another effect states that it can’t happen, the “can’t” effect takes precedence.
    // 101.2a. Adding abilities to objects and removing abilities from objects don’t fall under this rule. (See rule 113.10.)
    CANT_TAKES_PRECEDENCE(Condition),
    // 101.3. Any part of an instruction that’s impossible to perform is ignored. (In many cases the card will specify consequences for this; if it doesn’t, there’s no effect.)
    IMPOSSIBLE_ACTIONS_IGNORED,
    // 101.4. If multiple players would make choices and/or take actions at the same time, the active player (the player whose turn it is) makes any choices required, then the next player in turn order (usually the player seated to the active player’s left) makes any choices required, followed by the remaining nonactive players in turn order. Then the actions happen simultaneously. This rule is often referred to as the “Active Player, Nonactive Player (APNAP) order” rule.
    APNAP_CHOICE_ORDER,
    // 103.6. Some cards allow a player to take actions with them from their opening hand. Once the mulligan process (see rule 103.5) is complete, the starting player may take any such actions in any order. Then each other player in turn order may do the same.
    // 103.6a. If a card allows a player to begin the game with that card on the battlefield, the player taking this action puts that card onto the battlefield.
    // 103.6b. If a card allows a player to reveal it from their opening hand, the player taking this action does so. The card remains revealed until the first turn begins. Each card may be revealed this way only once.
    FIRST_ACTION_FROM_OPENING_HAND(Condition),
    // 103.8. The starting player takes their first turn.
    FIRST_TURN,
    // 103.8a. In a two-player game, the player who plays first skips the draw step (see rule 504, “Draw Step”) of their first turn.
    TWO_PLAYER_SKIP_DRAW(Condition),
    // 104.1. A game ends immediately when a player wins, when the game is a draw, or when the game is restarted.
    GAME_ENDS(Condition),
    // 104.2. There are several ways to win the game.
    // 104.2a. A player still in the game wins the game if that player’s opponents have all left the game. This happens immediately and overrides all effects that would preclude that player from winning the game.
    // 104.2b. An effect may state that a player wins the game.
    PLAYER_WINS(Condition),
    // 104.3. There are several ways to lose the game.
    // 104.3a. A player can concede the game at any time. A player who concedes leaves the game immediately. That player loses the game.
    // 104.3b. If a player’s life total is 0 or less, that player loses the game the next time a player would receive priority. (This is a state-based action. See rule 704.)
    // 104.3c. If a player is required to draw more cards than are left in their library, they draw the remaining cards and then lose the game the next time a player would receive priority. (This is a state-based action. See rule 704.)
    // 104.3d. If a player has ten or more poison counters, that player loses the game the next time a player would receive priority. (This is a state-based action. See rule 704.)
    // 104.3e. An effect may state that a player loses the game.
    // 104.3f. If a player would both win and lose the game simultaneously, that player loses the game.
    // 104.3j. In a Commander game, a player who’s been dealt 21 or more combat damage by the same commander over the course of the game loses the game. (This is a state-based action. See rule 704. See also rule 903.10.)
    PLAYER_LOSES(Condition),
    // 104.4. There are several ways for the game to be a draw.
    // 104.4a. If all the players remaining in a game lose simultaneously, the game is a draw.
    // 104.4b. If a game that’s not using the limited range of influence option (including a two-player game) somehow enters a “loop” of mandatory actions, repeating a sequence of events with no way to stop, the game is a draw. Loops that contain an optional action don’t result in a draw.
    // 104.4c. An effect may state that the game is a draw.
    // 104.5. If a player loses the game, that player leaves the game. If the game is a draw for a player, that player leaves the game. The multiplayer rules handle what happens when a player leaves the game; see rule 800.4.
    PLAYER_DRAWS(Condition),
    // 104.6. One card (Karn Liberated) restarts the game. All players still in the game when it restarts then immediately begin a new game. See rule 727, “Restarting the Game.”
    RESTART_GAME(Condition),
    // 105.2. An object can be one or more of the five colors, or it can be no color at all. An object is the color or colors of the mana symbols in its mana cost, regardless of the color of its frame. An object’s color or colors may also be defined by a color indicator or a characteristic-defining ability. See rule 202.2.
    // 105.5. If an effect refers to a color pair, it means exactly two of the five colors. There are ten color pairs: white and blue, white and black, blue and black, blue and red, black and red, black and green, red and green, red and white, green and white, and green and blue.
    OBJECT_HAS_COLORS_OR_NO_COLOR(Condition),
    // 105.3. Effects may change an object’s color or give a color to a colorless object. If an effect gives an object a new color, the new color replaces all previous colors the object had (unless the effect said the object became that color “in addition” to its other colors). Effects may also make a colored object become colorless.
    // 105.4. If a player is asked to choose a color, they must choose one of the five colors. “Multicolored” is not a color. Neither is “colorless.”
    OBJECT_HAS_NEW_COLOR(Condition),
    // 106.3. Mana is produced by the effects of mana abilities (see rule 605). It may also be produced by the effects of spells, as well as by the effects of abilities that aren’t mana abilities. A spell or ability that produces mana instructs a player to add that mana. If mana is produced by a spell, the source of that mana is that spell. If mana is produced by an ability, the source of that mana is the source of that ability (see rule 113.7).
    MANA_IS_ADDED_TO_POOL(Condition),
    // 106.4. When an effect instructs a player to add mana, that mana goes into a player’s mana pool. From there, it can be used to pay costs immediately, or it can stay in the player’s mana pool as unspent mana. Each player’s mana pool empties at the end of each step and phase, and the player is said to lose this mana. Cards with abilities that produce mana or refer to unspent mana have received errata in the Oracle™ card reference to no longer explicitly refer to the mana pool.
    MANA_POOLS_EMPTY_AT_END_OF_STEP_OR_PHASE(Condition),
    // 106.5. If an ability would produce one or more mana of an undefined type, it produces no mana instead.
    UNDEFINED_MANA_PRODUCED_NO_MANA(Condition),
    // 106.6. Some spells or abilities that produce mana restrict how that mana can be spent, have an additional effect that affects the spell or ability that mana is spent on, or create a delayed triggered ability (see rule 603.7a) that triggers when that mana is spent. This doesn’t affect the mana’s type.
    MANA_HAS_RESTRICTION(Condition),
    // 106.6a. Some replacement effects increase the amount of mana produced by a spell or ability. In these cases, any restrictions or additional effects created by the spell or ability will apply to all mana produced. If the spell or ability creates a delayed triggered ability that triggers when the mana is spent, a separate delayed triggered ability is created for each mana produced. If the spell or ability creates a continuous effect or replacement effect if the mana is spent, a separate effect is created once for each mana produced.
    MANA_RESTRICTIONS_APPLY_TO_ADDITIONAL_MANA_EFFECTS(Condition),
    // 106.7. Some abilities produce mana based on the type of mana another permanent or permanents “could produce.” The type of mana a permanent could produce at any time includes any type of mana that an ability of that permanent would produce if the ability were to resolve at that time, taking into account any applicable replacement effects in any possible order. Ignore whether any costs of the ability could or could not be paid. If that permanent wouldn’t produce any mana under these conditions, or no type of mana can be defined this way, there’s no type of mana it could produce.
    COUD_ADD_MANA(Condition),
    // 106.8. If an effect would add mana represented by a hybrid mana symbol to a player’s mana pool, that player chooses one half of that symbol. If a colored half is chosen, one mana of that color is added to that player’s mana pool. If a generic half is chosen, an amount of colorless mana represented by that half’s number is added to that player’s mana pool.
    HYBRID_ADD_MANA_CHOOSE_COLOR(Condition),
    // 106.9. If an effect would add mana represented by a Phyrexian mana symbol to a player’s mana pool, one mana of the color of that symbol is added to that player’s mana pool.
    PHYREXIAN_ADD_MANA_COLOR(Condition),
    // 106.10. If an effect would add mana represented by a generic mana symbol to a player’s mana pool, that much colorless mana is added to that player’s mana pool.
    GENERIC_ADD_MANA_COLOR_IS_COLORLESS(Condition),
    // 106.11. If an effect would add mana represented by one or more snow mana symbols to a player’s mana pool, that much colorless mana is added to that player’s mana pool.
    SNOW_ADD_MANA_IS_COLORLESS(Condition),
    // 106.12. To “tap [a permanent] for mana” is to activate a mana ability of that permanent that includes the {T} symbol in its activation cost. See rule 605, “Mana Abilities.”
    ACTIVATE_MANA_ABILITY_WITH_TAP_SYMBOL(Condition),
    // 106.12a. An ability that triggers whenever a permanent “is tapped for mana” or is tapped for mana of a specified type triggers whenever such a mana ability resolves and produces mana or the specified type of mana.
    MANA_ABILITIES_TRIGGER(Condition),
    // 106.12b. A replacement effect that applies if a permanent “is tapped for mana” or tapped for mana of a specific type and/or amount modifies the mana production event while such an ability is resolving and producing mana or the specified type and/or amount of mana.
    REPLACEMENT_EFFECTS_AFFECT_MANA_ABILITIES(Condition),
    // 106.13. One card (Drain Power) causes one player to lose unspent mana and another to add “the mana lost this way.” (Note that these may be the same player.) This empties the former player’s mana pool and causes the mana emptied this way to be put into the latter player’s mana pool. Which permanents, spells, and/or abilities produced that mana are unchanged, as are any restrictions or additional effects associated with any of that mana.
    DRAIN_POWER_MANA_CONDITIONS_CARRY_OVER(Condition),
    // 107.1. The only numbers the Magic game uses are integers.
    // 107.1a. You can’t choose a fractional number, deal fractional damage, gain fractional life, and so on. If a spell or ability could generate a fractional number, the spell or ability will tell you whether to round up or down.
    // 107.1c. If a rule or ability instructs a player to choose “any number,” that player may choose any positive number or zero.
    // 107.1b. Most of the time, the Magic game uses only positive numbers and zero. You can’t choose a negative number, deal negative damage, gain negative life, and so on. However, it’s possible for a game value, such as a creature’s power, to be less than zero. If a calculation or comparison needs to use a negative value, it does so. If a calculation that would determine the result of an effect yields a negative number, zero is used instead, unless that effect doubles, triples, or sets to a specific value a player’s life total or the power and/or toughness of a creature or creature card.
    POSITIVE_INTEGER_VALUES_ONLY,
    // 107.2. If anything needs to use a number that can’t be determined, either as a result or in a calculation, it uses 0 instead.
    UNDEFINED_NUMBERS_ARE_ZERO,
    // 107.3. Many objects use the letter X as a placeholder for a number that needs to be determined. Some objects have abilities that define the value of X; the rest let their controller choose the value of X.
    // 107.3a. If a spell or activated ability has a mana cost, alternative cost, additional cost, and/or activation cost with an {X}, [-X], or X in it, and the value of X isn’t defined by the text of that spell or ability, the controller of that spell or ability chooses and announces the value of X as part of casting the spell or activating the ability. (See rule 601, “Casting Spells.”) While a spell is on the stack, any X in its mana cost or in any alternative cost or additional cost it has equals the announced value. While an activated ability is on the stack, any X in its activation cost equals the announced value.
    // 107.3b. If a player is casting a spell that has an {X} in its mana cost, the value of X isn’t defined by the text of that spell, and an effect lets that player cast that spell while paying neither its mana cost nor an alternative cost that includes X, then the only legal choice for X is 0. This doesn’t apply to effects that only reduce a cost, even if they reduce it to zero. See rule 601, “Casting Spells.”
    // 107.3c. If a spell or activated ability has an {X}, [-X], or X in its cost and/or its text, and the value of X is defined by the text of that spell or ability, then that’s the value of X while that spell or ability is on the stack. The controller of that spell or ability doesn’t get to choose the value. Note that the value of X may change while that spell or ability is on the stack.
    // 107.3d. If a cost associated with a special action, such as a suspend cost or a morph cost, has an {X} or an X in it, the value of X is chosen by the player taking the special action immediately before they pay that cost.
    // 107.3e. If a spell or ability refers to the {X} or X in the mana cost, alternative cost, additional cost, or activation cost of another object, any X in that spell or ability’s text uses the value of X used by the other object.
    // 107.3f. Sometimes X appears in the text of a spell or ability but not in a mana cost, alternative cost, additional cost, or activation cost. If the value of X isn’t defined, the controller of the spell or ability chooses the value of X at the appropriate time (either as it’s put on the stack or as it resolves).
    // 107.3g. If a card in any zone other than the stack has an {X} in its mana cost, the value of {X} is treated as 0, even if the value of X is defined somewhere within its text.
    // 107.3h. If an effect instructs a player to pay an object’s mana cost that includes {X}, the value of X is treated as 0 unless the object is a spell on the stack. In that case, the value of X is the value chosen or determined for it as the spell was cast.
    // 107.3i. Normally, all instances of X on an object have the same value at any given time.
    // 107.3j. If an object gains an ability, the value of X within that ability is the value defined by that ability, or 0 if that ability doesn’t define a value of X. This is an exception to rule 107.3i. This may occur with ability-adding effects, text-changing effects, or copy effects.
    // 107.3k. If an object’s activated ability has an {X}, [-X], or X in its activation cost, the value of X for that ability is independent of any other values of X chosen for that object or for other instances of abilities of that object. This is an exception to rule 107.3i.
    // 107.3m. If an object’s enters-the-battlefield triggered ability or replacement effect refers to X, and the spell that became that object as it resolved had a value of X chosen for any of its costs, the value of X for that ability is the same as the value of X for that spell, although the value of X for that permanent is 0. This is an exception to rule 107.3i.
    // 107.3n. If a delayed triggered ability created by a resolving spell or ability refers to X, X is not defined in the text of that triggered ability, and the spell or ability that created it had a value of X chosen for any of its costs, the value of X for the triggered ability is the same as the value of X for the spell of ability that created it.
    // 107.3p. Some objects use the letter Y in addition to the letter X. Y follows the same rules as X.
    X_IS_CHOSEN_OR_ZERO(Condition),
    // 107.4b. Numerical symbols (such as {1}) and variable symbols (such as {X}) represent generic mana in costs. Generic mana in costs can be paid with any type of mana. For more information about {X}, see rule 107.3.
    NUMERICAL_AND_VARIABLE_MANA_COST_PAID_WITH_GENERIC_MANA,
    // 107.4c. The colorless mana symbol {C} is used to represent one colorless mana, and also to represent a cost that can be paid only with one colorless mana.
    COLORLESS_MANA_COST_CAN_ONLY_BE_PAID_WITH_COLORLESS,
    // 107.4d. The symbol {0} represents zero mana and is used as a placeholder for a cost that can be paid with no resources. (See rule 118.5.)
    ZERO_MANA_COST,
    // 107.4e. A hybrid mana symbol is also a colored mana symbol, even if one of its components is colorless. Each one represents a cost that can be paid in one of two ways, as represented by the two halves of the symbol. A hybrid symbol such as {W/U} can be paid with either white or blue mana, and a monocolored hybrid symbol such as {2/B} can be paid with either one black mana or two mana of any type. A hybrid mana symbol is all of its component colors.
    HYBRID_MANA_COST_CAN_BE_PAID_WITH_EITHER,
    // 107.4f. Phyrexian mana symbols are colored mana symbols: {W/P} is white, {U/P} is blue, {B/P} is black, {R/P} is red, and {G/P} is green. A Phyrexian mana symbol represents a cost that can be paid either with one mana of its color or by paying 2 life. There are also ten hybrid Phyrexian mana symbols. A hybrid Phyrexian mana symbol represents a cost that can be paid with one mana of either of its component colors or by paying 2 life. A hybrid Phyrexian mana symbol is both of its component colors.
    PHYREXIAN_MANA_COST_CAN_BE_PAID_WITH_LIFE,
    // 107.4h. When used in a cost, the snow mana symbol {S} represents a cost that can be paid with one mana of any type produced by a snow source (see rule 106.3). Effects that reduce the amount of generic mana you pay don’t affect {S} costs. The {S} symbol can also be used to refer to mana of any type produced by a snow source spent to pay a cost. Snow is neither a color nor a type of mana.
    SNOW_MANA_CAN_ONLY_BE_PAID_BY_SNOW_SOURCE,
    // 107.5. The tap symbol is {T}. The tap symbol in an activation cost means “Tap this permanent.” A permanent that’s already tapped can’t be tapped again to pay the cost. A creature’s activated ability with the tap symbol in its activation cost can’t be activated unless the creature has been under its controller’s control continuously since their most recent turn began. See rule 302.6.
    TAP_SYMBOL_ONLY_ACTIVATES_FOR_UNTAPPED_PERMANENTS_AND_NOT_SUMMONING_SICK(Condition),
    // 107.6. The untap symbol is {Q}. The untap symbol in an activation cost means “Untap this permanent.” A permanent that’s already untapped can’t be untapped again to pay the cost. A creature’s activated ability with the untap symbol in its activation cost can’t be activated unless the creature has been under its controller’s control continuously since their most recent turn began. See rule 302.6.
    UNTAP_SYMBOL_ONLY_ACTIVATES_FOR_TAPPED_PERMANENTS_AND_NOT_SUMMONING_SICK(Condition),
    // 107.7. Each activated ability of a planeswalker has a loyalty symbol in its cost. Positive loyalty symbols point upward and feature a plus sign followed by a number. Negative loyalty symbols point downward and feature a minus sign followed by a number or an X. Neutral loyalty symbols don’t point in either direction and feature a 0. [+N] means “Put N loyalty counters on this permanent,” [-N] means “Remove N loyalty counters from this permanent,” and [0] means “Put zero loyalty counters on this permanent.” Loyalty symbols may also appear in abilities that modify loyalty costs.
    PLANESWALKER_LOYALTY_COUNTERS,
    // 107.8. The text box of a leveler card contains two level symbols, each of which is a keyword ability that represents a static ability. The level symbol includes either a range of numbers, indicated here as “N1-N2,” or a single number followed by a plus sign, indicated here as “N3+.” Any abilities printed within the same text box striation as a level symbol are part of its static ability. The same is true of the power/toughness box printed within that striation, indicated here as “[P/T].” See rule 711, “Leveler Cards.”
    // 107.8a. “{LEVEL N1-N2} [Abilities] [P/T]” means “As long as this creature has at least N1 level counters on it, but no more than N2 level counters on it, it has base power and toughness [P/T] and has [abilities].”
    // 107.8b. “{LEVEL N3+} [Abilities] [P/T]” means “As long as this creature has N3 or more level counters on it, it has base power and toughness [P/T] and has [abilities].”
    LEVELER_LEVEL_COUNTERS,
    // 107.14. The energy symbol is {E}. It represents one energy counter. To pay {E}, a player removes one energy counter from themselves.
    ENERGY_COUNTER,
    // 107.15. The text box of a Saga card contains chapter symbols, each of which is a keyword ability that represents a triggered ability. A chapter symbol includes a Roman numeral, indicated here as “rN”. The text printed in the text box striation to the right of a chapter symbol is the effect of the triggered ability it represents. See rule 714, “Saga Cards.”
    // 107.15a. “{rN}—[Effect]” means “When one or more lore counters are put onto this Saga, if the number of lore counters on it was less than N and became at least N, [effect].”
    // 107.15b. “{rN1}, {rN2}—[Effect]” is the same as “{rN1}—[Effect]” and “{rN2}—[Effect].”
    SAGA_LORE_COUNTER,
    // 107.16. The text box of a Class card contains class level bars, each of which is a keyword ability that represents both an activated ability and a static ability. A class level bar includes the activation cost of its activated ability and a level number. Any abilities printed within the same text box section as the class level bar are part of its static ability. See rule 716, “Class Cards.”
    CLASS_LEVELS,
    // 107.17. The ticket symbol is {TK}. It represents one ticket counter.
    // 107.17a. A ticket symbol with a number inside it represents a ticket cost. To pay that cost, a player removes that many ticket counters from themselves.
    TICKET_COUNTER,
    // 107.18. The pawprint symbol is {P}. This symbol is used to indicate the modes on some modal spells, and does not represent a cost, mana, counters, or any type of persistent resource. See rule 700.2i.
    PAWPRINT_SYMBOL,
    // 108.3. The owner of a card in the game is the player who started the game with it in their deck. If a card is brought into the game from outside the game rather than starting in a player’s deck, its owner is the player who brought it into the game. If a card starts the game in the command zone, its owner is the player who put it into the command zone to start the game. Legal ownership of a card in the game is irrelevant to the game rules except for the rules for ante. (See rule 407.)
    OWNER_IS_PLAYER_WHO_BROUGHT_THE_CARD,
    // 108.3b. Some spells and abilities allow a player to take cards they own from outside the game and bring them into the game. (See rule 400.11b.) If a card outside that game is involved in a Magic game, its owner is determined as described in rule 108.3. If a card outside that game is in the sideboard of a Magic game (see rule 100.4), its owner is considered to be the player who started the game with it in their sideboard. In all other cases, the owner of a card outside the game is its legal owner.
    OUTSIDE_THE_GAME_IS_SIDEBOARD(Condition),
    // 108.4. A card doesn’t have a controller unless that card represents a permanent or spell; in those cases, its controller is determined by the rules for permanents or spells. See rules 110.2 and 112.2.
    // 108.4a. If anything asks for the controller of a card that doesn’t have one (because it’s not a permanent or spell), use its owner instead.
    ONLY_PERMANENTS_AND_SPELLS_HAVE_CONTROLLERS_THEN_OWNERS(Condition),
    // 109.1. An object is an ability on the stack, a card, a copy of a card, a token, a spell, a permanent, or an emblem.
    OBJECT_DEFINITION(Condition),
    // 109.2. If a spell or ability uses a description of an object that includes a card type or subtype, but doesn’t refer to a specific zone or include the word “card,” “spell,” “source,” or “scheme,” it means a permanent of that card type or subtype on the battlefield.
    CARD_TYPE_REFERENCE_ON_BATTLEFIELD(Condition),
    // 109.2a. If a spell or ability uses a description of an object that includes the word “card” and the name of a zone, it means a card matching that description in the stated zone.
    CARD_REFERENCED_IN_ZONE,
    // 109.2b. If a spell or ability uses a description of an object that includes the word “spell,” it means a spell matching that description on the stack.
    SPELL_REFERENCE_ON_STACK,
    // 109.2c. If a spell or ability uses a description of an object that includes the word “source,” it means a source matching that description—a source of an ability, of damage, or of mana—in any zone. See rules 113.7 and 609.7.
    SOURCE_REFERENCE,
    // 109.2d. If an ability of a scheme card includes the text “this scheme,” it means the scheme card in the command zone on which that ability is printed.
    SCHEME_REFERENCE,
    // 109.3. An object’s characteristics are name, mana cost, color, color indicator, card type, subtype, supertype, rules text, abilities, power, toughness, loyalty, defense, hand modifier, and life modifier. Objects can have some or all of these characteristics. Any other information about an object isn’t a characteristic. For example, characteristics don’t include whether a permanent is tapped, a spell’s target, an object’s owner or controller, what an Aura enchants, and so on.
    CHARACTERISTIC_REFERENCE,
    // 109.4. Only objects on the stack or on the battlefield have a controller. Objects that are neither on the stack nor on the battlefield aren’t controlled by any player. See rule 108.4. There are six exceptions to this rule:
    // 109.4a. The controller of a mana ability is determined as though it were on the stack. See rule 605, “Mana Abilities.”
    // 109.4b. A triggered ability that has triggered but is waiting to be placed on the stack is controlled by the player who controlled its source at the time it triggered, unless it’s a delayed triggered ability. To determine the controller of a delayed triggered ability, see rules 603.7d–f. See also rule 603, “Handling Triggered Abilities.”
    // 109.4c. An emblem is controlled by the player who puts it into the command zone. See rule 114, “Emblems.”
    CONTROLLER_REFERENCE,
    // 109.5. The words “you” and “your” on an object refer to the object’s controller, its would-be controller (if a player is attempting to play, cast, or activate it), or its owner (if it has no controller). For a static ability, this is the current controller of the object it’s on. For an activated ability, this is the player who activated the ability. For a triggered ability, this is the controller of the object when the ability triggered, unless it’s a delayed triggered ability. To determine the controller of a delayed triggered ability, see rules 603.7d–f.
    YOU_YOUR_REFERENCE,
    // 110.1. A permanent is a card or token on the battlefield. A permanent remains on the battlefield indefinitely. A card or token becomes a permanent as it enters the battlefield and it stops being a permanent as it’s moved to another zone by an effect or rule.
    PERMANENT_REMAINS_ON_BATTLEFIELD,
    // 110.2. A permanent’s owner is the same as the owner of the card that represents it (unless it’s a token; see rule 111.2). A permanent’s controller is, by default, the player under whose control it entered the battlefield. Every permanent has a controller.
    PERMANENT_OWNER_IS_OWNER_CONTROLLER_IS_CONTROLLER,
    // 110.2a. If an effect instructs a player to put an object onto the battlefield, that object enters the battlefield under that player’s control unless the effect states otherwise.
    PLAYER_PUTS_ON_BATTLEFIELD_PUTS_IN_CONTROL,
    // 110.2b. If an effect causes a player to gain control of another player’s permanent spell, the first player controls the permanent that spell becomes, but the permanent’s controller by default is the player who put that spell onto the stack. (This distinction is relevant in multiplayer games; see rule 800.4c.)
    PLAYER_GAINS_CONTROL_OF_PERMANENT_DEFAULTING_TO_PLAYER_WHO_PUT_ON_STACK,
    // 110.3. A nontoken permanent’s characteristics are the same as those printed on its card, as modified by any continuous effects. See rule 613, “Interaction of Continuous Effects.”
    PERMANENT_CHARACTERISTICS_AS_PRINTED_MODIFIED_BY_CONTINUOUS_EFFECTS,
    // 110.4. There are six permanent types: artifact, battle, creature, enchantment, land, and planeswalker. Instant and sorcery cards can’t enter the battlefield and thus can’t be permanents. Some kindred cards can enter the battlefield and some can’t, depending on their other card types. See section 3, “Card Types.”
    // 110.4a. The term “permanent card” is used to refer to a card that could be put onto the battlefield. Specifically, it means an artifact, battle, creature, enchantment, land, or planeswalker card.
    // 110.4b. The term “permanent spell” is used to refer to a spell that will enter the battlefield as a permanent as part of its resolution. Specifically, it means an artifact, battle, creature, enchantment, or planeswalker spell.
    // 110.4c. If a permanent somehow loses all its permanent types, it remains on the battlefield. It’s still a permanent.
    PERMANENT_TYPE_DEFINITION,
    // 110.5. A permanent’s status is its physical state. There are four status categories, each of which has two possible values: tapped/untapped, flipped/unflipped, face up/face down, and phased in/phased out. Each permanent always has one of these values for each of these categories.
    // 110.5a. Status is not a characteristic, though it may affect a permanent’s characteristics.
    // 110.5b. Permanents enter the battlefield untapped, unflipped, face up, and phased in unless a spell or ability says otherwise.
    // 110.5c. A permanent retains its status until a spell, ability, or turn-based action changes it, even if that status is not relevant to it.
    // 110.5d. Only permanents have status. Cards not on the battlefield do not. Although an exiled card may be face down, this has no correlation to the face-down status of a permanent. Similarly, cards not on the battlefield are neither tapped nor untapped, regardless of their physical state.
    STATUS_DEFINITION,
    // 111.1. Some effects put tokens onto the battlefield. A token is a marker used to represent any permanent that isn’t represented by a card.
    TOKEN_DEFINITION,
    // 111.2. The player who creates a token is its owner. The token enters the battlefield under that player’s control.
    TOKEN_OWNER,
    // 111.3. The spell or ability that creates a token may define the values of any number of characteristics for the token. This becomes the token’s “text.” The characteristic values defined this way are functionally equivalent to the characteristic values that are printed on a card; for example, they define the token’s copiable values. A token doesn’t have any characteristics not defined by the spell or ability that created it.
    TOKEN_TEXT_DEFINED_BY_SPELL,
    // 111.4. A spell or ability that creates a token sets both its name and its subtype(s). If the spell or ability doesn’t specify the name of the token, its name is the same as its subtype(s) plus the word “Token.” Once a token is on the battlefield, changing its name doesn’t change its subtype(s), and vice versa.
    TOKEN_NAME_AND_TYPE_SET_BY_SPELL,
    // 111.5. If a spell or ability would create a token, but a rule or effect states that a permanent with one or more of that token’s characteristics can’t enter the battlefield, the token is not created. Similarly, if an effect would create a token that is a copy of an instant or sorcery card, no token is created.
    TOKENS_DONT_ENTER_IF_RULE_OR_EFFECT_PREVENT_IT,
    // 111.6. A token is subject to anything that affects permanents in general or that affects the token’s card type or subtype. A token isn’t a card (even if represented by a card that has a Magic back or that came from a Magic booster pack).
    TOKENS_ARE_PERMANENTS_NOT_CARDS,
    // 111.7. A token that’s in a zone other than the battlefield ceases to exist. This is a state-based action; see rule 704. (Note that if a token changes zones, applicable triggered abilities will trigger before the token ceases to exist.)
    // 111.8. A token that has left the battlefield can’t move to another zone or come back onto the battlefield. If such a token would change zones, it remains in its current zone instead. It ceases to exist the next time state-based actions are checked; see rule 704.
    TOKENS_NOT_IN_BATTLEFIELD_DIE,
    // 111.9. Some effects instruct a player to create a legendary token. These may be written “create [name], a . . .” and list characteristics for the token. This is the same as an instruction to create a token with the listed characteristics that has the given name.
    LEGENDARY_TOKENS_DEFINITION,
    // 111.10. Some effects instruct a player to create a predefined token. These effects use the definition below to determine the characteristics the token is created with. The effect that creates a predefined token may also modify or add to the predefined characteristics.
    // 111.10a. A Treasure token is a colorless Treasure artifact token with “{T}, Sacrifice this token: Add one mana of any color.”
    // 111.10b. A Food token is a colorless Food artifact token with “{2}, {T}, Sacrifice this token: You gain 3 life.”
    // 111.10c. A Gold token is a colorless Gold artifact token with “Sacrifice this token: Add one mana of any color.”
    // 111.10d. A Walker token is a 2/2 black Zombie creature token named Walker.
    // 111.10e. A Shard token is a colorless Shard enchantment token with “{2}, Sacrifice this token: Scry 1, then draw a card.”
    // 111.10f. A Clue token is a colorless Clue artifact token with “{2}, Sacrifice this token: Draw a card.”
    // 111.10g. A Blood token is a colorless Blood artifact token with “{1}, {T}, Discard a card, Sacrifice this token: Draw a card.”
    // 111.10h. A Powerstone token is a colorless Powerstone artifact token with “{T}: Add {C}. This mana can’t be spent to cast a nonartifact spell.”
    // 111.10i. An Incubator token is a double-faced token. Its front face is a colorless Incubator artifact with “{2}: Transform this token.” Its back face is a 0/0 colorless Phyrexian artifact creature named Phyrexian Token.
    // 111.10j. A Cursed Role token is a colorless Aura Role enchantment token named Cursed with enchant creature and “Enchanted creature has base power and toughness 1/1.”
    // 111.10k. A Monster Role token is a colorless Aura Role enchantment token named Monster with enchant creature and “Enchanted creature gets +1/+1 and has trample.”
    // 111.10m. A Royal Role token is a colorless Aura Role enchantment token named Royal with enchant creature and “Enchanted creature gets +1/+1 and has ward {1}.”
    // 111.10n. A Sorcerer Role token is a colorless Aura Role enchantment token named Sorcerer with enchant creature and “Enchanted creature gets +1/+1 and has ‘Whenever this creature attacks, scry 1.’”
    // 111.10p. A Virtuous Role token is a colorless Aura Role enchantment token named Virtuous with enchant creature and “Enchanted creature gets +1/+1 for each enchantment you control.”
    // 111.10q. A Wicked Role token is a colorless Aura Role enchantment token named Wicked with enchant creature, “Enchanted creature gets +1/+1,” and “When this token is put into a graveyard from the battlefield, each opponent loses 1 life.”
    // 111.10r. A Young Hero Role token is a colorless Aura Role enchantment token named Young Hero with enchant creature and “Enchanted creature has ‘Whenever this creature attacks, if its toughness is 3 or less, put a +1/+1 counter on it.’”
    // 111.10s. A Map token is a colorless Map artifact token with “{1}, {T}, Sacrifice this token: Target creature you control explores. Activate only as a sorcery.” See rule 701.44, “Explore.”
    // 111.10t. A Junk token is a colorless Junk artifact token with “{T}, Sacrifice this token: Exile the top card of your library. You may play that card this turn. Activate only as a sorcery.”
    // 111.10u. A Lander token is a colorless Lander artifact token with “{2}, {T}, Sacrifice this token: Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.”
    PREDEFINED_TOKENS,
    // 112.1. A spell is a card on the stack. As the first step of being cast (see rule 601, “Casting Spells”), the card becomes a spell and is moved to the top of the stack from the zone it was in, which is usually its owner’s hand. (See rule 405, “Stack.”) A spell remains on the stack as a spell until it resolves (see rule 608, “Resolving Spells and Abilities”), is countered (see rule 701.6), or otherwise leaves the stack. For more information, see section 6, “Spells, Abilities, and Effects.”
    // 112.1a. A copy of a spell is also a spell, even if it has no card associated with it. See rule 707.10.
    // 112.1b. Some effects allow a player to cast a copy of a card; if the player does, that copy is a spell as well. See rule 707.12.
    SPELL_DEFINITION,
    // 112.2. A spell’s owner is the same as the owner of the card that represents it, unless it’s a copy. In that case, the owner of the spell is the player under whose control it was put on the stack. A spell’s controller is, by default, the player who put it on the stack. Every spell has a controller.
    // 112.2a. Some effects instruct a player to create a copy of a card and say they may cast it. In that case, the owner of that copy is the player who is instructed to create it and given permission to cast it.
    SPELL_OWNER,
    // 112.3. A noncopy spell’s characteristics are the same as those printed on its card, as modified by any continuous effects. See rule 613, “Interaction of Continuous Effects.”
    SPELL_CHARACTERISTICS_MODIFIED_BY_CONTINUOUS_EFFECTS,
    // 112.4. If an effect of a resolving spell or ability changes any characteristics of a permanent spell, the effect continues to apply to the permanent when the spell resolves. See rule 400.7.
    SPELL_CHARACTERISTIC_CHANGES_CONTINUE_TO_RESOLUTION,
    // 113.1. An ability can be one of three things:
    // 113.1a. An ability can be a characteristic an object has that lets it affect the game. An object’s abilities are defined by its rules text or by the effect that created it. Abilities can also be granted to objects by rules or effects. (Effects that grant abilities usually use the words “has,” “have,” “gains,” or “gain.”) Abilities generate effects. (See rule 609, “Effects.”)
    // 113.1b. An ability can be something that a player has that changes how the game affects the player. A player normally has no abilities unless granted to that player by effects.
    // 113.1c. An ability can be an activated or triggered ability on the stack. This kind of ability is an object. (See section 6, “Spells, Abilities, and Effects.”)
    ABILITIES_DEFINITION,
    // 113.2b. An additional cost or alternative cost to cast a card is an ability of the card.
    ABILITIES_INCLUDE_ADDITIONAL_COSTS,
    // 113.2c. An object may have multiple abilities. If the object is represented by a card, then aside from certain defined abilities that may be strung together on a single line (see rule 702, “Keyword Abilities”), each paragraph break in a card’s text marks a separate ability. If the object is not represented by a card, the effect that created it may have given it multiple abilities. An object may also be granted additional abilities by a spell or ability. If an object has multiple instances of the same ability, each instance functions independently. This may or may not produce more effects than a single instance; refer to the specific ability for more information.
    ABILITIES_CAN_BE_STACKED_SEPARATED_BY_PARAGRAPHS,
    // 113.2d. Abilities can generate one-shot effects or continuous effects. Some continuous effects are replacement effects or prevention effects. See rule 609, “Effects.”
    ABILITIES_ARE_ONE_SHOT_OR_CONTINUOUS,
    // 113.3a. Spell abilities are abilities that are followed as instructions while an instant or sorcery spell is resolving. Any text on an instant or sorcery spell is a spell ability unless it’s an activated ability, a triggered ability, or a static ability that fits the criteria described in rule 113.6.
    SPELL_ABILITIES_DEFINITION,
    // 113.3b. Activated abilities have a cost and an effect. They are written as “[Cost]: [Effect.] [Activation instructions (if any).]” A player may activate such an ability whenever they have priority. Doing so puts it on the stack, where it remains until it’s countered, it resolves, or it otherwise leaves the stack. See rule 602, “Activating Activated Abilities.”
    ACTIVATED_ABILITIES_DEFINITION,
    // 113.3c. Triggered abilities have a trigger condition and an effect. They are written as “[Trigger condition], [effect],” and include (and usually begin with) the word “when,” “whenever,” or “at.” Whenever the trigger event occurs, the ability is put on the stack the next time a player would receive priority and stays there until it’s countered, it resolves, or it otherwise leaves the stack. See rule 603, “Handling Triggered Abilities.”
    TRIGGERED_ABILITIES_DEFINITION,
    // 113.3d. Static abilities are written as statements. They’re simply true. Static abilities create continuous effects which are active while the permanent with the ability is on the battlefield and has the ability, or while the object with the ability is in the appropriate zone. See rule 604, “Handling Static Abilities.”
    STATIC_ABILITIES_DEFINITION,
    // 113.4. Some activated abilities and some triggered abilities are mana abilities. Mana abilities follow special rules: They don’t use the stack, and, under certain circumstances, a player can activate mana abilities even if they don’t have priority. See rule 605, “Mana Abilities.”
    MANA_ABILITIES_DEFINITION,
    // 113.5. Some activated abilities are loyalty abilities. Loyalty abilities follow special rules: A player may activate a loyalty ability of a permanent they control any time they have priority and the stack is empty during a main phase of their turn, but only if no player has previously activated a loyalty ability of that permanent that turn. See rule 606, “Loyalty Abilities.”
    LOYALTY_ABILITIES_DEFINITION,
    // 113.6. Abilities of an instant or sorcery spell usually function only while that object is on the stack. Abilities of all other objects usually function only while that object is on the battlefield. The exceptions are as follows:
    INSTANT_SORCERY_ABILITIES_FUNCTION_WHEN_ON_STACK,
    PERMANENT_ABILITIES_ONLY_FUNCTION_WHEN_ON_BATTLEFIELD(Condition),
    // 113.6a. Characteristic-defining abilities function everywhere, even outside the game and before the game begins. (See rule 604.3.)
    CHARACTERISTIC_ABILITIES_ARE_UBIQUITOUS,
    // 113.6b. An ability that states which zones it functions in functions only from those zones.
    ABILITIES_SCOPED_TO_ZONE,
    // 113.6c. An ability that states which zones it doesn’t function in functions everywhere except for the specified zones, even outside the game and before the game begins.
    ABILITIES_NOT_AFFECT_A_ZONE_AFFECTS_EVERYWHERE,
    // 113.6d. An object’s ability that allows a player to pay an alternative cost rather than its mana cost or otherwise modifies what that particular object costs to cast functions on the stack.
    ALTERNATIVE_COST_ABILITY,
    // 113.6e. An object’s ability that restricts or modifies how that particular object can be played or cast functions in any zone from which it could be played or cast and also on the stack. An object’s ability that grants it another ability that restricts or modifies how that particular object can be played or cast functions only on the stack.
    ABILITY_RESTRICTIONS_ON_STACK_AND_ANY_ZONE,
    // 113.6f. An object’s ability that restricts or modifies what zones that particular object can be played or cast from functions everywhere, even outside the game.
    CAST_ABILITIES_ARE_SCOPED_TO_ZONES,
    // 113.6g. An object’s ability that states it can’t be countered or can’t be copied functions on the stack.
    ABILITIES_THAT_PREVENT_COUNTER_OR_COPY_ON_STACK,
    // 113.6h. An object’s ability that modifies how that particular object enters the battlefield functions as that object is entering the battlefield. See rule 614.12.
    ETB_ABILITIES_APPLY_ON_ENTERING,
    // 113.6i. An object’s ability that states counters can’t be put on that object functions as that object is entering the battlefield in addition to functioning while that object is on the battlefield.
    ABILITY_THAT_PREVENTS_COUNTERS_ALSO_APPLIES_ON_ENTERING,
    // 113.6j. An object’s activated ability that has a cost that can’t be paid while the object is on the battlefield functions from any zone in which its cost can be paid.
    ACTIVATED_ABILITY_THAT_CANT_BE_PAID_IN_ALL_ZONES,
    // 113.6k. A trigger condition that can’t trigger from the battlefield functions in all zones it can trigger from. Other trigger conditions of the same triggered ability may function in different zones.
    TRIGGER_CONDITIONS_TRIGGER_FROM_ALL_ZONES_WHERE_IT_IS_LEGAL,
    // 113.6m. An ability whose cost or effect specifies that it moves the object it’s on out of a particular zone functions only in that zone, unless its trigger condition or a previous part of its cost or effect specifies that the object is put into that zone or, if the object is an Aura, that the object it enchants leaves the battlefield. The same is true if the effect of that ability creates a delayed triggered ability whose effect moves the object out of a particular zone.
    ABILITY_THAT_AFFECTS_INSIDE_ZONE_ONLY_WITH_EXCEPTIONS(Condition),
    // 113.6p. Abilities of emblems, plane cards, vanguard cards, scheme cards, and conspiracy cards function in the command zone. See rule 114, “Emblems”; rule 901, “Planechase”; rule 902, “Vanguard”; rule 904, “Archenemy”; and rule 905, “Conspiracy Draft.”
    ABILITIES_THAT_TRIGGER_FROM_COMMAND_ZONE,
    // 113.7. The source of an ability is the object that generated it. The source of an activated ability on the stack is the object whose ability was activated. The source of a triggered ability (other than a delayed triggered ability) on the stack, or one that has triggered and is waiting to be put on the stack, is the object whose ability triggered. To determine the source of a delayed triggered ability, see rules 603.7d–f.
    SOURCE_DEFINITION,
    // 113.7a. Once activated or triggered, an ability exists on the stack independently of its source. Destruction or removal of the source after that time won’t affect the ability. Note that some abilities cause a source to do something (for example, “This creature deals 1 damage to any target”) rather than the ability doing anything directly. In these cases, any activated or triggered ability that references information about the source for use while announcing an activated ability or putting a triggered ability on the stack checks that information when the ability is put onto the stack. Otherwise, it will check that information when it resolves. In both instances, if the source is no longer in the zone it’s expected to be in at that time, its last known information is used. The source can still perform the action even though it no longer exists.
    ABILITIES_ON_STACK_ARE_INDEPENDENT_OF_SOURCE,
    // 113.8. The controller of an activated ability on the stack is the player who activated it. The controller of a triggered ability on the stack (other than a delayed triggered ability) is the player who controlled the ability’s source when it triggered, or, if it had no controller, the player who owned the ability’s source when it triggered. To determine the controller of a delayed triggered ability, see rules 603.7d–f.
    CONTROLLER_OF_ABILITY_ON_STACK_IS_THE_PLAYER_WHO_ACTIVATED,
    // 113.9. Activated and triggered abilities on the stack aren’t spells, and therefore can’t be countered by anything that counters only spells. Activated and triggered abilities on the stack can be countered by effects that specifically counter abilities. Static abilities don’t use the stack and thus can’t be countered at all.
    ABILITIES_CAN_ONLY_BE_COUNTERED_BY_ABILITY_COUNTER_EXCEPT_STATIC_ABILITIES(Condition),
    // 113.10. Effects can add or remove abilities of objects. An effect that adds an ability will state that the object “gains” or “has” that ability, or similar. An effect that removes an ability will state that the object “loses” that ability.
    EFFECTS_CAN_ADD_OR_REMOVE_ABILITIES,
    // 113.10a. An effect that adds an activated ability may include activation instructions for that ability. These instructions become part of the ability that’s added to the object.
    EFFECTS_THAT_ADD_ACTIVATED_ABILITIES_INCLUDE_ACTIVATION_INSTRUCTIONS,
    // 113.10b. Effects that remove an ability remove all instances of it.
    EFFECTS_THAT_REMOVE_ABILITY_REMOVE_ALL_INSTANCES,
    // 113.10c. If two or more effects add and remove the same ability, in general the most recent one prevails. See rule 613 for more information about the interaction of continuous effects.
    MOST_RECENT_EFFECT_ADDED_OR_REMOVED_WINS,
    // 113.11. Effects can stop an object from having a specified ability. These effects say that the object “can’t have” that ability. If the object has that ability, it loses it. It’s also impossible for an effect or keyword counter to add that ability to the object. If a resolving spell or ability creates a continuous effect that would add the specified ability to such an object, that part of that continuous effect does not apply; however, other parts of that continuous effect will still apply, and that resolving spell or ability can still create other continuous effects. Continuous effects created by static abilities that would add the specified ability won’t apply to that object.
    EFFECTS_THAT_SAY_CANT_HAVE_PREVENT_THE_ABILITY_FROM_BEING_ADDED_BUT_LET_OTHERS_GO_THROUGH,
    // 113.12. An effect that sets an object’s characteristic, or simply states a quality of that object, is different from an ability granted by an effect. When an object “gains” or “has” an ability, that ability can be removed by another effect. If an effect defines a characteristic of the object (“[permanent] is [characteristic value]”), it’s not granting an ability. (See rule 604.3.) Similarly, if an effect states a quality of that object (“[creature] can’t be blocked,” for example), it’s neither granting an ability nor setting a characteristic.
    EFFECT_SETTING_CHARACTERISTIC_IS_NOT_AN_ABILITY,

    // 114. EMBLEMS

    // 114.1. Some effects put emblems into the command zone. An emblem is a marker used to represent an object that has one or more abilities, but usually no other characteristics.
    EMBLEMS_EXIST_IN_COMMAND_ZONE,
    // 114.2. An effect that creates an emblem is written “[Player] gets an emblem with [ability].” This means that [player] puts an emblem with [ability] into the command zone. The emblem is both owned and controlled by that player.
    EMBLEM_CREATION_SYNTAX,
    // 114.3. An emblem has no characteristics other than the abilities defined by the effect that created it. In particular, an emblem has no types, no mana cost, and no color. Most emblems also have no name.
    EMBLEM_HAS_NO_CHARACTERISTICS_EXCEPT_ABILITIES,
    // 114.4. Abilities of emblems function in the command zone.
    EMBLEM_ABILITIES_FUNCTION_IN_COMMAND_ZONE,
    // 114.5. An emblem is neither a card nor a permanent. Emblem isn’t a card type.
    EMBLEM_IS_NOT_A_CARD_OR_PERMANENT_OR_TYPE,

    // 115. Targets

    // 115.1. Some spells and abilities require their controller to choose one or more targets for them. The targets are object(s) and/or player(s) the spell or ability will affect. These targets are declared as part of the process of putting the spell or ability on the stack. The targets can’t be changed except by another spell or ability that explicitly says it can do so.
    SPELLS_AND_ABILITIES_MAY_REQUIRE_TARGETS,

    // 115.1a. An instant or sorcery spell is targeted if its spell ability identifies something it will affect by using the phrase “target [something],” where the “something” is a phrase that describes an object and/or player. The target(s) are chosen as the spell is cast; see rule 601.2c. (If an activated or triggered ability of an instant or sorcery uses the word target, that ability is targeted, but the spell is not.)
    INSTANT_SORCERY_TARGETED_IF_PHRASE_USED,

    // 115.1b. Aura spells are always targeted. An Aura’s target is specified by its enchant keyword ability (see rule 702.5, “Enchant”). The target is chosen as the spell is cast; see rule 601.2c. An Aura permanent doesn’t target anything; only the spell is targeted. (An activated or triggered ability of an Aura permanent can also be targeted.)
    AURA_SPELLS_ALWAYS_TARGETED,

    // 115.1c. An activated ability is targeted if it identifies something it will affect by using the phrase “target [something],” where the “something” is a phrase that describes an object and/or player. The target(s) are chosen as the ability is activated; see rule 602.2b.
    ACTIVATED_ABILITY_TARGETED_IF_PHRASE_USED,

    // 115.1d. A triggered ability is targeted if it identifies something it will affect by using the phrase “target [something],” where the “something” is a phrase that describes an object and/or player. The target(s) are chosen as the ability is put on the stack; see rule 603.3d.
    TRIGGERED_ABILITY_TARGETED_IF_PHRASE_USED,

    // 115.1e. Some keyword abilities, such as equip and modular, represent targeted activated or triggered abilities, and some keyword abilities, such as mutate, cause spells to have targets. In those cases, the phrase “target [something]” appears in the rule for that keyword ability rather than in the ability itself. (The keyword’s reminder text will often contain the word “target.”) See rule 702, “Keyword Abilities.”
    KEYWORD_ABILITIES_MAY_REPRESENT_TARGETED_ABILITIES,

    // 115.2. Only permanents are legal targets for spells and abilities, unless a spell or ability (a) specifies that it can target an object in another zone or a player, or (b) targets an object that can’t exist on the battlefield, such as a spell or ability. See also rule 115.4.
    TARGETS_MUST_BE_PERMANENTS_UNLESS_SPECIFIED(Condition),

    // 115.3. The same target can’t be chosen multiple times for any one instance of the word “target” on a spell or ability. If the spell or ability uses the word “target” in multiple places, the same object or player can be chosen once for each instance of the word “target” (as long as it fits the targeting criteria). This rule applies both when choosing targets for a spell or ability and when changing targets or choosing new targets for a spell or ability (see rule 115.7).
    TARGET_CANNOT_BE_CHOSEN_MULTIPLE_TIMES_PER_INSTANCE,

    // 115.4. Some spells and abilities that refer to damage require “any target,” “another target,” “two targets,” or similar rather than “target [something].” These targets may be creatures, players, planeswalkers, or battles. Other game objects, such as noncreature artifacts or spells, can’t be chosen.
    ANY_TARGET_INCLUDES_CREATURES_PLAYERS_PLANESWALKERS_BATTLES,

    // 115.5. A spell or ability on the stack is an illegal target for itself.
    SPELL_OR_ABILITY_ILLEGAL_TARGET_FOR_ITSELF,

    // 115.6. A spell or ability that requires targets may allow zero targets to be chosen. Such a spell or ability is still said to require targets, but that spell or ability is targeted only if one or more targets have been chosen for it.
    SPELL_MAY_ALLOW_ZERO_TARGETS,

    // 115.7a. If an effect allows a player to “change the target(s)” of a spell or ability, each target can be changed only to another legal target. If a target can’t be changed to another legal target, the original target is unchanged, even if the original target is itself illegal by then. If all the targets aren’t changed to other legal targets, none of them are changed.
    CHANGE_TARGETS_MUST_BE_LEGAL,

    // 115.7b. If an effect allows a player to “change a target” of a spell or ability, the process described in rule 115.7a is followed, except that only one of those targets may be changed (rather than all of them or none of them).
    CHANGE_SINGLE_TARGET_MUST_BE_LEGAL,

    // 115.7c. If an effect allows a player to “change any targets” of a spell or ability, the process described in rule 115.7a is followed, except that any number of those targets may be changed (rather than all of them or none of them).
    CHANGE_ANY_TARGETS_MUST_BE_LEGAL,

    // 115.7d. If an effect allows a player to “choose new targets” for a spell or ability, the player may leave any number of the targets unchanged, even if those targets would be illegal. If the player chooses to change some or all of the targets, the new targets must be legal and must not cause any unchanged targets to become illegal.
    CHOOSE_NEW_TARGETS_CAN_LEAVE_ILLEGAL_UNCHANGED,

    // 115.7e. When changing targets or choosing new targets for a spell or ability, only the final set of targets is evaluated to determine whether the change is legal.
    ONLY_FINAL_SET_OF_TARGETS_EVALUATED,

    // 115.7f. A spell or ability may “divide” or “distribute” an effect (such as damage or counters) among one or more targets. When changing targets or choosing new targets for that spell or ability, the original division can’t be changed.
    DIVIDED_EFFECT_DISTRIBUTION_CANNOT_CHANGE,

    // 115.8. Modal spells and abilities may have different targeting requirements for each mode. An effect that allows a player to change the target(s) of a modal spell or ability, or to choose new targets for a modal spell or ability, doesn’t allow that player to change its mode. (See rule 700.2.)
    CHANGING_TARGETS_DOES_NOT_CHANGE_MODE,

    // 115.9a. An object that looks for a “[spell or ability] with [a number of] targets” checks the number of times any object or player was chosen as the target of that spell or ability when it was put on the stack, not the number of its targets that are currently legal. If the same object or player became a target more than once, each of those instances is counted separately.
    CHECK_NUMBER_OF_TARGETS_AT_CAST_TIME,

    // 115.9b. An object that looks for a “[spell or ability] that targets [something]” checks the current state of that spell or ability’s targets. If an object it targets is still in the zone it’s expected to be in or a player it targets is still in the game, that target’s current information is used, even if it’s not currently legal for that spell or ability. If an object it targets is no longer in the zone it’s expected to be in or a player it targets is no longer in the game, that target is ignored; its last known information is not used.
    CHECK_CURRENT_STATE_OF_TARGETS,

    // 115.9c. An object that looks for a “[spell or ability] that targets only [something]” checks the number of different objects or players that were chosen as targets of that spell or ability when it was put on the stack (as modified by effects that changed those targets), not the number of those objects or players that are currently legal targets. If that number is one (even if the spell or ability targets that object or player multiple times), the current state of that spell or ability’s target is checked as described in rule 115.9b.
    CHECK_ONLY_SOMETHING_TARGETS_AT_CAST_TIME,

    // 115.10. Spells and abilities can affect objects and players they don’t target. In general, those objects and players aren’t chosen until the spell or ability resolves. See rule 608, “Resolving Spells and Abilities.”
    SPELLS_CAN_AFFECT_UNTARGETED_OBJECTS,

    // 115.10a. Just because an object or player is being affected by a spell or ability doesn’t make that object or player a target of that spell or ability. Unless that object or player is identified by the word “target” in the text of that spell or ability, or the rule for that keyword ability, it’s not a target.
    AFFECTED_IS_NOT_TARGETED,

    // 115.10b. In particular, the word “you” in an object’s text doesn’t indicate a target.
    YOU_DOES_NOT_INDICATE_TARGET,

    // 116. Special Actions

    // 116.1. Special actions are actions a player may take when they have priority that don’t use the stack. These are not to be confused with turn-based actions and state-based actions, which the game generates automatically. (See rule 703, “Turn-Based Actions,” and rule 704, “State-Based Actions.”)
    SPECIAL_ACTIONS_DO_NOT_USE_STACK,

    // 116.2a. Playing a land is a special action. To play a land, a player puts that land onto the battlefield from the zone it was in (usually that player’s hand). By default, a player can take this action only once during each of their turns. A player can take this action any time they have priority and the stack is empty during a main phase of their turn. See rule 305, “Lands.”
    PLAYING_LAND_IS_SPECIAL_ACTION,

    // 116.2b. Turning a face-down creature face up is a special action. A player can take this action any time they have priority. See rule 708, “Face-Down Spells and Permanents.”
    TURNING_FACE_DOWN_CREATURE_UP_IS_SPECIAL_ACTION,

    // 116.2c. Some effects allow a player to take an action at a later time, usually to end a continuous effect or to stop a delayed triggered ability from triggering. Doing so is a special action. A player can take such an action any time they have priority, unless that effect specifies another timing restriction, for as long as the effect allows it.
    LATER_ACTION_TO_END_EFFECT_IS_SPECIAL_ACTION,

    // 116.2d. Some effects from static abilities allow a player to take an action to ignore the effect from that ability for a duration. Doing so is a special action. A player can take such an action any time they have priority.
    ACTION_TO_IGNORE_STATIC_EFFECT_IS_SPECIAL_ACTION,

    // 116.2e. One card (Circling Vultures) has the ability “You may discard Circling Vultures any time you could cast an instant.” Doing so is a special action. A player can take such an action any time they have priority.
    DISCARD_CIRCLING_VULTURES_IS_SPECIAL_ACTION,

    // 116.2f. A player who has a card with suspend in their hand may exile that card. This is a special action. A player can take this action any time they have priority, but only if they could begin to cast that card by putting it onto the stack. See rule 702.62, “Suspend.”
    EXILING_SUSPEND_CARD_IS_SPECIAL_ACTION,

    // 116.2g. A player who has chosen a companion may pay {3} to put that card from outside the game into their hand. This is a special action. A player can take this action any time they have priority and the stack is empty during a main phase of their turn, but only if they haven’t done so yet this game. (See rule 702.139, “Companion.”)
    BRINGING_COMPANION_TO_HAND_IS_SPECIAL_ACTION,

    // 116.2h. A player who has a card with foretell in their hand may pay {2} and exile that card face down. This is a special action. A player may take this action any time they have priority during their turn. See rule 702.143, “Foretell.”
    EXILING_FORETELL_CARD_IS_SPECIAL_ACTION,

    // 116.2i. In a Planechase game, rolling the planar die is a special action. A player can take this action any time they have priority and the stack is empty during a main phase of their turn. Taking this action costs a player an amount of mana equal to the number of times they have previously taken this action on that turn. Note that this number won’t be equal to the number of times the player has rolled the planar die that turn if an effect has caused the player to roll the planar die that turn. See rule 901, “Planechase.”
    ROLLING_PLANAR_DIE_IS_SPECIAL_ACTION,

    // 116.2j. In a Conspiracy Draft game, turning a face-down conspiracy card in the command zone face up is a special action. A player can take this action any time they have priority. See rule 905.4a.
    TURNING_CONSPIRACY_FACE_UP_IS_SPECIAL_ACTION,

    // 116.2k. A player who has a card with plot in their hand may exile that card. This is a special action. A player can take this action any time they have priority during their own turn while the stack is empty. See rule 702.170, “Plot.”
    EXILING_PLOT_CARD_IS_SPECIAL_ACTION,

    // 116.2m. A player who controls a permanent that has one or more locked halves (see rule 709.5) may pay the mana cost of a locked half of that permanent to give that permanent the appropriate unlocked designation. This cost is referred to as an “unlock cost.” A player can take this action any time they have priority and the stack is empty during a main phase of their turn.
    PAYING_UNLOCK_COST_IS_SPECIAL_ACTION,

    // 116.3. If a player takes a special action, that player receives priority afterward.
    PLAYER_RECEIVES_PRIORITY_AFTER_SPECIAL_ACTION,
    // 117.1. Unless a spell or ability is instructing a player to take an action, which player can take actions at any given time is determined by a system of priority. The player with priority may cast spells, activate abilities, and take special actions.
    PRIORITY_DETERMINES_ACTION_LEGALITY,
    
    // 117.1a. A player may cast an instant spell any time they have priority. A player may cast a noninstant spell during their main phase any time they have priority and the stack is empty.
    CASTING_TIMING_RESTRICTIONS_BASED_ON_CARD_TYPE,
    
    // 117.1b. A player may activate an activated ability any time they have priority.
    ACTIVATED_ABILITY_TIMING,
    
    // 117.1c. A player may take some special actions any time they have priority. A player may take other special actions during their main phase any time they have priority and the stack is empty. See rule 116, “Special Actions.”
    SPECIAL_ACTION_TIMING,
    
    // 117.1d. A player may activate a mana ability whenever they have priority, whenever they are casting a spell or activating an ability that requires a mana payment, or whenever a rule or effect asks for a mana payment (even in the middle of casting or resolving a spell or activating or resolving an ability).
    MANA_ABILITY_TIMING,
    
    // 117.2a. Triggered abilities can trigger at any time, including while a spell is being cast, an ability is being activated, or a spell or ability is resolving. (See rule 603, “Handling Triggered Abilities.”) However, nothing actually happens at the time an ability triggers. Each time a player would receive priority, each ability that has triggered but hasn’t yet been put on the stack is put on the stack. See rule 117.5.
    TRIGGERED_ABILITIES_WAIT_FOR_PRIORITY,
    
    // 117.2b. Static abilities continuously affect the game. Priority doesn’t apply to them. (See rule 604, “Handling Static Abilities,” and rule 611, “Continuous Effects.”)
    STATIC_ABILITIES_IGNORE_PRIORITY,
    
    // 117.2c. Turn-based actions happen automatically when certain steps or phases begin. They’re dealt with before a player would receive priority. See rule 117.3a. Turn-based actions also happen automatically when each step and phase ends; no player receives priority afterward. See rule 703, “Turn-Based Actions.”
    TURN_BASED_ACTIONS_PRECEDE_PRIORITY,
    
    // 117.2d. State-based actions happen automatically when certain conditions are met. See rule 704. They’re dealt with before a player would receive priority. See rule 117.5.
    STATE_BASED_ACTIONS_PRECEDE_PRIORITY,
    
    // 117.2e. Resolving spells and abilities may instruct players to make choices or take actions, or may allow players to activate mana abilities. Even if a player is doing so, no player has priority while a spell or ability is resolving. See rule 608, “Resolving Spells and Abilities.”
    NO_PRIORITY_DURING_RESOLUTION,
    
    // 117.3a. The active player receives priority at the beginning of most steps and phases, after any turn-based actions (such as drawing a card during the draw step; see rule 703) have been dealt with and abilities that trigger at the beginning of that phase or step have been put on the stack. No player receives priority during the untap step. Players usually don’t get priority during the cleanup step (see rule 514.3).
    ACTIVE_PLAYER_RECEIVES_PRIORITY_FIRST_IN_STEP,
    
    // 117.3b. The active player receives priority after a spell or ability (other than a mana ability) resolves.
    ACTIVE_PLAYER_RECEIVES_PRIORITY_AFTER_RESOLUTION,
    
    // 117.3c. If a player has priority when they cast a spell, activate an ability, or take a special action, that player receives priority afterward.
    PLAYER_RETAINS_PRIORITY_AFTER_ACTION,
    
    // 117.3d. If a player has priority and chooses not to take any actions, that player passes. If any mana is in that player’s mana pool, they announce what mana is there. Then the next player in turn order receives priority.
    PASSING_PRIORITY_TO_NEXT_PLAYER,
    
    // 117.4. If all players pass in succession (that is, if all players pass without taking any actions in between passing), the spell or ability on top of the stack resolves or, if the stack is empty, the phase or step ends.
    ALL_PASS_RESOLVES_TOP_OBJECT_OR_ENDS_STEP,
    
    // 117.5. Each time a player would get priority, the game first performs all applicable state-based actions as a single event (see rule 704, “State-Based Actions”), then repeats this process until no state-based actions are performed. Then triggered abilities are put on the stack (see rule 603, “Handling Triggered Abilities”). These steps repeat in order until no further state-based actions are performed and no abilities trigger. Then the player who would have received priority does so.
    SBA_AND_TRIGGERS_CHECKED_BEFORE_PRIORITY_GRANTED,
    
    // 117.7. If a player with priority casts a spell or activates an activated ability while another spell or ability is already on the stack, the new spell or ability has been cast or activated “in response to” the earlier spell or ability. The new spell or ability will resolve first. See rule 608, “Resolving Spells and Abilities.”
    IN_RESPONSE_TO_DEFINITION,
    
    // 118.1. A cost is an action or payment necessary to take another action or to stop another action from taking place. To pay a cost, a player carries out the instructions specified by the spell, ability, or effect that contains that cost.
    COST_DEFINITION,
    
    // 118.2. If a cost includes a mana payment, the player paying the cost has a chance to activate mana abilities. Paying the cost to cast a spell or activate an activated ability follows the steps in rules 601.2f–h.
    OPPORTUNITY_TO_ACTIVATE_MANA_ABILITIES_DURING_PAYMENT,
    
    // 118.3. A player can’t pay a cost without having the necessary resources to pay it fully. For example, a player with only 1 life can’t pay a cost of 2 life, and a permanent that’s already tapped can’t be tapped to pay a cost. See rule 202, “Mana Cost and Color,” and rule 602, “Activating Activated Abilities.”
    CANT_PAY_COST_WITHOUT_FULL_RESOURCES,
    
    // 118.5. Some costs are represented by {0}, or are reduced to {0}. The action necessary for a player to pay such a cost is the player’s acknowledgment that they are paying it. Even though such a cost requires no resources, it’s not automatically paid.
    ZERO_COST_REQUIRES_ACKNOWLEDGMENT,
    
    // 118.6. Some objects have no mana cost. This represents an unpayable cost. An ability can also have an unpayable cost if its cost is based on the mana cost of an object with no mana cost. Attempting to cast a spell or activate an ability that has an unpayable cost is a legal action. However, attempting to pay an unpayable cost is an illegal action.
    NO_MANA_COST_IS_UNPAYABLE,
    
    // 118.6a. If an unpayable cost is increased by an effect or an additional cost is imposed, the cost is still unpayable. If an alternative cost is applied to an unpayable cost, including an effect that allows a player to cast a spell without paying its mana cost, the alternative cost may be paid.
    ALTERNATIVE_COST_CAN_BYPASS_UNPAYABLE_COST,
    
    // 118.7. What a player actually needs to do to pay a cost may be changed or reduced by effects. If the mana component of a cost is reduced to nothing by cost reduction effects, it’s considered to be {0}. Paying a cost changed or reduced by an effect counts as paying the original cost.
    COST_REDUCTION_EFFECTS_APPLY,
    
    // 118.8. Some spells and abilities have additional costs. An additional cost is a cost listed in a spell’s rules text, or applied to a spell or ability from another effect, that its controller must pay at the same time they pay the spell’s mana cost or the ability’s activation cost. Note that some additional costs are listed in keywords; see rule 702.
    ADDITIONAL_COSTS_DEFINITION,
    
    // 118.8a. Any number of additional costs may be applied to a spell as it’s being cast or to an ability as it’s being activated. The controller of the spell or ability announces their intentions to pay any or all of those costs as described in rule 601.2b.
    MULTIPLE_ADDITIONAL_COSTS_ALLOWED,
    
    // 118.8d. Additional costs don’t change a spell’s mana cost, only what its controller has to pay to cast it. Spells and abilities that ask for that spell’s mana cost still see the original value.
    ADDITIONAL_COSTS_DO_NOT_CHANGE_MANA_COST,
    
    // 118.9. Some spells have alternative costs. An alternative cost is a cost listed in a spell’s text, or applied to it from another effect, that its controller may pay rather than paying the spell’s mana cost. Alternative costs are usually phrased, “You may [action] rather than pay [this object’s] mana cost,” or “You may cast [this object] without paying its mana cost.” Note that some alternative costs are listed in keywords; see rule 702.
    ALTERNATIVE_COSTS_DEFINITION,
    
    // 118.9a. Only one alternative cost can be applied to any one spell as it’s being cast. The controller of the spell announces their intentions to pay that cost as described in rule 601.2b.
    ONLY_ONE_ALTERNATIVE_COST_ALLOWED,
    
    // 118.9d. If an alternative cost is being paid to cast a spell, any additional costs, cost increases, and cost reductions that affect that spell are applied to that alternative cost. (See rule 601.2f.)
    MODIFIERS_APPLY_TO_ALTERNATIVE_COST,
    
    // 118.10. Each payment of a cost applies to only one spell, ability, or effect. For example, a player can’t sacrifice just one creature to activate the activated abilities of two permanents that each require sacrificing a creature as a cost. Also, the resolution of a spell or ability doesn’t pay another spell or ability’s cost, even if part of its effect is doing the same thing the other cost asks for.
    COST_PAYMENT_IS_SINGLE_USE,
    
    // 118.12. Some spells, activated abilities, and triggered abilities read, “[Do something]. If [a player] [does, doesn’t, or can’t], [effect].” Or “[A player] may [do something]. If [that player] [does, doesn’t, or can’t], [effect].” The action [do something] is a cost, paid when the spell or ability resolves. The “If [a player] [does, doesn’t, or can’t]” clause checks whether the player chose to pay an optional cost or started to pay a mandatory cost, regardless of what events actually occurred.
    OPTIONAL_ACTION_COSTS_ON_RESOLUTION,
    
    // 119.2. Damage dealt to a player normally causes that player to lose that much life. See rule 120.3.
    DAMAGE_CAUSES_LIFE_LOSS,
    
    // 119.4. If a cost or effect allows a player to pay an amount of life greater than 0, the player may do so only if their life total is greater than or equal to the amount of the payment. If a player pays life, the payment is subtracted from their life total; in other words, the player loses that much life.
    CANT_PAY_LIFE_IF_INSUFFICIENT,
    
    // 119.5. If an effect sets a player’s life total to a specific number, the player gains or loses the necessary amount of life to end up with the new total.
    SETTING_LIFE_TOTAL_CAUSES_GAIN_OR_LOSS,
    
    // 119.6. If a player has 0 or less life, that player loses the game as a state-based action. See rule 704.
    ZERO_LIFE_IS_GAME_LOSS,
    
    // 119.7. If an effect says that a player can’t gain life, that player can’t make an exchange such that the player’s life total would become higher; in that case, the exchange won’t happen. Similarly, if an effect redistributes life totals, a player can’t receive a new life total such that the player’s life total would become higher. In addition, a cost that involves having that player gain life can’t be paid, and a replacement effect that would replace a life gain event affecting that player won’t do anything.
    CANT_GAIN_LIFE_PREVENTS_INCREASES,
    
    // 119.8. If an effect says that a player can’t lose life, that player can’t make an exchange such that the player’s life total would become lower; in that case, the exchange won’t happen. Similarly, if an effect redistributes life totals, a player can’t receive a new life total such that the player’s life total would become lower. In addition, a cost that involves having that player pay life can’t be paid.
    CANT_LOSE_LIFE_PREVENTS_DECREASES,
    
    // 120.1. Objects can deal damage to battles, creatures, planeswalkers, and players. This is generally detrimental to the object or player that receives that damage. An object that deals damage is the source of that damage.
    DAMAGE_RECIPIENTS,
    
    // 120.3. Damage may have one or more of the following results, depending on whether the recipient of the damage is a player or permanent, the characteristics of the damage’s source, and the characteristics of the damage’s recipient (if it’s a permanent).
    DAMAGE_RESULTS_VARY_BY_CHARACTERISTICS,
    
    // 120.4. Damage is processed in a four-part sequence.
    DAMAGE_PROCESSING_SEQUENCE,
    
    // 120.5. Damage dealt to a creature, planeswalker, or battle doesn’t destroy it. Likewise, the source of that damage doesn’t destroy it. Rather, state-based actions may destroy a creature or otherwise put a permanent into its owner’s graveyard, due to the results of the damage dealt to that permanent. See rule 704.
    DAMAGE_DOES_NOT_DIRECTLY_DESTROY,
    
    // 120.6. Damage marked on a creature remains until the cleanup step, even if that permanent stops being a creature. If the total damage marked on a creature is greater than or equal to its toughness, that creature has been dealt lethal damage and is destroyed as a state-based action (see rule 704). All damage marked on a permanent is removed when it regenerates (see rule 701.19, “Regenerate”) and during the cleanup step (see rule 514.2).
    DAMAGE_REMAINS_UNTIL_CLEANUP,
    
    // 120.8. If a source would deal 0 damage, it does not deal damage at all. That means abilities that trigger on damage being dealt won’t trigger. It also means that replacement effects that would increase the damage dealt by that source, or would have that source deal that damage to a different object or player, have no event to replace, so they have no effect.
    ZERO_DAMAGE_IS_NOT_DEALT,
    
    // 121.2. Cards may only be drawn one at a time. If a player is instructed to draw multiple cards, that player performs that many individual card draws.
    DRAW_CARDS_ONE_AT_A_TIME,
    
    // 121.3. If there are no cards in a player’s library and an effect offers that player the choice to draw a card, that player can choose to do so. However, if an effect says that a player can’t draw cards and another effect offers that player the choice to draw a card, that player can’t choose to do so.
    CAN_CHOOSE_TO_DRAW_FROM_EMPTY_LIBRARY,
    
    // 121.4. A player who attempts to draw a card from a library with no cards in it loses the game the next time a player would receive priority. (This is a state-based action. See rule 704.)
    DRAW_FROM_EMPTY_LIBRARY_IS_GAME_LOSS,
    
    // 121.5. If an effect moves cards from a player’s library to that player’s hand without using the word “draw,” the player has not drawn those cards. This makes a difference for abilities that trigger on drawing cards and effects that replace card draws, as well as if the player’s library is empty.
    MOVE_TO_HAND_IS_NOT_A_DRAW,
    
    // 122.1. A counter is a marker placed on an object or player that modifies its characteristics and/or interacts with a rule, ability, or effect. Counters are not objects and have no characteristics. Notably, a counter is not a token, and a token is not a counter. Counters with the same name or description are interchangeable.
    COUNTER_DEFINITION,
    
    // 122.1a. A +X/+Y counter on a creature or on a creature card in a zone other than the battlefield, where X and Y are numbers, adds X to that object’s power and Y to that object’s toughness. Similarly, -X/-Y counters subtract from power and toughness. See rule 613.4c.
    STAT_COUNTERS_MODIFY_PT,
    
    // 122.2. Counters on an object are not retained if that object moves from one zone to another. The counters are not “removed”; they simply cease to exist. See rule 400.7.
    COUNTERS_CEASE_TO_EXIST_ON_ZONE_CHANGE,
    
    // 122.3. If a permanent has both a +1/+1 counter and a -1/-1 counter on it, N +1/+1 and N -1/-1 counters are removed from it as a state-based action, where N is the smaller of the number of +1/+1 and -1/-1 counters on it. See rule 704.
    PLUS_AND_MINUS_COUNTERS_CANCEL,
    
    // 123.1. A sticker is a marker placed on an object that modifies its characteristics and/or interacts with a rule, ability, or effect. Stickers are not objects. Notably, a sticker is not a counter or a token. Changes to an object from stickers are not part of its copiable values. There are four kinds of stickers: name stickers; ability stickers; power and toughness stickers; and art stickers.
    STICKER_DEFINITION,
    
    // 123.5. Stickers on an object are not retained as that object moves to a hidden zone. Stickers are retained as that object moves to a public zone and continue to apply to the new object it becomes in that zone; this is an exception to rule 400.7.
    STICKERS_RETAINED_IN_PUBLIC_ZONES,
}   
