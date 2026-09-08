mod dossier;

use blake3::Hash;
use indexmap::IndexMap;

use crate::account::dossier::Dossier;

pub struct Varve {
    files: IndexMap<Hash, Dossier>
}