#[macro_use]
extern crate serde_derive;

mod main_window;
pub use main_window::*;

mod server;
pub use server::*;

mod state;
pub use state::*;

mod about_window;
pub use about_window::*;

mod ui_framework;
use ui_framework::*;

use gtk::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.hgonomeg.filend"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let state = State::new().into_handle();
        let window = MainWindow::new(state);
        window.set_application(app);
    });

    let ret = application.run();
    std::process::exit(ret);
}
