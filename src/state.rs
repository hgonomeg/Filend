use preferences::AppInfo;
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
struct LocalFiles {

}

pub struct State {
    local_files: Option<LocalFiles>
}

impl State {
    pub fn new() -> Self {
        Self { local_files: None }
    }
    pub fn load(&mut self) {

    }
}