use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use crate::state::State;

pub struct MainWindow {
    widget: gtk::ApplicationWindow,
    state: Rc<RefCell<State>>,
}

impl MainWindow {
    pub fn new(state: Rc<RefCell<State>>) -> Self {
        let builder = gtk::Builder::from_string(include_str!("main_window.ui"));
        let widget : gtk::ApplicationWindow = builder.get_object("main_window")
            .expect("Failed to load the main window");
        widget.show_all();
        let mut ret = Self { widget, state };
        ret.init();
        ret
    }
    pub fn init(&mut self) {
        
    }
    pub fn set_application(&self, app: &gtk::Application) {
        self.widget.set_application(Some(app));
        app.add_window(&self.widget);
    }
}