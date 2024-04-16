use eframe::egui;
use std::collections::HashMap;
use egui::{Color32, FontId, Pos2, Stroke, Ui};

// graf nije perfektan iako je skoro implementiran treba:
// uciniti da se prozor ne resize do odredjene mere
// urediti ui generalno
// napraviti kolko tolku funkcionalnost
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 340.0]),
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
#[derive(Clone, Copy)]
struct City {
    name: &'static str,
    position: Pos2,
}
impl City {
    fn new(name: &'static str, x: f32, y: f32) -> Self {
        City {
            name,
            position: Pos2::new(x,y),
        }
    }
}
struct SerbiaMap {
    cities: HashMap<&'static str, City>,
}
impl SerbiaMap {
    fn new() -> Self {
        let mut cities = HashMap::new();
        cities.insert("Belgrade", City::new("Belgrade", 100.0, 100.0));
        cities.insert("Novi Sad", City::new("Novi Sad", 200.0, 200.0));
        cities.insert("Nis", City::new("Nis", 300.0, 150.0));
        SerbiaMap { cities }
    }
    fn draw(&self, ui: &mut Ui) {
        let radius = 10.0;
        let color = Color32::from_rgb(100, 200, 100);
        for (_, city) in &self.cities {
            ui.painter().circle_stroke(city.position, radius, Stroke::new(1.0, color));
            ui.painter()
                .text(city.position, egui::Align2::CENTER_CENTER, city.name, FontId::monospace(16.0), color);
        }

        for(name1, city1) in &self.cities {
            for (name2, city2) in &self.cities {
                if name1 != name2 {
                    ui.painter().line_segment(
                        [city1.position, city2.position],
                        Stroke::new(1.0, color)
                    );
                }
            }
        }
    }
}
struct MyApp {
    right_visible: bool,
}

impl Default for MyApp {
    // ovo je default state (kada se ucita igrica itd)
    fn default() -> Self {
        Self {
            right_visible: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let serbia_map = SerbiaMap::new();
        egui::CentralPanel::default().show(ctx, |ui|{
            serbia_map.draw(ui);
        });
        if self.right_visible {
            egui::SidePanel::right("right_panel").show(ctx, |ui| {
                if ui.button("Play!").clicked() {
                    self.right_visible = false;
                }
                if ui.button("Settings").clicked() {
                }
            });
        }
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
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