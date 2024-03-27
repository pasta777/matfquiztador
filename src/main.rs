use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "rs.ac.bg.matf.conquiztador";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title

    let button = Button::builder()
        .label("placeholder")
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Conquiztador")
        .child(&button)
        .build();

    // Present window
    window.present();
}