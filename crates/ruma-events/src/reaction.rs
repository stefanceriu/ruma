//! Types for the *m.reaction* event.

use ruma_events_macros::EventContent;
use ruma_identifiers::EventId;
use serde::{Deserialize, Serialize};

use crate::MessageEvent;

/// A reaction to another event.
pub type ReactionEvent = MessageEvent<ReactionEventContent>;

/// The payload for a `ReactionEvent`.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.reaction", kind = Message)]
pub struct ReactionEventContent {
    /// Information about the related event.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Relation,
}

impl ReactionEventContent {
    /// Creates a new `ReactionEventContent` from the given relation.
    ///
    /// You can also construct a `ReactionEventContent` from a relation using `From` / `Into`.
    pub fn new(relates_to: Relation) -> Self {
        Self { relates_to }
    }
}

impl From<Relation> for ReactionEventContent {
    fn from(relates_to: Relation) -> Self {
        Self::new(relates_to)
    }
}

/// The relation that contains info which event the reaction is applying to.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[serde(tag = "rel_type", rename = "m.annotation")]
pub struct Relation {
    /// The event that is being reacted to.
    pub event_id: EventId,

    /// A string that holds the emoji reaction.
    pub emoji: String,
}

impl Relation {
    /// Creates a new `Relation` with the given event ID and emoji.
    pub fn new(event_id: EventId, emoji: String) -> Self {
        Self { event_id, emoji }
    }
}
