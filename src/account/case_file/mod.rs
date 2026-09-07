mod entry;

use blake3::Hash;
use indexmap::IndexMap;

use crate::account::case_file::entry::Entry;

pub struct CaseFile {
    entries: IndexMap<Hash, Entry>
}