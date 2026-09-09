use time::Timestamp;

use crate::workbench::descriptors::{instruments::Instrument, mediums::Medium};

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