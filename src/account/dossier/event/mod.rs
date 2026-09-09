mod instrument;
mod medium;

use time::Timestamp;

use crate::account::dossier::event::{instrument::Instrument, medium::Medium};

pub struct Event {
    kind: EventKind,
    instrument: Instrument,
    medium: Medium,
    from: Timestamp,
    to: Timestamp
}

pub enum EventKind {
    Natural,
    Synthetic
}