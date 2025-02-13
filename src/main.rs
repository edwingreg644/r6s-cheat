use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};
use std::process::Command;

fn main() {
    let app = Application::new(Some("com.example.r6s_cheat"), Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("R6s Cheat");
        window.set_default_size(300, 200);

        let label = Label::new(Some("R6s Cheat Loaded"));
        let button = Button::with_label("Launch Cheat");
        button.connect_clicked(|_| {
            Command::new("r6s-cheat.exe").spawn().expect("Failed to launch cheat");
        });

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        vbox.pack_start(&label, true, true, 0);
        vbox.pack_start(&button, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });
    app.run();
}