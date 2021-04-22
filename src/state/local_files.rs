use serde::{Serialize,Deserialize};
use std::collections::BTreeMap;
use std::path::{Path,PathBuf};
use rand::prelude::*;

#[derive(Serialize,Deserialize)]
pub struct LocalFiles {
    files: BTreeMap<u64,PathBuf>
}

impl LocalFiles {
    pub fn add_file(&mut self, filepath: &Path) {
        let get_id = || ->u64 {
            5 //dummy
        };
        self.files.insert(get_id(),filepath.to_owned());
    }
}