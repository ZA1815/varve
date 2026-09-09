mod claim;
mod dossier;

use blake3::Hash;
use indexmap::IndexMap;

use crate::account::{claim::Claim, dossier::Dossier};

pub struct Varve {
    claims: IndexMap<Hash, Claim>,
    dossiers: IndexMap<Hash, Dossier>
}
