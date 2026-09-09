use time::Timestamp;

pub struct Event {
    kind: EventKind,
    medium: Medium,
    timestamp: Timestamp
}

pub enum EventKind {
    Natural,
    Synthetic
}

// Start to enumerate this soon
pub enum Medium {
    
}