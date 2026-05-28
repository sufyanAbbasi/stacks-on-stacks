use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    W,
    U,
    B,
    R,
    G,
    C,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManaSymbols {
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
    WU,
    WB,
    UB,
    UR,
    BR,
    BG,
    RG,
    RW,
    GW,
    GU,
    // The monocolored hybrid symbols
    NW(u8),
    NU(u8),
    NB(u8),
    NR(u8),
    NG(u8),
    CW,
    CU,
    CB,
    CR,
    CG,
    // The Phyrexian mana symbols
    WP,
    UP,
    BP,
    RP,
    GP,
    // The hybrid Phyrexian symbols
    WUP,
    WBP,
    UBP,
    URP,
    BRP,
    BGP,
    RGP,
    RWP,
    GWP,
    GUP,
    // The generic Phyrexian
    H,
    // The snow mana symbol
    S,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardType {
    Artifact,
    Battle,
    Conspiracy,
    Creature,
    Dungeon,
    Enchantment,
    Instant,
    Kindred,
    Land,
    Phenomenon,
    Plane,
    Planeswalker,
    Scheme,
    Sorcery,
    Vanguard,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PermanentType {
    Artifact,
    Battle,
    Creature,
    Enchantment,
    Land,
    Planeswalker,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PermanentStatus {
    pub tapped: bool,
    pub flipped: bool,
    pub face_down: bool,
    pub phased_out: bool,
}

// ==========================================
// 205.4. SUPERTYPES
// ==========================================
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Supertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
}

// ==========================================
// 205.3. SUBTYPES
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactType {
    Attraction,
    Blood,
    Bobblehead,
    Book,
    Clue,
    Contraption,
    Equipment,
    Food,
    Fortification,
    Gold,
    Incubator,
    Infinity,
    Junk,
    Lander,
    Map,
    Mutagen,
    Powerstone,
    Spacecraft,
    Stone,
    Treasure,
    Vehicle,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnchantmentType {
    Aura,
    Background,
    Cartouche,
    Case,
    Class,
    Curse,
    Role,
    Room,
    Rune,
    Saga,
    Shard,
    Shrine,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LandType {
    Cave,
    Desert,
    Forest,
    Gate,
    Island,
    Lair,
    Locus,
    Mine,
    Mountain,
    Plains,
    Planet,
    PowerPlant, // "Power-Plant"
    Sphere,
    Swamp,
    Tower,
    Town,
    Urzas, // "Urza's"
}

impl LandType {
    pub fn is_basic(&self) -> bool {
        matches!(
            self,
            LandType::Forest
                | LandType::Island
                | LandType::Mountain
                | LandType::Plains
                | LandType::Swamp
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaneswalkerType {
    Ajani,
    Aminatou,
    Angrath,
    Arlinn,
    Ashiok,
    Bahamut,
    Basri,
    Bolas,
    Calix,
    Chandra,
    Comet,
    Dack,
    Dakkon,
    Daretti,
    Davriel,
    Dellian,
    Dihada,
    Domri,
    Dovin,
    Ellywick,
    Elminster,
    Elspeth,
    Estrid,
    Freyalise,
    Garruk,
    Gideon,
    Grist,
    Guff,
    Huatli,
    Jace,
    Jared,
    Jaya,
    Jeska,
    Kaito,
    Karn,
    Kasmina,
    Kaya,
    Kiora,
    Koth,
    Liliana,
    Lolth,
    Lukka,
    Minsc,
    Mordenkainen,
    Nahiri,
    Narset,
    Niko,
    Nissa,
    Nixilis,
    Oko,
    Quintorius,
    Ral,
    Rowan,
    Saheeli,
    Samut,
    Sarkhan,
    Serra,
    Sivitri,
    Sorin,
    Szat,
    Tamiyo,
    Tasha,
    Teferi,
    Teyo,
    Tezzeret,
    Tibalt,
    Tyvar,
    Ugin,
    Urza,
    Venser,
    Vivien,
    Vraska,
    Vronos,
    Will,
    Windgrace,
    Wrenn,
    Xenagos,
    Yanggu,
    Yanling,
    Zariel,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpellType {
    Adventure,
    Arcane,
    Lesson,
    Omen,
    Trap,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CreatureType {
    Advisor,
    Aetherborn,
    Alien,
    Ally,
    Angel,
    Antelope,
    Ape,
    Archer,
    Archon,
    Armadillo,
    Army,
    Artificer,
    Assassin,
    AssemblyWorker, // "Assembly-Worker"
    Astartes,
    Atog,
    Aurochs,
    Avatar,
    Azra,
    Badger,
    Balloon,
    Barbarian,
    Bard,
    Basilisk,
    Bat,
    Bear,
    Beast,
    Beaver,
    Beeble,
    Beholder,
    Berserker,
    Bird,
    Bison,
    Blinkmoth,
    Boar,
    Bringer,
    Brushwagg,
    Camarid,
    Camel,
    Capybara,
    Caribou,
    Carrier,
    Cat,
    Centaur,
    Child,
    Chimera,
    Citizen,
    Cleric,
    Clown,
    Cockatrice,
    Construct,
    Coward,
    Coyote,
    Crab,
    Crocodile,
    Ctan, // "C'tan"
    Custodes,
    Cyberman,
    Cyclops,
    Dalek,
    Dauthi,
    Demigod,
    Demon,
    Deserter,
    Detective,
    Devil,
    Dinosaur,
    Djinn,
    Doctor,
    Dog,
    Dragon,
    Drake,
    Dreadnought,
    Drix,
    Drone,
    Druid,
    Dryad,
    Dwarf,
    Echidna,
    Efreet,
    Egg,
    Elder,
    Eldrazi,
    Elemental,
    Elephant,
    Elf,
    Elk,
    Employee,
    Eye,
    Faerie,
    Ferret,
    Fish,
    Flagbearer,
    Fox,
    Fractal,
    Frog,
    Fungus,
    Gamer,
    Gargoyle,
    Germ,
    Giant,
    Giraffe,
    Gith,
    Glimmer,
    Gnoll,
    Gnome,
    Goat,
    Goblin,
    God,
    Golem,
    Gorgon,
    Graveborn,
    Gremlin,
    Griffin,
    Guest,
    Hag,
    Halfling,
    Hamster,
    Harpy,
    Hedgehog,
    Hellion,
    Hero,
    Hippo,
    Hippogriff,
    Homarid,
    Homunculus,
    Horror,
    Horse,
    Human,
    Hydra,
    Hyena,
    Illusion,
    Imp,
    Incarnation,
    Inkling,
    Inquisitor,
    Insect,
    Jackal,
    Jellyfish,
    Juggernaut,
    Kangaroo,
    Kavu,
    Kirin,
    Kithkin,
    Knight,
    Kobold,
    Kor,
    Kraken,
    Llama,
    Lamia,
    Lammasu,
    Leech,
    Lemur,
    Leviathan,
    Lhurgoyf,
    Licid,
    Lizard,
    Lobster,
    Manticore,
    Masticore,
    Mercenary,
    Merfolk,
    Metathran,
    Minion,
    Minotaur,
    Mite,
    Mole,
    Monger,
    Mongoose,
    Monk,
    Monkey,
    Moogle,
    Moonfolk,
    Mount,
    Mouse,
    Mutant,
    Myr,
    Mystic,
    Nautilus,
    Necron,
    Nephilim,
    Nightmare,
    Nightstalker,
    Ninja,
    Noble,
    Noggle,
    Nomad,
    Nymph,
    Octopus,
    Ogre,
    Ooze,
    Orb,
    Orc,
    Orgg,
    Otter,
    Ouphe,
    Ox,
    Oyster,
    Pangolin,
    Peasant,
    Pegasus,
    Pentavite,
    Performer,
    Pest,
    Phelddagrif,
    Phoenix,
    Phyrexian,
    Pilot,
    Pincher,
    Pirate,
    Plant,
    Platypus,
    Porcupine,
    Possum,
    Praetor,
    Primarch,
    Prism,
    Processor,
    Qu,
    Rabbit,
    Raccoon,
    Ranger,
    Rat,
    Rebel,
    Reflection,
    Rhino,
    Rigger,
    Robot,
    Rogue,
    Sable,
    Salamander,
    Samurai,
    Sand,
    Saproling,
    Satyr,
    Scarecrow,
    Scientist,
    Scion,
    Scorpion,
    Scout,
    Sculpture,
    Seal,
    Serf,
    Serpent,
    Servo,
    Shade,
    Shaman,
    Shapeshifter,
    Shark,
    Sheep,
    Siren,
    Skeleton,
    Skunk,
    Slith,
    Sliver,
    Sloth,
    Slug,
    Snail,
    Snake,
    Soldier,
    Soltari,
    Sorcerer,
    Spawn,
    Specter,
    Spellshaper,
    Sphinx,
    Spider,
    Spike,
    Spirit,
    Splinter,
    Sponge,
    Squid,
    Squirrel,
    Starfish,
    Surrakar,
    Survivor,
    Symbiote,
    Synth,
    Tentacle,
    Tetravite,
    Thalakos,
    Thopter,
    Thrull,
    Tiefling,
    TimeLord, // "Time Lord"
    Toy,
    Treefolk,
    Trilobite,
    Triskelavite,
    Troll,
    Turtle,
    Tyranid,
    Unicorn,
    Utrom,
    Vampire,
    Varmint,
    Vedalken,
    Villain,
    Volver,
    Wall,
    Walrus,
    Warlock,
    Warrior,
    Weasel,
    Weird,
    Werewolf,
    Whale,
    Wizard,
    Wolf,
    Wolverine,
    Wombat,
    Worm,
    Wraith,
    Wurm,
    Yeti,
    Zombie,
    Zubera,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlanarType {
    TheAbyss,
    Alara,
    AlfavaMetraxis,
    Amonkhet,
    AndrozaniMinor,
    Antausia,
    Apalapucia,
    Arcavios,
    Arkhos,
    Avishkar,
    Azgol,
    Belenon,
    BolasMeditationRealm,
    Capenna,
    Cridhe,
    TheDalekAsylum,
    Darillium,
    Dominaria,
    Earth,
    Echoir,
    Eldraine,
    Equilor,
    Ergamon,
    Fabacin,
    Fiora,
    Gallifrey,
    Gargantikar,
    Gobakhan,
    HorseheadNebula,
    Ikoria,
    Innistrad,
    Iquatana,
    Ir,
    Ixalan,
    Kaldheim,
    Kamigawa,
    Kandoka,
    Karsus,
    Kephalai,
    Kinshala,
    Kolbahan,
    Kylem,
    Kyneth,
    TheLibrary,
    Lorwyn,
    Luvion,
    Mars,
    Mercadia,
    Mirrodin,
    Moag,
    Mongseng,
    Moon,
    Muraganda,
    Necros,
    NewEarth,
    NewPhyrexia,
    OutsideMuttersSpiral,
    Phyrexia,
    Pyrulea,
    Rabiah,
    Rath,
    Ravnica,
    Regatha,
    Segovia,
    SerrasRealm,
    Shadowmoor,
    Shandalar,
    Shenmeng,
    Skaro,
    Spacecraft,
    Tarkir,
    Theros,
    Time,
    Trenzalore,
    Ulgrotha,
    UnknownPlanet,
    Valla,
    Vryn,
    Wildfire,
    Xerex,
    Zendikar,
    Zhalfir,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DungeonType {
    Undercity,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BattleType {
    Siege,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subtype {
    Artifact(ArtifactType),
    Enchantment(EnchantmentType),
    Land(LandType),
    Planeswalker(PlaneswalkerType),
    Spell(SpellType),
    Creature(CreatureType),
    Planar(PlanarType),
    Dungeon(DungeonType),
    Battle(BattleType),
}

// ==========================================
// CARD & PERMANENT ATTRIBUTES
// ==========================================

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CardAttributes {
    pub name: String,
    pub types: Vec<CardType>,
    pub supertypes: Vec<Supertype>,
    pub subtypes: Vec<Subtype>,
    pub rules_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PermanentAttributes {
    pub types: Vec<PermanentType>,
    pub status: PermanentStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SpellAttributes {
    pub color: Vec<Color>,
    pub cost: Vec<ManaSymbols>,
    pub cmc: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LandAttributes {
    pub card: CardAttributes,
    pub permanent: PermanentAttributes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PlaneswalkerAttributes {
    pub card: CardAttributes,
    pub permanent: PermanentAttributes,
    pub loyalty_counters: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LevelerAttributes {
    pub card: CardAttributes,
    pub permanent: PermanentAttributes,
    pub spell: SpellAttributes,
    pub level_counters: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SagaAttributes {
    pub card: CardAttributes,
    pub permanent: PermanentAttributes,
    pub spell: SpellAttributes,
    pub lore_counters: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ClassAttributes {
    pub card: CardAttributes,
    pub permanent: PermanentAttributes,
    pub spell: SpellAttributes,
    pub class_level: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreatureAttributes {
    pub card: CardAttributes,
    pub permanent: PermanentAttributes,
    pub spell: SpellAttributes,
    pub power: String,
    pub toughness: String,
    pub damage_marked: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ArtifactAttributes {
    pub card: CardAttributes,
    pub permanent: PermanentAttributes,
    pub spell: SpellAttributes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct EnchantmentAttributes {
    pub card: CardAttributes,
    pub permanent: PermanentAttributes,
    pub spell: SpellAttributes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BattleAttributes {
    pub card: CardAttributes,
    pub permanent: PermanentAttributes,
    pub spell: SpellAttributes,
    pub defense: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InstantAttributes {
    pub card: CardAttributes,
    pub spell: SpellAttributes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SorceryAttributes {
    pub card: CardAttributes,
    pub spell: SpellAttributes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KindredAttributes {
    pub card: CardAttributes,
    pub spell: SpellAttributes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Card {
    Artifact(ArtifactAttributes),
    Battle(BattleAttributes),
    Creature(CreatureAttributes),
    Enchantment(EnchantmentAttributes),
    Instant(InstantAttributes),
    Land(LandAttributes),
    Planeswalker(PlaneswalkerAttributes),
    Sorcery(SorceryAttributes),
    Kindred(KindredAttributes),
    Leveler(LevelerAttributes),
    Saga(SagaAttributes),
    Class(ClassAttributes),
}

/// --- RULE 111.10: PREDEFINED TOKENS ---
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredefinedToken {
    Treasure,
    Food,
    Gold,
    Walker,
    Shard,
    Clue,
    Blood,
    Powerstone,
    Incubator,
    CursedRole,
    MonsterRole,
    RoyalRole,
    SorcererRole,
    VirtuousRole,
    WickedRole,
    YoungHeroRole,
    Map,
    Junk,
    Lander,
    Mutagen,
}

impl PredefinedToken {
    /// Compiles a predefined token into its standard characteristics as defined in Rule 111.10a-v.
    pub fn get_card_characteristics(self) -> Card {
        match self {
            PredefinedToken::Treasure => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Treasure".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Treasure)],
                    rules_text: "{T}, Sacrifice this token: Add one mana of any color.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Food => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Food".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Food)],
                    rules_text: "{2}, {T}, Sacrifice this token: You gain 3 life.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Gold => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Gold".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Gold)],
                    rules_text: "Sacrifice this token: Add one mana of any color.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Clue => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Clue".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Clue)],
                    rules_text: "{2}, Sacrifice this token: Draw a card.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Blood => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Blood".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Blood)],
                    rules_text: "{1}, {T}, Discard a card, Sacrifice this token: Draw a card.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Powerstone => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Powerstone".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Powerstone)],
                    rules_text: "{T}: Add {C}. This mana can't be spent to cast a nonartifact spell.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Map => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Map".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Map)],
                    rules_text: "{1}, {T}, Sacrifice this token: Target creature you control explores. Activate only as a sorcery.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Junk => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Junk".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Junk)],
                    rules_text: "{T}, Sacrifice this token: Exile the top card of your library. You may play that card this turn. Activate only as a sorcery.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Lander => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Lander".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Lander)],
                    rules_text: "{2}, {T}, Sacrifice this token: Search your library for a basic land card, put it onto the battlefield tapped, then shuffle.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Mutagen => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Mutagen".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Mutagen)],
                    rules_text: "{1}, {T}, Sacrifice this token: Put a +1/+1 counter on target creature. Activate only as a sorcery.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Walker => Card::Creature(CreatureAttributes {
                card: CardAttributes {
                    name: "Walker".to_string(),
                    types: vec![CardType::Creature],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Creature(CreatureType::Zombie)],
                    rules_text: "".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Creature],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::B],
                    cost: vec![],
                    cmc: 0,
                },
                power: "2".to_string(),
                toughness: "2".to_string(),
                damage_marked: 0,
            }),
            PredefinedToken::Shard => Card::Enchantment(EnchantmentAttributes {
                card: CardAttributes {
                    name: "Shard".to_string(),
                    types: vec![CardType::Enchantment],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Enchantment(EnchantmentType::Shard)],
                    rules_text: "{2}, Sacrifice this token: Scry 1, then draw a card.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Enchantment],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::Incubator => Card::Artifact(ArtifactAttributes {
                card: CardAttributes {
                    name: "Incubator".to_string(),
                    types: vec![CardType::Artifact],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Artifact(ArtifactType::Incubator)],
                    rules_text: "{2}: Transform this token.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Artifact],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::CursedRole => Card::Enchantment(EnchantmentAttributes {
                card: CardAttributes {
                    name: "Cursed".to_string(),
                    types: vec![CardType::Enchantment],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Enchantment(EnchantmentType::Role)],
                    rules_text: "Enchant creature. Enchanted creature has base power and toughness 1/1.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Enchantment],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::MonsterRole => Card::Enchantment(EnchantmentAttributes {
                card: CardAttributes {
                    name: "Monster".to_string(),
                    types: vec![CardType::Enchantment],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Enchantment(EnchantmentType::Role)],
                    rules_text: "Enchant creature. Enchanted creature gets +1/+1 and has trample.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Enchantment],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::RoyalRole => Card::Enchantment(EnchantmentAttributes {
                card: CardAttributes {
                    name: "Royal".to_string(),
                    types: vec![CardType::Enchantment],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Enchantment(EnchantmentType::Role)],
                    rules_text: "Enchant creature. Enchanted creature gets +1/+1 and has ward {1}.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Enchantment],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::SorcererRole => Card::Enchantment(EnchantmentAttributes {
                card: CardAttributes {
                    name: "Sorcerer".to_string(),
                    types: vec![CardType::Enchantment],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Enchantment(EnchantmentType::Role)],
                    rules_text: "Enchant creature. Enchanted creature gets +1/+1 and has 'Whenever this creature attacks, scry 1.'".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Enchantment],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::VirtuousRole => Card::Enchantment(EnchantmentAttributes {
                card: CardAttributes {
                    name: "Virtuous".to_string(),
                    types: vec![CardType::Enchantment],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Enchantment(EnchantmentType::Role)],
                    rules_text: "Enchant creature. Enchanted creature gets +1/+1 for each enchantment you control.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Enchantment],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::WickedRole => Card::Enchantment(EnchantmentAttributes {
                card: CardAttributes {
                    name: "Wicked".to_string(),
                    types: vec![CardType::Enchantment],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Enchantment(EnchantmentType::Role)],
                    rules_text: "Enchant creature. Enchanted creature gets +1/+1. When this token is put into a graveyard from the battlefield, each opponent loses 1 life.".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Enchantment],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
            PredefinedToken::YoungHeroRole => Card::Enchantment(EnchantmentAttributes {
                card: CardAttributes {
                    name: "Young Hero".to_string(),
                    types: vec![CardType::Enchantment],
                    supertypes: vec![],
                    subtypes: vec![Subtype::Enchantment(EnchantmentType::Role)],
                    rules_text: "Enchant creature. Enchanted creature has 'Whenever this creature attacks, if its toughness is 3 or less, put a +1/+1 counter on it.'".to_string(),
                },
                permanent: PermanentAttributes {
                    types: vec![PermanentType::Enchantment],
                    status: PermanentStatus::default(),
                },
                spell: SpellAttributes {
                    color: vec![Color::C],
                    cost: vec![],
                    cmc: 0,
                },
            }),
        }
    }
}

impl Card {
    pub fn name(&self) -> &str {
        match self {
            Card::Artifact(attrs) => &attrs.card.name,
            Card::Battle(attrs) => &attrs.card.name,
            Card::Creature(attrs) => &attrs.card.name,
            Card::Enchantment(attrs) => &attrs.card.name,
            Card::Instant(attrs) => &attrs.card.name,
            Card::Land(attrs) => &attrs.card.name,
            Card::Planeswalker(attrs) => &attrs.card.name,
            Card::Sorcery(attrs) => &attrs.card.name,
            Card::Kindred(attrs) => &attrs.card.name,
            Card::Leveler(attrs) => &attrs.card.name,
            Card::Saga(attrs) => &attrs.card.name,
            Card::Class(attrs) => &attrs.card.name,
        }
    }

    pub fn get_spell_cost(&self) -> Option<&[ManaSymbols]> {
        match self {
            Card::Artifact(attrs) => Some(&attrs.spell.cost),
            Card::Battle(attrs) => Some(&attrs.spell.cost),
            Card::Creature(attrs) => Some(&attrs.spell.cost),
            Card::Enchantment(attrs) => Some(&attrs.spell.cost),
            Card::Instant(attrs) => Some(&attrs.spell.cost),
            Card::Land(_) => None,
            Card::Planeswalker(_) => None,
            Card::Sorcery(attrs) => Some(&attrs.spell.cost),
            Card::Kindred(attrs) => Some(&attrs.spell.cost),
            Card::Leveler(attrs) => Some(&attrs.spell.cost),
            Card::Saga(attrs) => Some(&attrs.spell.cost),
            Card::Class(attrs) => Some(&attrs.spell.cost),
        }
    }
}

