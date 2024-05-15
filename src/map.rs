use std::collections::{HashMap, HashSet};
use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{Color32, FontId, Stroke};
use egui::{Sense, Ui};
use rand::{Rng};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;

#[derive(Clone)]
struct City {
    name: &'static str,
    position: Pos2,
    connected_to: Vec<&'static str>,
}
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum CityState {
    Default,
    Hovered,
    Clicked,
    Player,
    Bot,
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
    pub city_states: HashMap<&'static str, CityState>,
    is_capital: HashMap<&'static str, bool>,
    eligible_cities_p: HashSet<&'static str>,
    eligible_cities_b: HashSet<&'static str>,
    pub player: bool,
    pub first_turn: bool,
    pub show_question: bool,
    pub correct_p: bool,
    pub war_phase: i8,
    pub bot_attacked: bool,
    pub attacked_city: &'static str,
    rng: ThreadRng,
    pub difficulty: f64,
    pub num_turns: u8,
    pub max_turns: u8,
    pub winner: &'static str,
    pub player_defended: u32,
    pub bot_defended: u32,
    pub player_capital_health: u8,
    pub bot_capital_health: u8,
    pub bot_gain_health_try: bool,
    pub bot_gained_health: bool,
    pub capital_attacked: bool,
    pub rng_gen: bool,
    gain_health: f64,
}

