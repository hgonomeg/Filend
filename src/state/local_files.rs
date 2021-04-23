use serde::{Serialize,Deserialize};
use std::collections::BTreeMap;
use std::path::{Path,PathBuf};
use rand::prelude::*;

#[derive(Serialize,Deserialize)]
pub struct LocalFiles {
    files: BTreeMap<u32,PathBuf>
}

impl LocalFiles {
    pub fn add_file(&mut self, filepath: &Path) {
        let get_id = || -> u32 {
            loop {
                let ret : u32  = random();
                if ! self.files.contains_key(&ret) {
                    return ret;
                }
            }
        };
        self.files.insert(get_id(),filepath.to_owned());
    }
}