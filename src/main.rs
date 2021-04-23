mod main_window;
pub use main_window::*;

mod server;
pub use server::*;

mod state;
pub use state::*;

mod about_window;
pub use about_window::*;

use gtk::prelude::*;
use gio::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.hgonomeg.filend"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let state = Rc::from(RefCell::from(State::new()));
        let window = MainWindow::new(state);
        window.set_application(app);
    });

    let ret = application.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
