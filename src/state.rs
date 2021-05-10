use preferences::{Preferences,AppInfo};
extern crate serde_derive;

pub mod local_files;
pub use local_files::*;

pub mod gui;
pub use gui::*;

use crate::server::*;

use std::path::Path;

pub const APP_INFO: AppInfo = AppInfo{name: "filend", author: "hgonomeg@gmail.com"};
const PREFS_KEY: &str = "local_files";

pub struct State {
    local_files: Option<LocalFiles>,
    gui_state: Option<Gui>,
    server: Option<Server>
}

impl State {
    pub fn new() -> Self {
        Self { 
            local_files: None,
            gui_state: None,
            server: None
        }
    }
    pub fn load(&mut self) -> Result<(),String> {
        self.gui_state = Some(Gui::new());
        match LocalFiles::load(&APP_INFO,PREFS_KEY) {
            Ok(local_files) => {
                self.local_files = Some(local_files);
            },
            Err(_e) => {
                eprintln!("Couldn't load local files' list! Generating a new one...");
                self.local_files = Some(LocalFiles::default());
                match self.local_files.as_ref().unwrap().save(&APP_INFO,PREFS_KEY) {
                    Err(_e) => {
                        return Err(format!("Couldn't create storage for local files!"));
                    }
                    _ => {}
                }
            }
        }   
        for (id,filepath) in &self.local_files.as_ref().unwrap().files {
            self.gui_state.as_ref().unwrap().add_file(id,filepath);
        }
        self.server = Some(Server::new(7789));
        
        Ok(())
    }
    pub fn get_model(&self) -> Option<&gtk::ListStore> {
        if let Some(gui) = &self.gui_state {
            Some(gui.get_model())
        } else {
            None
        }
    }
    fn sync_local_files(&self) -> Result<(),String> {
        match self.local_files.as_ref().unwrap().save(&APP_INFO,PREFS_KEY) {
            Err(_e) => {
                return Err(format!("Couldn't save changes in the storage for local files!"));
            }
            Ok(()) => {
                Ok(())
            }
        }
    }
    pub fn add_file(&mut self, file: &Path) -> Result<(),String> {
        match self.local_files.as_mut() {
            Some(mut files) => {
                let id = files.add_file(file);
                self.gui_state.as_ref().unwrap().add_file(&id,file);
                self.sync_local_files()?;
                Ok(())
            },
            None => {
                Err(format!("Local files' database uninitialized!"))
            }
        }
    }
}