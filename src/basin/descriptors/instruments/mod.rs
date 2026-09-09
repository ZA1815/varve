pub mod general;
pub mod software;

use crate::basin::descriptors::instruments::{general::GeneralInstrument, software::SoftwareInstrument};

pub enum Instrument {
    General(GeneralInstrument),
    Software(SoftwareInstrument)
}