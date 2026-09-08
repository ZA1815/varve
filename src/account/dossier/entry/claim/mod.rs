mod instrument;

use crate::account::dossier::entry::claim::instrument::Instrument;

pub struct Claim {
    prose: String,
    instrument: Instrument
}