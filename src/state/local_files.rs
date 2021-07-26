extern crate serde_derive;
use std::collections::BTreeMap;
use std::path::{Path,PathBuf};
use rand::prelude::*;

#[derive(Serialize,Deserialize,Debug,PartialEq,Default)]
pub struct LocalFiles {
    pub(super) files: BTreeMap<u32,PathBuf>
}

impl LocalFiles {
    pub fn add_file(&mut self, filepath: &Path) -> u32 {
        let mut ret : u32;
        loop {
            ret = random();
            if ! self.files.contains_key(&ret) {
                break;
            }
        }
        self.files.insert(ret,filepath.to_owned());
        ret
    }
    pub fn id_to_hexstring(id: u32) -> String {
        format!("{:x}",id)
    }
}