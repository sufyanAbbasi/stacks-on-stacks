use serde::{Serialize, Deserialize};
use crate::effects::PlayerId;

/// Represents the major phases of a turn as defined in Section 500.1.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Phase {
    Beginning,
    PrecombatMain,
    Combat,
    PostcombatMain,
    Ending,
}

/// Represents the individual steps within phases (Section 500.1).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Step {
    // Beginning Phase Steps
    Untap,
    Upkeep,
    Draw,

    // Precombat Main Phase has no sub-steps
    PrecombatMain,

    // Combat Phase Steps
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,

    // Postcombat Main Phase has no sub-steps
    PostcombatMain,

    // Ending Phase Steps
    End,
    Cleanup,
}

/// Represents an automatic, turn-based action that does not use the stack (Section 703).
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnBasedAction {
    /// Active player untaps all permanents they control (Rule 502.2 / 703.4a).
    UntapPermanents,
    /// Active player draws a card from their library (Rule 504.1 / 703.4c).
    DrawCard,
    /// Active player declares attackers (Rule 508.1 / 703.4f).
    DeclareAttackers,
    /// Defending player declares blockers (Rule 509.1 / 703.4g).
    DeclareBlockers,
    /// Active and defending players assign and deal combat damage (Rule 510.1 / 703.4h).
    DealCombatDamage,
    /// Active player discards down to their maximum hand size (Rule 514.1 / 703.4i).
    DiscardToMaxHandSize,
    /// Damage is cleared from permanents and "until end of turn" effects expire (Rule 514.2).
    CleanupDamageAndEffects,
}

/// Models the state of a single turn in the simulation (Section 500).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TurnState {
    pub turn_number: u32,
    pub active_player: PlayerId,
    pub current_phase: Phase,
    pub current_step: Step,
}

impl TurnState {
    /// Initializes a new turn for a given player.
    pub fn new(turn_number: u32, active_player: PlayerId) -> Self {
        Self {
            turn_number,
            active_player,
            current_phase: Phase::Beginning,
            current_step: Step::Untap,
        }
    }

    /// Advances to the next phase/step in standard turn structure.
    /// Returns the transition result: (next_state, list of automatic turn-based actions).
    pub fn advance(&self) -> (Self, Vec<TurnBasedAction>) {
        let mut next_state = self.clone();
        let mut actions = Vec::new();

        match self.current_step {
            Step::Untap => {
                next_state.current_step = Step::Upkeep;
            }
            Step::Upkeep => {
                next_state.current_step = Step::Draw;
                actions.push(TurnBasedAction::DrawCard);
            }
            Step::Draw => {
                next_state.current_phase = Phase::PrecombatMain;
                next_state.current_step = Step::PrecombatMain;
            }
            Step::PrecombatMain => {
                next_state.current_phase = Phase::Combat;
                next_state.current_step = Step::BeginningOfCombat;
            }
            Step::BeginningOfCombat => {
                next_state.current_step = Step::DeclareAttackers;
                actions.push(TurnBasedAction::DeclareAttackers);
            }
            Step::DeclareAttackers => {
                next_state.current_step = Step::DeclareBlockers;
                actions.push(TurnBasedAction::DeclareBlockers);
            }
            Step::DeclareBlockers => {
                next_state.current_step = Step::CombatDamage;
                actions.push(TurnBasedAction::DealCombatDamage);
            }
            Step::CombatDamage => {
                next_state.current_step = Step::EndOfCombat;
            }
            Step::EndOfCombat => {
                next_state.current_phase = Phase::PostcombatMain;
                next_state.current_step = Step::PostcombatMain;
            }
            Step::PostcombatMain => {
                next_state.current_phase = Phase::Ending;
                next_state.current_step = Step::End;
            }
            Step::End => {
                next_state.current_step = Step::Cleanup;
                actions.push(TurnBasedAction::DiscardToMaxHandSize);
                actions.push(TurnBasedAction::CleanupDamageAndEffects);
            }
            Step::Cleanup => {
                // Turn is complete! The simulation would start a new turn for the next player here.
                // For safety in single-turn modeling, we stay in cleanup or let the simulator increment turn.
            }
        }

        (next_state, actions)
    }

    /// Returns the automatic actions that must happen immediately when entering this step
    /// at the start of a turn (e.g. entering the Untap step).
    pub fn get_entry_actions(&self) -> Vec<TurnBasedAction> {
        match self.current_step {
            Step::Untap => vec![TurnBasedAction::UntapPermanents],
            Step::Draw => vec![TurnBasedAction::DrawCard],
            _ => vec![],
        }
    }

    /// Determines if players receive priority to cast spells/activate abilities
    /// during the current step under normal conditions (Rule 500.2).
    /// Note: Players do not get priority during Untap (Rule 502.3) and normally
    /// do not during Cleanup (Rule 514.3) unless an ability triggers.
    pub fn players_receive_priority(&self) -> bool {
        match self.current_step {
            Step::Untap | Step::Cleanup => false,
            _ => true,
        }
    }
}
