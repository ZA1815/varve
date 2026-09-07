mod case_file;

use blake3::Hash;
use indexmap::IndexMap;

use crate::account::case_file::CaseFile;

pub struct Varve {
    files: IndexMap<Hash, CaseFile>
}