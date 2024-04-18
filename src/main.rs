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
#[derive(Clone)]
struct City {
    name: &'static str,
    position: Pos2,
    connected_to: Vec<&'static str>
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum CityState {
    Default,
    Hovered,
    Clicked,
}
impl City {
    fn new(name: &'static str, x: f32, y: f32, connected_to: Vec<&'static str>) -> Self {
        City {
            name,
            position: Pos2::new(x,y),
            connected_to,
        }
    }
}
struct SerbiaMap {
    cities: HashMap<&'static str, City>,
    city_states: HashMap<&'static str, CityState>,
}
impl SerbiaMap {
    fn normalize(panel_center: Pos2, city_position: Pos2) -> Pos2 {
        Pos2::new(panel_center.x + city_position.x, panel_center.y - city_position.y)
    }
    fn new() -> Self {
        let mut cities = HashMap::new();
        let mut city_states = HashMap::new();
        // HACK
        // (ovo je privremeno resenje za connected_to vektor)
        let mut for_belgrades = Vec::new();
        let mut from_belgrade = Vec::new();
        for_belgrade.push("Belgrade");
        from_belgrade.push("Novi Sad");
        from_belgrade.push("Nis");
        // ovde je kraj tome
        cities.insert("Belgrade", City::new("Belgrade", 0.0, 25.0, from_belgrade.clone()));
        cities.insert("Novi Sad", City::new("Novi Sad", -50.0, 50.0, for_belgrade.clone()));
        cities.insert("Nis", City::new("Nis", 25.0, -75.0, for_belgrade.clone()));

        for(name, _) in &cities {
            city_states.insert(*name, CityState::Default);
        }
        SerbiaMap {
            cities,
            city_states
        }
    }
    fn draw(&mut self, ui: &mut Ui, panel_size: Pos2) {
        let panel_center = Pos2::new(panel_size.x / 2.0, panel_size.y / 2.0);
        let radius = 10.0;
        for(_name1, city1) in &self.cities {
            for name2 in &city1.connected_to {
                let city_position1 = Self::normalize(panel_center, city1.position);
                let city_position2 = Self::normalize(panel_center, self.cities[name2].position);
                ui.painter().line_segment(
                    [city_position1, city_position2],
                    Stroke::new(1.0, Color32::from_rgb(50, 100, 50))
                );
            }
        }
        for (name, city) in &mut self.cities {
            let state = *self.city_states.get(name).unwrap_or(&CityState::Default);
            let city_position = Self::normalize(panel_center, city.position);
            let node_rect = Rect::from_center_size(city_position, Vec2::from([radius * 2.0; 2]));
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
            ui.painter().circle_stroke(city_position, radius, Stroke::new(1.0, color));
            ui.painter()
                .text(Pos2::new(city_position.x, city_position.y - 20.0), egui::Align2::CENTER_CENTER, city.name, FontId::monospace(12.0), color);
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
            let panel_size = ui.available_size();
            self.serbia_map.draw(ui, Pos2::new(panel_size.x, panel_size.y));
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