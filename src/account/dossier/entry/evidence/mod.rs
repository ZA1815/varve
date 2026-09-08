mod medium;

use time::Timestamp;

use crate::account::dossier::entry::evidence::medium::Medium;

pub struct Evidence {
    medium: Medium,
    timestamp: Timestamp
}