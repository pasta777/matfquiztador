mod map;

use eframe::egui;
use egui::{Align, Layout, Pos2};
use crate::map::SerbiaMap;

// na escape se izlazi sada

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_fullscreen(true),
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
    show_confirmation_dialog: bool,
    main_menu: bool,
    playing: bool,
    serbia_map: SerbiaMap,
}

impl Default for MyApp {
    // ovo je default state (kada se ucita igrica itd)
    fn default() -> Self {
        Self {
            show_confirmation_dialog: false,
            main_menu: true,
            playing: false,
            serbia_map: SerbiaMap::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.main_menu {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional)
                );
                ui.with_layout(Layout::top_down(Align::Center).with_cross_align(Align::Center), |ui| {
                    // HACK ZA CENTRIRANJE TEKSTA NE KORISTITI NIGDE DRUGDE
                    // TREBA PRONACI BOLJI NACIN ALI TOP DOWN SA CROSS ALIGN NE RADI
                    // NE RADI NI KOMBINOVANJE HORIZONTAL CENTER I VERTICAL CENTER -- KNOWN BUG IN EGUI
                    for _i in 1..20 {
                        ui.label("");
                    }
                    // HACK SE OVDE ZAVRSAVA
                    if ui.button("Play!").clicked() {
                        self.main_menu = false;
                        self.playing = true;
                    }
                    if ui.button("Settings").clicked() {
                    }
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        }
        if self.playing {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let panel_size = ui.available_size();
                    if self.serbia_map.first_turn {
                        ui.heading("Choose capital city.");
                    }
                    self.serbia_map.draw(ui, Pos2::new(panel_size.x, panel_size.y));
                });
            }
        if self.serbia_map.show_question {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    for _i in 1..10 {
                        ui.label("");
                    }
                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Button,
                        egui::FontId::new(20.0, eframe::epaint::FontFamily::Proportional),
                    );
                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Heading,
                        egui::FontId::new(25.0, eframe::epaint::FontFamily::Proportional),
                    );
                    ui.heading("The Question placeholder");
                    if ui.button("Option 1").clicked() {
                        self.serbia_map.correct_p = false;
                        self.serbia_map.show_question = false;
                    }
                    if ui.button("Option 2").clicked() {
                        self.serbia_map.correct_p = true;
                        self.serbia_map.show_question = false;
                    }
                    if ui.button("Option 3").clicked() {
                        self.serbia_map.correct_p = false;
                        self.serbia_map.show_question = false;
                    }
                    if ui.button("Option 4").clicked() {
                        self.serbia_map.correct_p = false;
                        self.serbia_map.show_question = false;
                    }
                });
            });
        }

        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Button,
                        egui::FontId::new(20.0, eframe::epaint::FontFamily::Proportional),
                    );
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                        }
                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_confirmation_dialog = true;
        }
    }
}