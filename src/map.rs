use std::collections::HashMap;
use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{Color32, FontId, Stroke};
use egui::{Sense, Ui};
#[derive(Clone)]
struct City {
    name: &'static str,
    position: Pos2,
    connected_to: Vec<&'static str>,
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
            position: Pos2::new(x, y),
            connected_to,
        }
    }
}

pub struct SerbiaMap {
    cities: HashMap<&'static str, City>,
    city_states: HashMap<&'static str, CityState>,
}

impl SerbiaMap {
    fn normalize(panel_center: Pos2, city_position: Pos2) -> Pos2 {
        Pos2::new(panel_center.x + city_position.x, panel_center.y - city_position.y)
    }
    pub fn new() -> Self {
        let mut cities = HashMap::new();
        let mut city_states = HashMap::new();

        let mut from_belgrade = Vec::new(); //done
        let mut from_pancevo = Vec::new(); //done
        let mut from_sabac = Vec::new(); //done
        let mut from_valjevo = Vec::new(); //done
        let mut from_sremskamitrovica = Vec::new(); //done
        let mut from_smederevo = Vec::new(); //done
        let mut from_zrenjenin = Vec::new(); //done
        let mut from_arandjelovac = Vec::new(); //done
        let mut from_novisad = Vec::new(); //done
        let mut from_sombor = Vec::new(); //done
        let mut from_subotica = Vec::new(); //done
        let mut from_kikinda = Vec::new(); //done
        let mut from_cacak = Vec::new(); //done
        let mut from_kraljevo = Vec::new(); //done
        let mut from_pozarevac = Vec::new(); //done
        let mut from_kragujevac = Vec::new(); //done
        let mut from_jagodina = Vec::new(); //done
        let mut from_bor = Vec::new(); //done
        let mut from_zajecar = Vec::new(); //done
        let mut from_uzice = Vec::new(); //done
        let mut from_krusevac = Vec::new(); //done
        let mut from_nis = Vec::new(); //done
        let mut from_prokuplje = Vec::new(); //done
        let mut from_pirot = Vec::new(); //done
        let mut from_leskovac = Vec::new(); //done
        let mut from_vranje = Vec::new(); //done
        let mut from_pristina = Vec::new(); //done
        let mut from_pec = Vec::new(); //done
        let mut from_prizren = Vec::new(); //done
        let mut from_kosovskamitrovica = Vec::new(); //done
        let mut from_gnjilane = Vec::new();
        from_belgrade.push("Pancevo"); // Beograd, Smederevo, Zrenjenin
        from_belgrade.push("Sabac"); // Beograd, Sremska Mitrovica, Valjevo
        from_belgrade.push("Valjevo"); // Beograd, Cacak, Sabac, Arandjelovac
        from_belgrade.push("Sremska Mitrovica"); // Beograd, Novi Sad,Sabac, Sombor
        from_belgrade.push("Smederevo"); // Beograd, Pozarevac, Arandjelovac, Pancevo Velika Plana
        from_belgrade.push("Zrenjenin"); // Beograd, Novi Sad, Pancevo, Kikinda
        from_belgrade.push("Arandjelovac"); // Valjevo, Beograd, Cacak, Kragujevac, Velika Plana

        from_pancevo.push("Belgrade");
        from_pancevo.push("Smederevo");
        from_pancevo.push("Zrenjenin");

        from_sabac.push("Belgrade");
        from_sabac.push("Sremska Mitrovica");
        from_sabac.push("Valjevo");

        from_valjevo.push("Belgrade");
        from_valjevo.push("Cacak");
        from_valjevo.push("Sabac");
        from_valjevo.push("Arandjelovac");
        from_valjevo.push("Uzice");

        from_sremskamitrovica.push("Belgrade");
        from_sremskamitrovica.push("Novi Sad");
        from_sremskamitrovica.push("Sabac");

        from_smederevo.push("Belgrade");
        from_smederevo.push("Pozarevac");
        from_smederevo.push("Arandjelovac");
        from_smederevo.push("Pancevo");
        //from_smederevo.push("Velika Plana");

        from_zrenjenin.push("Belgrade");
        from_zrenjenin.push("Novi Sad");
        from_zrenjenin.push("Pancevo");
        from_zrenjenin.push("Kikinda");

        from_arandjelovac.push("Belgrade");
        from_arandjelovac.push("Valjevo");
        from_arandjelovac.push("Cacak");
        from_arandjelovac.push("Kragujevac");
        //from_arandjelovac.push("Velika Plana");


        from_novisad.push("Sremska Mitrovica"); //-
        from_novisad.push("Zrenjenin"); //-
        from_novisad.push("Sombor"); // Novi Sad, Subotica
        from_novisad.push("Subotica"); //Sombor, Novi Sad, Kikinda

        from_sombor.push("Novi Sad");
        from_sombor.push("Subotica");

        from_subotica.push("Sombor");
        from_subotica.push("Novi Sad");
        from_subotica.push("Kikinda");

        from_kikinda.push("Zrenjenin");//gas
        from_kikinda.push("Subotica");

        from_cacak.push("Kraljevo");
        from_cacak.push("Arandjelovac");
        from_cacak.push("Kragujevac");
        from_cacak.push("Valjevo");
        from_cacak.push("Uzice");

        from_kraljevo.push("Cacak");
        from_kraljevo.push("Kragujevac");
        from_kraljevo.push("Jagodina");
        from_kraljevo.push("Krusevac");

        from_pozarevac.push("Smederevo");
        //from_pozarevac.push("Velika Plana");

        from_kragujevac.push("Kraljevo");
        from_kragujevac.push("Jagodina");
        from_kragujevac.push("Cacak");
        from_kragujevac.push("Arandjelovac");
        //from_kragujevac("Velika Plana");

        from_jagodina.push("Kraljevo");
        from_jagodina.push("Kragujevac");
        from_jagodina.push("Bor");
        from_jagodina.push("Krusevac");
        //from_jagodina.push("Velika Plana");

        from_bor.push("Jagodina");
        from_bor.push("Zajecar");
        //from_bor.push("Velika Plana");

        from_zajecar.push("Bor");
        from_zajecar.push("Nis");

        from_uzice.push("Cacak");
        from_uzice.push("Valjevo");

        from_krusevac.push("Kraljevo");
        from_krusevac.push("Nis");
        from_krusevac.push("Jagodina");
        from_krusevac.push("Prokuplje");

        from_nis.push("Krusevac");
        from_nis.push("Zajecar");
        from_nis.push("Pirot");
        from_nis.push("Prokuplje");

        from_prokuplje.push("Nis");
        from_prokuplje.push("Leskovac");
        from_prokuplje.push("Krusevac");

        from_pirot.push("Nis");
        from_pirot.push("Leskovac");

        from_leskovac.push("Pirot");
        from_leskovac.push("Nis");
        from_leskovac.push("Vranje");

        from_vranje.push("Leskovac");
        from_vranje.push("Pristina");
        from_vranje.push("Gnjilane");

        from_pristina.push("Vranje");
        from_pristina.push("Prizren");
        from_pristina.push("Kosovska Mitrovica");
        from_pristina.push("Pec");
        from_pristina.push("Gnjilane");

        from_pec.push("Pristina");
        from_pec.push("Kosovska Mitrovica");
        from_pec.push("Prizren");

        from_prizren.push("Pristina");
        from_prizren.push("Pec");

        from_kosovskamitrovica.push("Pristina");
        from_kosovskamitrovica.push("Pec");

        from_gnjilane.push("Pristina");
        from_gnjilane.push("Vranje");

        cities.insert("Belgrade", City::new("Belgrade", 0.0, 195.0, from_belgrade.clone()));
        cities.insert("Pancevo", City::new("Pancevo", 30.0, 225.0, from_pancevo.clone()));
        cities.insert("Sabac", City::new("Sabac", -100.0, 195.0, from_sabac.clone()));
        cities.insert("Valjevo", City::new("Valjevo", -85.0, 75.0, from_valjevo.clone()));
        cities.insert("Sremska Mitrovica",City::new("Sremska Mitrovica", -100.0,240.0,from_sremskamitrovica.clone()));
        cities.insert("Smederevo",City::new("Smederevo", 80.0,170.0,from_smederevo.clone()));
        cities.insert("Arandjelovac",City::new("Arandjelovac", 0.0,70.0,from_arandjelovac.clone()));
        cities.insert("Novi Sad",City::new("Novi Sad", -85.0, 320.0, from_novisad.clone()));
        cities.insert("Sombor",City::new("Sombor", -150.0, 440.0, from_sombor.clone()));
        cities.insert("Subotica",City::new("Subotica", -90.0, 470.0, from_subotica.clone()));
        cities.insert("Kikinda",City::new("Kikinda", 150.0, 225.0, from_kikinda.clone()));
        cities.insert("Cacak",City::new("Cacak", -40.0, -10.0, from_cacak.clone()));
        cities.insert("Kraljevo",City::new("Kraljevo", 10.0, -50.0, from_kraljevo.clone()));
        cities.insert("Zrenjenin",City::new("Zrenjenin", 80.0, 170.0, from_zrenjenin.clone()));
        cities.insert("Pozarevac",City::new("Pozarevac", 130.0,160.0, from_pozarevac.clone()));
        cities.insert("Kragujevac",City::new("Kragujevac", 80.0, 10.0, from_kragujevac.clone()));
        cities.insert("Jagodina",City::new("Jagodina", 130.0, 0.0, from_jagodina.clone()));
        cities.insert("Bor",City::new("Bor", 260.0, 10.0, from_bor.clone()));
        cities.insert("Zajecar",City::new("Zajecar", 290.0, -10.0, from_zajecar.clone()));
        cities.insert("Uzice",City::new("Uzice", -105.0, -20.0, from_uzice.clone()));
        cities.insert("Krusevac",City::new("Krusevac", 130.0, -70.0, from_krusevac.clone()));
        cities.insert("Nis",City::new("Nis", 210.0, -150.0, from_nis.clone()));
        cities.insert("Prokuplje",City::new("Prokuplje", 180.0, -150.0, from_prokuplje.clone()));
        cities.insert("Pirot",City::new("Pirot", 320.0, -170.0, from_pirot.clone()));
        cities.insert("Leskovac",City::new("Leskovac", 230.0, -220.0, from_leskovac.clone()));
        cities.insert("Vranje",City::new("Vranje", 210.0, -370.0, from_vranje.clone()));
        cities.insert("Pristina",City::new("Pristina", 105.0, -330.0, from_pristina.clone()));
        cities.insert("Pec",City::new("Pec", -80.0, -330.0, from_pec.clone()));
        cities.insert("Prizren",City::new("Prizren", 10.0, -430.0, from_prizren.clone()));
        cities.insert("Kosovska Mitrovica",City::new("Kosovska Mitrovica", 50.0, -230.0, from_kosovskamitrovica.clone()));
        cities.insert("Gnjilane",City::new("Gnjilane", 180.0, -390.0, from_gnjilane.clone()));


        for (name, _) in &cities {
            city_states.insert(*name, CityState::Default);
        }
        SerbiaMap {
            cities,
            city_states,
        }
    }
    pub fn draw(&mut self, ui: &mut Ui, panel_size: Pos2) {
        let panel_center = Pos2::new(panel_size.x / 2.0, panel_size.y / 2.0);
        let radius = 10.0;
        for (name1, city1) in &self.cities {
            for name2 in &city1.connected_to {
                let city_position1 = Self::normalize(panel_center, city1.position);
                let city_position2 = Self::normalize(panel_center, self.cities[name2].position);
                let color =
                    if self.city_states[name1] == CityState::Clicked && self.city_states[name2] == CityState::Clicked {
                        Color32::from_rgb(150, 150, 50)
                    } else {
                        Color32::from_rgb(50, 100, 50)
                    };
                ui.painter().line_segment(
                    [city_position1, city_position2],
                    Stroke::new(1.0, color)
                );
            }
        }
        for (name, city) in &self.cities {
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
            ui.painter().circle_filled(city_position, radius, color);
            ui.painter()
                .text(Pos2::new(city_position.x, city_position.y - 20.0), egui::Align2::CENTER_CENTER, city.name, FontId::monospace(12.0), color);
        }
    }
}
