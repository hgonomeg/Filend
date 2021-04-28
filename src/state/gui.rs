use gtk::prelude::*;
use glib::types::Type;
use std::path::Path;
use crate::local_files::LocalFiles;

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
    pub fn add_file(&self, id: &u32, filepath: &Path) {
        let values: [&dyn ToValue; 2] = [
                &filepath.as_os_str().to_str().unwrap(),
                &LocalFiles::id_to_hexstring(*id),
            ];
        self.files_model.set(&self.files_model.append(), &[0,1],&values);
    }
}