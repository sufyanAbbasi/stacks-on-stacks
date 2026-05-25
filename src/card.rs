enum Color {
    W,
    U,
    B,
    R,
    G,
    C,
}

enum ManaSymbols {
    // Standard mana symbols:
    W,
    U,
    B,
    R,
    G,
    C,
    // The numerical symbols:
    N(u8),
    // The variable symbol:
    X,
    // The hybrid symbols:
    W_U,
    W_B,
    U_B,
    U_R,
    B_R,
    B_G,
    R_G,
    R_W,
    G_W,
    G_U,
    // The monocolored hybrid symbols
    N_W(u8),
    N_U(u8),
    N_B(u8),
    N_R(u8),
    N_G(u8),
    C_W,
    C_U,
    C_B,
    C_R,
    C_G,
    // The Phyrexian mana symbols
    W_P,
    U_P,
    B_P,
    R_P,
    G_P,
    // The hybrid Phyrexian symbols
    W_U_P,
    W_B_P,
    U_B_P,
    U_R_P,
    B_R_P,
    B_G_P,
    R_G_P,
    R_W_P,
    G_W_P,
    G_U_P,
    // The generic Phyrexian
    H,
    // The snow mana symbol
    S,
}

enum PermanentType {
    Artifact,
    Battle,
    Creature,
    Enchantment,
    Land,
    Planeswalker,
}

enum PermanentStatus {
    Tapped(bool),
    Flipped(bool),
    FaceDown(bool),
    PhasedOut(bool),
}

enum Card {
    Land(LandAttributes),
    Planeswalker(PlaneswalkerAttributes),
    Leveler(LevelerAttributes),
    Saga(SagaAttributes),
    Class(ClassAttributes),
}

struct CardAttributes {}

struct PermanentAttributes {
    types: Vec<PermanentType>,
    status: (
        PermanentStatus::Tapped,
        PermanentStatus::Flipped,
        PermanentStatus::FaceDown,
        PermanentStatus::PhasedOut,
    ),
}

struct SpellAttributes {
    color: Vec<Color>,
    cost: Vec<ManaSymbols>,
}

struct LandAttributes {
    card: CardAttributes,
    permanent: PermanentAttributes,
}

struct PlaneswalkerAttributes {
    card: CardAttributes,
    permanent: PermanentAttributes,
    loyalty_counters: u32,
}

struct LevelerAttributes {
    card: CardAttributes,
    permanent: PermanentAttributes,
    spell: SpellAttributes,
    level_counters: u32,
}

struct SagaAttributes {
    card: CardAttributes,
    permanent: PermanentAttributes,
    spell: SpellAttributes,
    lore_counters: u32,
}

struct ClassAttributes {
    card: CardAttributes,
    permanent: PermanentAttributes,
    spell: SpellAttributes,
    class_level: u32,
}
