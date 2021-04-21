use gtk::prelude::*;

pub struct MainWindow {
    widget: gtk::ApplicationWindow
}

impl MainWindow {
    pub fn new() -> Self {
        let builder = gtk::Builder::from_string(include_str!("main_window.ui"));
        let widget : gtk::ApplicationWindow = builder.get_object("main_window")
            .expect("Failed to load the main window");
        widget.show_all();
        Self { widget }
    }
    pub fn set_application(&self, app: &gtk::Application) {
        self.widget.set_application(Some(app));
        app.add_window(&self.widget);
    }
}