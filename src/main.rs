use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider};
use gtk::gdk::Display;

const APP_ID: &str = "rs.ac.bg.matf.matfquiztador";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    app.run()
}
fn load_css() {
    // ovo sam samo iskopirao nmp u sustini sta radi tacno ALI treba jos da implementiram
    // strukturu fajlova i ovo je sve za sad neka vrsta placeholdera :)
    let provider = CssProvider::new();
    provider.load_from_path("style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

}
fn build_ui(app: &Application) {

    let button = Button::builder()
        .label("placeholder")
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Matfquiztador")
        .child(&button)
        .build();

    window.present();
}