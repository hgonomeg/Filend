use preferences::AppInfo;

pub mod local_files;
pub use local_files::*;

pub mod gui;
pub use gui::*;

pub struct State {
    local_files: Option<LocalFiles>,
    gui_state: Option<Gui>,
}

impl State {
    pub fn new() -> Self {
        Self { 
            local_files: None,
            gui_state: None
        }
    }
    pub fn load(&mut self) {

    }
}