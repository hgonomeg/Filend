mod main_window;
pub use main_window::*;

mod server;
pub use server::*;

mod state;
pub use state::*;

use gtk::prelude::*;
use gio::prelude::*;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.hgonomeg.filend"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = MainWindow::new();
        window.set_application(app);
    });

    let ret = application.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
