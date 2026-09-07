mod instrument;

use crate::account::case_file::entry::claim::instrument::Instrument;

pub struct Claim {
    prose: String,
    instrument: Instrument
}