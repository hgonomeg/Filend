use gtk::prelude::*;

pub struct AboutWindow {
    widget: gtk::Window,
    version_label: gtk::Label
}

impl AboutWindow {
    pub fn init(&mut self) {
        let ver_str = concat!("version: ",env!("CARGO_PKG_VERSION"));
        self.version_label.set_text(ver_str);
    }
    pub fn new() -> Self {
        let builder = gtk::Builder::from_string(include_str!("about_window.ui"));
        let widget : gtk::Window = builder.object("about_window")
            .expect("Failed to load the about window");
        let version_label : gtk::Label = builder.object("version_label")
            .expect("Failed to load the version label");
        widget.show_all();
        let mut ret = Self{
            widget,
            version_label
        };
        ret.init();
        ret
    }
}
