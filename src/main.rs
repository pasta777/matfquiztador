mod map;

use eframe::egui;
use egui::{Pos2};
use crate::map::SerbiaMap;

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
struct MyApp {
    right_visible: bool,
    serbia_map: SerbiaMap,
}

impl Default for MyApp {
    // ovo je default state (kada se ucita igrica itd)
    fn default() -> Self {
        Self {
            right_visible: true,
            serbia_map: SerbiaMap::new(),
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
                    //todo
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