impl SerbiaMap {
    fn normalize(panel_size: Pos2, city_position: Pos2) -> Pos2 {
        let panel_center = Pos2::new(panel_size.x / 2.0, panel_size.y / 2.0);
        let x_new = panel_center.x + city_position.x * panel_size.x;
        let y_new = panel_center.y - city_position.y * panel_size.y;
        Pos2::new(x_new, y_new)
    }
    fn neighbors(city: City, city_state: CityState, city_states: &HashMap<&'static str, CityState>, eligible_cities: &mut HashSet<&'static str>) {
        for c in city.connected_to {
            if city_states[c] == city_state {
                eligible_cities.insert(c);
            }
        }
    }
    pub fn state_change(&mut self, correct: bool) -> bool {
        let x: f64 = self.rng.gen();
        let bot = x < self.difficulty;
        for (name, state) in &mut self.city_states {
            if *state == CityState::Clicked {
                if correct && bot {
                } else if correct {
                    if self.is_capital[*name] {
                        self.capital_attacked = true;
                        if self.player {
                            self.bot_capital_health -= 1;
                        } else {
                            *state = CityState::Player;
                        }
                        if self.bot_capital_health == 0 {
                            *state = CityState::Player;
                            self.eligible_cities_p.remove(*name);
                            self.winner = "Player";
                            self.is_capital.insert(*name, false);
                        }
                    } else {
                        self.capital_attacked = false;
                        *state = CityState::Player;
                        self.eligible_cities_p.remove(*name);
                        for c in &self.cities[*name].connected_to {
                            self.eligible_cities_b.remove(*c);
                        }
                    }
                } else if bot {
                    if self.is_capital[*name] {
                        self.capital_attacked = true;
                        if self.player {
                            *state = CityState::Bot;
                        } else {
                            self.player_capital_health -= 1;
                        }
                        if self.player_capital_health == 0 {
                            *state = CityState::Bot;
                            self.eligible_cities_b.remove(*name);
                            self.winner = "Bot";
                            self.is_capital.insert(*name, false);
                        }
                    } else {
                        self.capital_attacked = false;
                        *state = CityState::Bot;
                        self.eligible_cities_b.remove(*name);
                        for c in &self.cities[*name].connected_to {
                            self.eligible_cities_p.remove(*c);
                        }
                    }
                } else {
                    if self.player {
                        *state = CityState::Bot;
                    } else {
                        *state = CityState::Player;
                    }
                }
            }
        }
        return bot;
        // rust got me tweakin'
    }
    pub fn new() -> Self {
        let mut cities = HashMap::new();
        let mut city_states = HashMap::new();
        let mut is_capital = HashMap::new();

        let mut from_belgrade = Vec::new(); //done
        let mut from_pancevo = Vec::new(); //done
        let mut from_sabac = Vec::new(); //done
        let mut from_valjevo = Vec::new(); //done
        let mut from_sremskamitrovica = Vec::new(); //done
        let mut from_smederevo = Vec::new(); //done
        let mut from_zrenjanin = Vec::new(); //done
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
        let mut from_novipazar = Vec::new();

        from_belgrade.push("Pancevo"); // Beograd, Smederevo, Zrenjenin
        from_belgrade.push("Sabac"); // Beograd, Sremska Mitrovica, Valjevo
        from_belgrade.push("Valjevo"); // Beograd, Cacak, Sabac, Arandjelovac
        from_belgrade.push("Sremska Mitrovica"); // Beograd, Novi Sad,Sabac, Sombor
        from_belgrade.push("Smederevo"); // Beograd, Pozarevac, Arandjelovac, Pancevo Velika Plana
        from_belgrade.push("Arandjelovac"); // Valjevo, Beograd, Cacak, Kragujevac, Velika Plana
        from_belgrade.push("Novi Sad");

        from_pancevo.push("Belgrade");
        from_pancevo.push("Smederevo");
        from_pancevo.push("Zrenjanin");

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
        from_smederevo.push("Jagodina");
        //from_smederevo.push("Velika Plana");

        from_zrenjanin.push("Novi Sad");
        from_zrenjanin.push("Pancevo");
        from_zrenjanin.push("Kikinda");

        from_arandjelovac.push("Belgrade");
        from_arandjelovac.push("Valjevo");
        from_arandjelovac.push("Cacak");
        from_arandjelovac.push("Kragujevac");
        from_arandjelovac.push("Smederevo");
        //from_arandjelovac.push("Velika Plana");

        from_novisad.push("Sremska Mitrovica"); //-
        from_novisad.push("Zrenjanin"); //-
        from_novisad.push("Sombor"); // Novi Sad, Subotica
        from_novisad.push("Subotica"); //Sombor, Novi Sad, Kikinda
        from_novisad.push("Belgrade");

        from_sombor.push("Novi Sad");
        from_sombor.push("Subotica");

        from_subotica.push("Sombor");
        from_subotica.push("Novi Sad");
        from_subotica.push("Kikinda");

        from_kikinda.push("Zrenjanin");//gas
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
        from_kraljevo.push("Novi Pazar");

        from_pozarevac.push("Smederevo");
        from_pozarevac.push("Bor");
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
        from_jagodina.push("Smederevo");
        from_jagodina.push("Nis");
        //from_jagodina.push("Velika Plana");

        from_bor.push("Jagodina");
        from_bor.push("Zajecar");
        from_bor.push("Pozarevac");
        //from_bor.push("Velika Plana");

        from_zajecar.push("Bor");
        from_zajecar.push("Nis");

        from_uzice.push("Cacak");
        from_uzice.push("Valjevo");
        from_uzice.push("Novi Pazar");

        from_krusevac.push("Kraljevo");
        from_krusevac.push("Nis");
        from_krusevac.push("Jagodina");
        from_krusevac.push("Prokuplje");

        from_nis.push("Krusevac");
        from_nis.push("Zajecar");
        from_nis.push("Pirot");
        from_nis.push("Prokuplje");
        from_nis.push("Leskovac");
        from_nis.push("Jagodina");

        from_prokuplje.push("Nis");
        from_prokuplje.push("Leskovac");
        from_prokuplje.push("Krusevac");

        from_pirot.push("Nis");
        from_pirot.push("Leskovac");

        from_leskovac.push("Pirot");
        from_leskovac.push("Nis");
        from_leskovac.push("Vranje");
        from_leskovac.push("Prokuplje");

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
        from_kosovskamitrovica.push("Novi Pazar");

        from_gnjilane.push("Pristina");
        from_gnjilane.push("Vranje");

        from_novipazar.push("Kosovska Mitrovica");
        from_novipazar.push("Kraljevo");
        from_novipazar.push("Uzice");

        cities.insert("Belgrade", City::new("Belgrade", 0.000, 0.195, from_belgrade.clone()));
        cities.insert("Pancevo", City::new("Pancevo", 0.030, 0.225, from_pancevo.clone()));
        cities.insert("Sabac", City::new("Sabac", -0.100, 0.195, from_sabac.clone()));
        cities.insert("Valjevo", City::new("Valjevo", -0.085, 0.075, from_valjevo.clone()));
        cities.insert("Sremska Mitrovica",City::new("Sremska Mitrovica", -0.100,0.240,from_sremskamitrovica.clone()));
        cities.insert("Smederevo",City::new("Smederevo", 0.080,0.170,from_smederevo.clone()));
        cities.insert("Arandjelovac",City::new("Arandjelovac", 0.000,0.070,from_arandjelovac.clone()));
        cities.insert("Novi Sad",City::new("Novi Sad", -0.085, 0.320, from_novisad.clone()));
        cities.insert("Sombor",City::new("Sombor", -0.150, 0.440, from_sombor.clone()));
        cities.insert("Subotica",City::new("Subotica", -0.090, 0.470, from_subotica.clone()));
        cities.insert("Kikinda",City::new("Kikinda", 0.060, 0.430, from_kikinda.clone()));
        cities.insert("Cacak",City::new("Cacak", -0.040, -0.010, from_cacak.clone()));
        cities.insert("Kraljevo",City::new("Kraljevo", 0.010, -0.050, from_kraljevo.clone()));
        cities.insert("Zrenjanin",City::new("Zrenjanin", 0.080, 0.320, from_zrenjanin.clone()));
        cities.insert("Pozarevac",City::new("Pozarevac", 0.130,0.160, from_pozarevac.clone()));
        cities.insert("Kragujevac",City::new("Kragujevac", 0.080, 0.010, from_kragujevac.clone()));
        cities.insert("Jagodina",City::new("Jagodina", 0.130, 0.000, from_jagodina.clone()));
        cities.insert("Bor",City::new("Bor", 0.260, 0.010, from_bor.clone()));
        cities.insert("Zajecar",City::new("Zajecar", 0.290, -0.010, from_zajecar.clone()));
        cities.insert("Uzice",City::new("Uzice", -0.105, -0.020, from_uzice.clone()));
        cities.insert("Krusevac",City::new("Krusevac", 0.130, -0.070, from_krusevac.clone()));
        cities.insert("Nis",City::new("Nis", 0.210, -0.150, from_nis.clone()));
        cities.insert("Prokuplje",City::new("Prokuplje", 0.180, -0.150, from_prokuplje.clone()));
        cities.insert("Pirot",City::new("Pirot", 0.320, -0.170, from_pirot.clone()));
        cities.insert("Leskovac",City::new("Leskovac", 0.230, -0.220, from_leskovac.clone()));
        cities.insert("Vranje",City::new("Vranje", 0.210, -0.370, from_vranje.clone()));
        cities.insert("Pristina",City::new("Pristina", 0.105, -0.330, from_pristina.clone()));
        cities.insert("Pec",City::new("Pec", -0.080, -0.330, from_pec.clone()));
        cities.insert("Prizren",City::new("Prizren", 0.010, -0.430, from_prizren.clone()));
        cities.insert("Kosovska Mitrovica",City::new("Kosovska Mitrovica", 0.050, -0.230, from_kosovskamitrovica.clone()));
        cities.insert("Gnjilane",City::new("Gnjilane", 0.180, -0.390, from_gnjilane.clone()));
        cities.insert("Novi Pazar",City::new("Novi Pazar", 0.000, -0.150, from_novipazar.clone()));

        for (name, _) in &cities {
            city_states.insert(*name, CityState::Default);
            is_capital.insert(*name, false);
        }
        SerbiaMap {
            cities,
            city_states,
            is_capital,
            eligible_cities_p: HashSet::new(),
            eligible_cities_b: HashSet::new(),
            player: true,
            first_turn: true,
            show_question: false,
            correct_p: false,
            war_phase: 0,
            bot_attacked: false,
            attacked_city: "",
            rng: rand::thread_rng(),
            difficulty: 0.5,
            num_turns: 0,
            max_turns: 25,
            winner: "",
            player_defended: 0,
            bot_defended: 0,
            player_capital_health: 3,
            bot_capital_health: 3,
            bot_gain_health_try: false,
            bot_gained_health: false,
            capital_attacked: false,
            rng_gen: false,
            gain_health: 1.0,
        }
    }
    pub fn draw(&mut self, ui: &mut Ui, panel_size: Pos2) {
        let radius = 10.0;
        for (name1, city1) in &self.cities {
            for name2 in &city1.connected_to {
                let city_position1 = Self::normalize(panel_size, city1.position);
                let city_position2 = Self::normalize(panel_size, self.cities[name2].position);
                let color =
                    if self.city_states[name1] == self.city_states[name2] && (self.city_states[name1] != CityState::Default || self.city_states[name1] != CityState::Hovered) {
                        if self.city_states[name1] == CityState::Player {
                            Color32::from_rgb(50, 50, 255)
                        } else if self.city_states[name1] == CityState::Bot {
                            Color32::from_rgb(255,50,50)
                        } else {
                            Color32::from_rgb(50, 100, 50)
                        }
                    } else if self.war_phase == 32 {
                        if (self.city_states[name1] == CityState::Bot && self.city_states[name2] == CityState::Hovered) || (self.city_states[name2] == CityState::Bot && self.city_states[name1] == CityState::Hovered) {
                            Color32::from_rgb(255,50,50)
                        } else {
                            Color32::from_rgb(200, 0, 200)
                        }
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
            let city_position = Self::normalize(panel_size, city.position);
            let node_rect = Rect::from_center_size(city_position, Vec2::from([radius * 2.0; 2]));
            let is_hovered = ui
                .interact(node_rect, egui::Id::new(*name), Sense::hover())
                .hovered();
            let is_clicked = ui
                .interact(node_rect, egui::Id::new(*name), Sense::click())
                .clicked();
            let is_eligible = self.eligible_cities_p.contains(name) || self.eligible_cities_p.is_empty();

            let new_state = match (state, is_hovered, is_clicked, is_eligible, self.war_phase) {
                (CityState::Clicked, _, _, _, 32) => CityState::Clicked,
                (CityState::Bot, true, _, true, 32) => {
                    if self.player {
                        CityState::Hovered
                    } else {
                        CityState::Bot
                    }
                },
                (CityState::Bot, _, true, true, 32) => {
                    if self.player {
                        CityState::Clicked
                    } else {
                        CityState::Bot
                    }
                },
                (CityState::Player, _, _, _, _) => CityState::Player,
                (CityState::Bot, _, _, _, _) => CityState::Bot,
                (_, true, false, true, _) => CityState::Hovered, // Change to hovered if hovered
                (_, _, true, true, _) => CityState::Clicked, // Change to clicked if clicked
                (CityState::Hovered, _, _, true, 32) => CityState::Bot,
                _ => CityState::Default, // Default state otherwise
            };
            self.city_states.insert(*name, new_state);

            if self.winner != "" {
                self.player = false;
                if self.bot_capital_health == 0 {
                    self.city_states.insert(*name, CityState::Player);
                } else if self.player_capital_health == 0 {
                    self.city_states.insert(*name, CityState::Bot);
                }
            } else if self.num_turns == self.max_turns {
                self.player = false;
                let mut player_territories = 0;
                let mut bot_territories = 0;
                for (_name, state) in &self.city_states {
                    if *state == CityState::Player {
                        player_territories += 1;
                    } else if *state == CityState::Bot {
                        bot_territories += 1;
                    }
                }
                if player_territories > bot_territories {
                    self.winner = "Player";
                } else if bot_territories > player_territories {
                    self.winner = "Bot";
                } else {
                    if self.player_defended > self.bot_defended {
                        self.winner = "Player";
                    } else if self.bot_defended > self.player_defended {
                        self.winner = "Bot";
                    } else {
                        self.winner = "Draw";
                    }
                }
            } else if self.war_phase == 32 { // FAZA RATOVANJA
                if self.player {
                    if state == CityState::Player {
                        SerbiaMap::neighbors(city.clone(), CityState::Bot, &self.city_states, &mut self.eligible_cities_p);
                        for c in &self.eligible_cities_p {
                            if self.city_states[c] == CityState::Clicked {
                                self.show_question = true;
                            }
                        }
                    }
                } else {
                    if !self.rng_gen {
                        self.gain_health = self.rng.gen();
                        self.rng_gen = true;
                    }
                    if self.gain_health < 0.04 && self.bot_capital_health < 4 {
                        self.bot_gain_health_try = true;
                        let x: f64 = self.rng.gen();
                        if x < self.difficulty {
                            self.bot_capital_health += 1;
                            self.bot_gained_health = true;
                        } else {
                            self.bot_gained_health = false;
                        }
                        self.player = true;
                        self.rng_gen = false;
                        self.num_turns += 1;
                    } else {
                        if state == CityState::Bot {
                            SerbiaMap::neighbors(city.clone(), CityState::Player, &self.city_states, &mut self.eligible_cities_b);
                        }
                        if !self.bot_attacked {
                            let mut temp = Vec::new();
                            for c in &self.eligible_cities_b {
                                temp.push(*c);
                            }
                            if !temp.is_empty() {
                                temp.shuffle(&mut self.rng);
                                let x = temp[0];
                                self.city_states.insert(x, CityState::Clicked);
                                self.bot_attacked = true;
                                self.attacked_city = x;
                            }
                        }
                    }
                }
            } else { // FAZA ODABIRA
                if self.player { // IGRAC
                    if self.eligible_cities_p.is_empty() { // AKO NEMA SLOBODNIH POLJA
                        if new_state == CityState::Clicked {  // AKO JE KLIKNUTO BILO KOJE POLJE
                            if self.first_turn { // PRVI POTEZ
                                self.city_states.insert(*name, CityState::Player);
                                self.war_phase = self.war_phase + 1;
                                self.is_capital.insert(*name, true);
                                for neighbor in &city.connected_to {
                                    if self.city_states[*neighbor] != CityState::Bot && self.city_states[*neighbor] != CityState::Player {
                                        self.eligible_cities_p.insert(*neighbor);
                                    }
                                }
                            } else { // BILO KOJI POTEZ
                                self.city_states.insert(*name, CityState::Player);
                                self.war_phase = self.war_phase + 1;
                                for neighbor in &city.connected_to {
                                    if self.city_states[*neighbor] != CityState::Bot && self.city_states[*neighbor] != CityState::Player {
                                        self.eligible_cities_p.insert(*neighbor);
                                    }
                                }
                                self.eligible_cities_p.remove(*name);
                                self.eligible_cities_b.remove(*name);
                            }
                            self.player = false;
                        }
                    } else { // AKO IMA SLOBODNIH POLJA
                        if new_state == CityState::Clicked && self.eligible_cities_p.contains(name) { // AKO JE KLIKNUTO SLOBODNO POLJE
                            //self.show_question = true;
                            self.city_states.insert(*name, CityState::Player);
                            self.war_phase = self.war_phase + 1;
                            for neighbor in &city.connected_to {
                                if self.city_states[*neighbor] != CityState::Bot && self.city_states[*neighbor] != CityState::Player {
                                    self.eligible_cities_p.insert(*neighbor);
                                }
                            }
                            self.eligible_cities_p.remove(*name);
                            self.eligible_cities_b.remove(*name);

                            self.player = false;
                        }
                    }
                    // uslovi: CityState::Clicked, susedi su od CityState::Player
                    // dopustena polja: zuta boja
                } else { // BOT GRANA
                    if self.eligible_cities_b.is_empty() { // AKO NEMA SLOBODNIH POLJA
                        if state == CityState::Default { // BILO KOJE POLJE
                            if self.first_turn { // PRVI POTEZ
                                let mut player_capital = "";
                                for (n, b) in &self.is_capital {
                                    if *b {
                                        player_capital = *n;
                                    }
                                }
                                let forbidden_cities = self.cities[player_capital].connected_to.clone();
                                if !forbidden_cities.contains(name) {
                                    self.war_phase = self.war_phase + 1;
                                    self.city_states.insert(*name, CityState::Bot);
                                    self.is_capital.insert(*name, true);
                                    for neighbor in &city.connected_to {
                                        if self.city_states[*neighbor] != CityState::Bot && self.city_states[*neighbor] != CityState::Player {
                                            self.eligible_cities_b.insert(*neighbor);
                                        }
                                    }
                                    self.first_turn = false;
                                    self.player = true;
                                }
                            } else {
                                self.city_states.insert(*name, CityState::Bot);
                                self.war_phase = self.war_phase + 1;
                                for neighbor in &city.connected_to {
                                    if self.city_states[*neighbor] != CityState::Bot && self.city_states[*neighbor] != CityState::Player {
                                        self.eligible_cities_b.insert(*neighbor);
                                    }
                                }
                                self.eligible_cities_p.remove(*name);
                                self.eligible_cities_b.remove(*name);
                                self.player = true;
                            }
                            //println!("nema {}", self.eligible_cities_b.len());
                        }
                    } else { // IMA SLOBODNIH POLJA
                        if state == CityState::Default && self.eligible_cities_b.contains(name) { // IZABRANO SLOBODNO POLJE
                            self.city_states.insert(*name, CityState::Bot);
                            self.war_phase = self.war_phase + 1;
                            for neighbor in &city.connected_to {
                                if self.city_states[*neighbor] != CityState::Bot && self.city_states[*neighbor] != CityState::Player {
                                    self.eligible_cities_b.insert(*neighbor);
                                }
                            }
                            self.eligible_cities_p.remove(*name);
                            self.eligible_cities_b.remove(*name);
                            self.player = true;
                        }
                    }
                }
            }
            let color = match new_state {
                CityState::Player => {
                    if self.is_capital[*name] {
                        Color32::from_rgb(0, 0, 200)
                    } else {
                        Color32::from_rgb(50, 50, 255)
                    }
                },
                CityState::Bot => {
                    if self.is_capital[*name] {
                        Color32::from_rgb(200,0,0)
                    } else {
                        Color32::from_rgb(255,50,50)
                    }
                },
                CityState::Hovered =>
                    if self.war_phase == 32 {
                        Color32::from_rgb(200, 0, 200)
                    } else {
                        Color32::from_rgb(200, 100, 100)
                    },
                _ => {
                    if self.eligible_cities_p.contains(name) || self.eligible_cities_p.is_empty() {
                        Color32::from_rgb(200, 200, 0)
                    } else {
                        Color32::from_rgb(100, 200, 100)
                    }
                }
            };
            let custom_name = if self.is_capital[*name] {
                if self.city_states[*name] == CityState::Player || (!self.player && self.city_states[*name] == CityState::Clicked && self.war_phase == 32) {
                    format!("{} ({}/4)", city.name, self.player_capital_health)
                } else {
                    format!("{} ({}/4)", city.name, self.bot_capital_health)
                }
            } else {
                String::from(city.name)
            };
            ui.painter().circle_filled(city_position, radius, color);
            ui.painter()
                .text(Pos2::new(city_position.x, city_position.y - 20.0), egui::Align2::CENTER_CENTER, custom_name, FontId::monospace(12.0), color);
        }
    }
}
