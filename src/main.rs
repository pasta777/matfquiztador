use eframe::egui;
use std::collections::HashMap;
use egui::{Color32, FontId, Pos2, Rect, Sense, Stroke, Ui, Vec2};

// graf nije perfektan iako je skoro implementiran treba:
// uciniti da se prozor ne resize do odredjene mere
// urediti ui generalno
// napraviti kolko tolku funkcionalnost - uradjeno donekle
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
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum CityState {
    Default,
    Hovered,
    Clicked,
}
impl City {
    fn new(name: &'static str, x: f32, y: f32) -> Self {
        City {
            name,
            position: Pos2::new(x,y)
        }
    }
}
struct SerbiaMap {
    cities: HashMap<&'static str, City>,
    city_states: HashMap<&'static str, CityState>,
}
impl SerbiaMap {
    fn new() -> Self {
        let mut cities = HashMap::new();
        let mut city_states = HashMap::new();

        cities.insert("Belgrade", City::new("Belgrade", 100.0, 100.0));
        cities.insert("Novi Sad", City::new("Novi Sad", 200.0, 200.0));
        cities.insert("Nis", City::new("Nis", 300.0, 150.0));

        for(name, _) in &cities {
            city_states.insert(*name, CityState::Default);
        }
        SerbiaMap {
            cities,
            city_states
        }
    }
    fn draw(&mut self, ui: &mut Ui) {
        let radius = 10.0;
        for (name, city) in &mut self.cities {
            let state = *self.city_states.get(name).unwrap_or(&CityState::Default);
            let node_rect = Rect::from_center_size(city.position, Vec2::from([radius * 2.0; 2]));
            let is_hovered = ui
                .interact(node_rect, egui::Id::new(*name), Sense::hover())
                .hovered();
            let is_clicked = ui
                .interact(node_rect, egui::Id::new(*name), Sense::click())
                .clicked();

            let new_state = match (state, is_hovered, is_clicked) {
                (CityState::Clicked, _, _) => CityState::Clicked, // Keep clicked state if already clicked
                (_, true, false) => CityState::Hovered, // Change to hovered if hovered
                (_, _, true) => CityState::Clicked, // Change to clicked if clicked
                _ => CityState::Default, // Default state otherwise
            };

            self.city_states.insert(*name, new_state);
            let color = match new_state {
                CityState::Clicked => Color32::from_rgb(200, 200, 100),
                CityState::Hovered => Color32::from_rgb(200, 100, 100),
                _ => Color32::from_rgb(100, 200, 100)
            };
            ui.painter().circle_stroke(city.position, radius, Stroke::new(1.0, color));
            ui.painter()
                .text(city.position, egui::Align2::CENTER_CENTER, city.name, FontId::monospace(16.0), color);
        }

        for(name1, city1) in &self.cities {
            for (name2, city2) in &self.cities {
                if name1 != name2 {
                    ui.painter().line_segment(
                        [city1.position, city2.position],
                        Stroke::new(1.0, Color32::from_rgb(100, 200, 100))
                    );
                }
            }
        }
    }
}
struct MyApp {
    right_visible: bool,
    serbia_map: SerbiaMap
}

impl Default for MyApp {
    // ovo je default state (kada se ucita igrica itd)
    fn default() -> Self {
        Self {
            right_visible: true,
            serbia_map: SerbiaMap::new()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            self.serbia_map.draw(ui);
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