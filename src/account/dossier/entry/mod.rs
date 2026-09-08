mod claim;
mod evidence;

use crate::account::dossier::entry::{claim::Claim, evidence::Evidence};

pub struct Entry {
    claim: Claim,
    evidence: Vec<Evidence>,
    // Should be Vec<(Person, Option<Organization>)>, have to decide what those types look like first
    attribution: Vec<()>
}