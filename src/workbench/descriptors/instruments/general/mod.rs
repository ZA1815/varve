pub mod planning;
pub mod research;

use crate::workbench::descriptors::instruments::general::{planning::PlanningInstrument, research::ResearchInstrument};

pub enum GeneralInstrument {
    Planning(PlanningInstrument),
    Research(ResearchInstrument)
    // Maybe Restraint, does it fit?
}