pub mod general;
pub mod software;

use crate::workbench::descriptors::instruments::{general::GeneralInstrument, software::SoftwareInstrument};

pub enum Instrument {
    General(GeneralInstrument),
    Software(SoftwareInstrument)
}