use gtk::prelude::*;
use glib::types::Type;

pub struct Gui {
    files_model: gtk::ListStore
}

impl Gui {
    pub fn new() -> Self {
        Self { 
            files_model: gtk::ListStore::new(&[
                glib::Type::String,
                glib::Type::String
            ])
        }
    }
    pub fn get_model(&self) -> &gtk::ListStore {
        &self.files_model
    }
}