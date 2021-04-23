use gtk::prelude::*;

pub struct AboutWindow {
    widget: gtk::Window
}

impl AboutWindow {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_string(include_str!("about_window.ui"));
        let widget : gtk::Window = builder.get_object("about_window")
            .expect("Failed to load the about window");
        widget.show_all();
        Self{
            widget
        }
    }
}