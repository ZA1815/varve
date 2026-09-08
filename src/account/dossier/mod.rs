mod entry;

use blake3::Hash;
use indexmap::IndexMap;

use crate::account::dossier::entry::Entry;

pub struct Dossier {
    entries: IndexMap<Hash, Entry>
}