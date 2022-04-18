use gtk::{builders::*,prelude::*};
// use gio::prelude::*;
use glib::clone;
use crate::state::State;
use crate::about_window::AboutWindow;
use crate::ui_framework::*;

pub struct MainWindow {
    widget: gtk::ApplicationWindow,
    files_table: gtk::TreeView,
    status_bar: gtk::Statusbar,
    file_add: gtk::MenuItem,
    file_quit: gtk::MenuItem,
    help_about: gtk::MenuItem,
    state: Handle<State>,
}

impl MainWindow {
    pub fn new(state: Handle<State>) -> Self {
        let builder = gtk::Builder::from_string(include_str!("main_window.ui"));
        let widget : gtk::ApplicationWindow = builder.object("main_window")
            .expect("Failed to load the main window");
        let files_table : gtk::TreeView = builder.object("files_table")
            .expect("Failed to fetch the files table");
        let status_bar : gtk::Statusbar = builder.object("status_bar")
            .expect("Failed to fetch the status bar");
        let file_add : gtk::MenuItem = builder.object("file_add")
            .expect("Failed to fetch the file->add menu option");
        let file_quit : gtk::MenuItem = builder.object("file_quit")
            .expect("Failed to fetch the edit->quit menu option");
        let help_about : gtk::MenuItem = builder.object("help_about")
            .expect("Failed to fetch the help->about menu option");
        widget.show_all();
        let mut ret = Self { 
            widget, 
            files_table, 
            status_bar, 
            state,
            file_add,
            file_quit,
            help_about
        };
        ret.init();
        ret
    }
    pub fn init(&mut self) {
        self.state.borrow_mut().load().unwrap();

        let state = self.state.clone();
        self.file_add.connect_activate(clone!( @weak self.widget as widget => move |_arg| {
            let chooser = gtk::FileChooserDialog::with_buttons(
                Some("Pick a file"),
                Some(&widget),
                gtk::FileChooserAction::Open,
                &[("_Cancel", gtk::ResponseType::Cancel), ("_Open", gtk::ResponseType::Accept)]
            );
            match chooser.run() {
                gtk::ResponseType::Accept => {
                    let files = chooser.files();
                    for i in files {
                        state.borrow_mut().add_file(&i.path().unwrap()).unwrap();
                    }
                    
                }
                _ => {

                }
            }
            chooser.close();
        }));
        self.file_quit.connect_activate(clone!(@weak self.widget as widget => move |_arg| {
            widget.close();
        }));
        self.help_about.connect_activate(|_arg| {
            let _about_window = AboutWindow::new();
        });


        let generate_column = |title: &str, id: i32|{
            let column = TreeViewColumnBuilder::new().title(title).build();
            column.set_sort_column_id(id);
            column.set_resizable(true);
            let renderer = CellRendererTextBuilder::new().build();
            column.pack_start(&renderer,true);
            column.add_attribute(&renderer, "text", id);
            column
        };
        let column_names : Vec<&str> = [
            "File",
            "ID"
        ].to_vec();

        for i in column_names.iter().enumerate().map(|(id,title)| generate_column(title,id as i32)) {
            self.files_table.append_column(&i);
        } 
        self.files_table.set_model(self.state.borrow().get_model());
    }
    pub fn set_application(&self, app: &gtk::Application) {
        self.widget.set_application(Some(app));
        app.add_window(&self.widget);
    }
}