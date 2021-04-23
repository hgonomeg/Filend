use preferences::AppInfo;

pub mod local_files;
pub use local_files::*;

pub mod gui;
pub use gui::*;

pub const APP_INFO: AppInfo = AppInfo{name: "filend", author: "hgonomeg@gmail.com"};

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
        self.gui_state = Some(Gui::new());
        //self.local_files = Some()

        
    }
    pub fn get_model(&self) -> Option<&gtk::ListStore> {
        if let Some(gui) = &self.gui_state {
            Some(gui.get_model())
        } else {
            None
        }
    }
}