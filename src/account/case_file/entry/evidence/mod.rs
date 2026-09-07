mod medium;

use time::Timestamp;

use crate::account::case_file::entry::evidence::medium::Medium;

pub struct Evidence {
    medium: Medium,
    timestamp: Timestamp
}