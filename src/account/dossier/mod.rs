mod event;

use blake3::Hash;
use indexmap::IndexMap;

use crate::account::{claim::Claim, dossier::event::Event};

pub struct Dossier {
    claims: IndexMap<Hash, Claim>,
    events: IndexMap<Hash, Event>
}