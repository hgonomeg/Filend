use serde::{Serialize,Deserialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Serialize,Deserialize)]
pub struct LocalFiles {
    files: BTreeMap<u64,PathBuf>
}

impl LocalFiles {
    
}