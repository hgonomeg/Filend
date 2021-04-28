use preferences::{Preferences,AppInfo};
extern crate serde_derive;

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
    pub fn load(&mut self) -> Result<(),String> {
        self.gui_state = Some(Gui::new());
        let prefs_key = "local_files";
        match LocalFiles::load(&APP_INFO,prefs_key) {
            Ok(local_files) => {
                self.local_files = Some(local_files);
                Ok(())
            },
            Err(_e) => {
                eprintln!("Couldn't load local files' list! Generating a new one...");
                self.local_files = Some(LocalFiles::default());
                match self.local_files.as_ref().unwrap().save(&APP_INFO,prefs_key) {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(_e) => {
                        Err(format!("Couldn't create storage for local files!"))
                    }
                }
                
            }
        }   
    }
    pub fn get_model(&self) -> Option<&gtk::ListStore> {
        if let Some(gui) = &self.gui_state {
            Some(gui.get_model())
        } else {
            None
        }
    }
}