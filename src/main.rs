use gtk::glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

struct AppModel {
    //counter: u8,
}

#[derive(Debug)]
enum AppInput {
    Play,
    Settings,
}

// ostavljeno kao referenca
struct AppWidgets {
//     label: gtk::Label,
}

impl SimpleComponent for AppModel {

    /// The type of the messages that this component can receive.
    type Input = AppInput;
    /// The type of the messages that this component can send.
    type Output = ();
    /// The type of data with which this component will be initialized.
    type Init = u8;
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Matfquiztador")
            .default_width(300)
            .default_height(100)
            .build()
    }

    /// Initialize the UI and model.
    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { /*counter*/ };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();


        let play_button = gtk::Button::with_label("Play!");
        let settings_button = gtk::Button::with_label("Settings");

        // ovo sam ostavio kao referencu
        //let label = gtk::Label::new(Some(&format!("Counter: {}", model.counter)));
        //label.set_margin_all(5);

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.append(&play_button);
        vbox.append(&settings_button);

        play_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Play);
        }));

        settings_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Settings);
        }));

        // ostavljeno kao referenca
        let widgets = AppWidgets { /*label*/  };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Play => {
                // ovde treba da se otvori nova strana gde ce igrac birati da li ce igrati singleplayer protiv bota ili multiplayer
                // jos nisam siguran kako i da li cemo implementirati multiplayer
            }
            AppInput::Settings => {
                // samo otvoriti settings gde igrac bira tezinu bota i jos neke stvari kao sto su mozda mapa???
            }
        }
    }

    /// Update the view to represent the updated model.
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
    }
}

fn main() {
    let app = RelmApp::new("matfquiztador.prototype");
    app.run::<AppModel>(0);
}