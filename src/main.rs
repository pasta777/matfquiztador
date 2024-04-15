use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Matfquiztador",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    // ovde stoje opcije za igricu
}

impl Default for MyApp {
    // ovo je default state (kada se ucita igrica itd)
    fn default() -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // placeholder za mapu
            ui.image(egui::include_image!("../example.png"));
            ui.heading("The Question placeholder");
            ui.horizontal(|ui| {
                let _op1 = ui.button("Option 1");
                let _op2 = ui.button("Option 2");
            });
            ui.horizontal(|ui| {
                let _op3 = ui.button("Option 3");
                let _op4 = ui.button("Option 4");
            });
        });
    }
}