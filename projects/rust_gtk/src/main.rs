extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, Window, WindowType};


fn main() {
    gtk::init().expect("failed to initialize gtk");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Rust UI");
    window.set_default_size(320, 240);

    let button = Button::with_label("click here to quit");

    button.connect_clicked(|_| {
        gtk::main_quit();
    });

    window.add(&button);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
