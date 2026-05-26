use crate::effects::{CardId, PlayerId, OneShotEffect, Target};

/// Unique identifier for an item currently resting on the stack.
/// This allows other spells/abilities to target and counter specific stack items.
pub type StackItemId = u32;

/// Represents an unresolved spell or ability object on the stack (Rule 405.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackObject {
    /// A cast spell on the stack (Rule 405.1 / 601.2).
    /// Once resolved, permanent spells enter the battlefield (Rule 608.3),
    /// while instant/sorcery spells apply their one-shot effects and go to the graveyard (Rule 608.2).
    Spell {
        card_id: CardId,
        caster: PlayerId,
        /// The list of immediate effects that apply when this spell resolves.
        effects: Vec<OneShotEffect>,
    },
    /// Activated ability on the stack (Rule 405.1 / 602.2).
    /// Does not include mana abilities, which resolve immediately without using the stack (Rule 605.1a).
    ActivatedAbility {
        source_id: CardId,
        activator: PlayerId,
        ability_id: u32,
        /// The list of immediate effects that apply when this ability resolves.
        effects: Vec<OneShotEffect>,
    },
    /// A triggered ability on the stack (Rule 405.1 / 603.2).
    /// Created automatically when a trigger condition is met, then placed on the stack (Rule 603.3).
    TriggeredAbility {
        source_id: CardId,
        controller: PlayerId,
        ability_id: u32,
        /// The list of immediate effects that apply when this triggered ability resolves.
        effects: Vec<OneShotEffect>,
    },
}

/// Represents a single wrapper item on the stack with a unique ID for targeting/countering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackItem {
    pub id: StackItemId,
    pub object: StackObject,
    pub targets: Vec<Target>,
}

/// Represents the Stack zone, modeled as a last-in, first-out (LIFO) queue (Section 405).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Stack {
    /// The actual ordered queue of stack items, where the last item is the top of the stack (Rule 405.5).
    pub items: Vec<StackItem>,
    /// Unique ID generator for newly placed stack items.
    next_id: StackItemId,
}

impl Stack {
    /// Initializes an empty stack zone.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a cast spell onto the stack.
    pub fn push_spell(&mut self, card_id: CardId, caster: PlayerId, effects: Vec<OneShotEffect>) -> StackItemId {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(StackItem {
            id,
            object: StackObject::Spell { card_id, caster, effects },
            targets: Vec::new(),
        });
        id
    }

    /// Pushes an activated ability onto the stack.
    pub fn push_activated_ability(&mut self, source_id: CardId, activator: PlayerId, ability_id: u32, effects: Vec<OneShotEffect>) -> StackItemId {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(StackItem {
            id,
            object: StackObject::ActivatedAbility { source_id, activator, ability_id, effects },
            targets: Vec::new(),
        });
        id
    }

    /// Pushes a triggered ability onto the stack.
    pub fn push_triggered_ability(&mut self, source_id: CardId, controller: PlayerId, ability_id: u32, effects: Vec<OneShotEffect>) -> StackItemId {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(StackItem {
            id,
            object: StackObject::TriggeredAbility { source_id, controller, ability_id, effects },
            targets: Vec::new(),
        });
        id
    }

    /// Pops the top object off the stack (Rule 405.5).
    /// This represents the object that resolves next when all players pass in succession (Rule 117.4).
    pub fn pop(&mut self) -> Option<StackItem> {
        self.items.pop()
    }

    /// Removes (counters or exiles) a specific item on the stack by its ID (Rule 701.5).
    /// Returns the removed item if successful, or None if the ID wasn't found.
    pub fn remove(&mut self, id: StackItemId) -> Option<StackItem> {
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Returns the number of items currently on the stack.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if there are no items on the stack.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns a reference to the top item on the stack without removing it.
    pub fn peek(&self) -> Option<&StackItem> {
        self.items.last()
    }

    /// Returns an iterator over the stack items, from bottom of the stack to the top.
    pub fn iter(&self) -> std::slice::Iter<'_, StackItem> {
        self.items.iter()
    }
}
