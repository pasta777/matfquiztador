mod map;

use eframe::{egui};
use egui::{Align, Layout, Pos2};
use htmlentity::entity::decode;
use serde::Deserialize;
use tokio::runtime;
use htmlentity::entity::ICodedDataTrait;
use rand::Rng;
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
            Box::new(MyApp::new())
        }),
    )
}
/*
#[derive(Debug,Deserialize)]
struct SessionToken {
    response_code: i32,
    response_message: String,
    token: String
}

#[derive(Debug,Deserialize)]
struct Question {
    r#type: String,
    difficulty: String,
    category: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>
}

#[derive(Debug,Deserialize)]
struct QuestionsBatch {
    response_code: i32,
    results: Vec<Question>
}
*/
struct MyApp {
    //rt: runtime::Runtime, <- koristim za pravljenje asinhronih blokova
    show_confirmation_dialog: bool,
    main_menu: bool,
    playing: bool,
    serbia_map: SerbiaMap,
    test_question: u32,
    confirm: bool,
    bot_correct: bool,
    player_correct: bool,
    settings: bool,
    question_generated: bool,
    correct_answer: u8,
    /* Ovo je sve rezervisano za pitanja
    session_token_url: String,
    question_vector: Vec<Question>,
    question_generated: bool,
    question_text: String,
    correct_answer: String,
    incorrect_answers: Vec<String>
    */
}
impl MyApp {
    fn new() -> Self {
        Self {
            //rt: runtime::Builder::new_multi_thread().enable_all().build().unwrap(),
            show_confirmation_dialog: false,
            main_menu: true,
            playing: false,
            serbia_map: SerbiaMap::new(),
            test_question: 1,
            confirm: false,
            bot_correct: false,
            player_correct: false,
            settings: false,
            question_generated: false,
            correct_answer: 1,
            /*
            session_token_url: String::new(),
            question_vector: Vec::new(),
            question_generated: false,
            question_text: String::new(),
            correct_answer: String::new(),
            incorrect_answers: Vec::new()
            */
        }
    }
    /*async fn create_session_token() -> Result<String, Box<dyn std::error::Error + 'static>> {
        let mut response = reqwest::get("https://opentdb.com/api_token.php?command=request").await?;

        // Check if the request was successful (status code 200)
        if response.status().is_success() {
            // Read the response body as text
            let body = response.text().await?;
            let data: SessionToken = serde_json::from_str(&body)?;
            let session_token_real = data.token;

            // Build the URL with the session token
            let url: &str = "https://opentdb.com/api.php?amount=50&type=multiple&token=";
            let combined: String = format!("{}{}", url, session_token_real);
            Ok(combined)
        } else {
            println!("Request failed with status code: {}", response.status());
            Err("Request failed".into())
        }
    }
    fn spawn_session_token(&mut self, stu: &mut String) {
        self.rt.spawn(async move {
            match Self::create_session_token().await {
                Ok(session_token) => *stu = session_token.clone(),
                Err(e) => println!("Error occured: {}", e.to_string())
            }
        });
    }
    fn generate_question(index: usize, question_text: &mut String, correct_answer_text: &mut String,
                         incorrect_answers_text: &mut Vec<String>, question_vector: &mut Vec<Question>) {
        let pulled_question = decode(question_vector[index].question.as_ref()).to_string();
        let pulled_correct = decode(question_vector[index].correct_answer.as_ref()).to_string();
        let pulled_incorrects = question_vector[index].incorrect_answers.clone();
        for ia in pulled_incorrects {
            let ignus_fatus = decode(ia.as_ref()).to_string();
            match ignus_fatus {
                Ok(decoded_incorrect) => { incorrect_answers_text.push(decoded_incorrect); },
                Err(e) => { eprintln!("Decoding Error {}", e); }
            }
        }
        match pulled_question {
            Ok(decoded_question) => { *question_text = decoded_question; },
            Err(e) => { eprintln!("Decoding Error {}", e); }
        }
        match pulled_correct {
            Ok(decoded_correct) => { *correct_answer_text = decoded_correct; },
            Err(e) => { eprintln!("Decoding Error {}", e); }
        }
        question_vector.remove(index);
    }
    async fn fill_question_vector(session_url: String) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
        let response = reqwest::get(&session_url).await?;

        if response.status().is_success() {
            let body = response.text().await?;
            let data: QuestionsBatch = serde_json::from_str(&body)?;

            // Extract the vector of questions from the data
            let questions_vector = data.results;

            Ok(questions_vector)
        } else {
            println!("Request failed with status code: {}", response.status());
            Err("Request failed".into())
        }
    }
    */
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
                        self.settings = true;
                        self.main_menu = false;
                    }
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        }
        if self.settings {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional)
                );
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Heading,
                    egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
                );
                for _i in 1..10 {
                    ui.label("");
                }
                ui.vertical_centered(|ui|{
                    ui.heading("Difficulty");
                    ui.radio_value(&mut self.serbia_map.difficulty, 0.25, "Easy");
                    ui.radio_value(&mut self.serbia_map.difficulty, 0.5, "Medium");
                    ui.radio_value(&mut self.serbia_map.difficulty, 0.75, "Hard");
                    ui.separator();
                    ui.heading("Max number of turns");
                    ui.horizontal(|ui| {
                        for _i in 1..85 {
                            ui.label("");
                        }
                        ui.add(egui::Slider::new(&mut self.serbia_map.max_turns, 10..=50));

                    });
                    ui.separator();
                    if ui.button("Confirm").clicked() {
                        self.settings = false;
                        self.main_menu = true;
                    }
                });
            });
        }
        if self.playing {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(20.0, eframe::epaint::FontFamily::Proportional),
                );
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Heading,
                    egui::FontId::new(25.0, eframe::epaint::FontFamily::Proportional),
                );
                let panel_size = ui.available_size();
                if self.serbia_map.first_turn {
                    ui.heading("Choose capital city.");
                }
                if self.serbia_map.winner != "" {
                    if self.serbia_map.winner != "Draw" {
                        ui.heading(format!("The winner is: {}", self.serbia_map.winner));
                    } else {
                        ui.heading("It's draw.");
                    }
                    if ui.button("Exit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                } else if self.serbia_map.war_phase == 32 {
                    ui.heading("War phase");
                } else {
                    ui.heading("Click on the free territory to claim it.");
                }
                if self.serbia_map.bot_attacked {
                    ui.heading(format!("The bot has attacked {}", self.serbia_map.attacked_city));
                    if ui.button("Confirm").clicked() {
                        self.serbia_map.show_question = true;
                    }
                } else if self.serbia_map.war_phase == 32 && self.serbia_map.winner == "" {
                    ui.heading("Click on a neighboring cities to attack.");
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
                        egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
                    );
                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Heading,
                        egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
                    );
                    if self.confirm {
                        if self.player_correct && self.bot_correct {
                            ui.heading("Both you and Bot answered correctly");
                        } else if self.player_correct {
                            if self.serbia_map.bot_attacked {
                                self.serbia_map.player_defended += 1;
                            }
                            ui.heading("You answered correctly, bot didn't");
                        } else if self.bot_correct {
                            if !self.serbia_map.bot_attacked {
                                self.serbia_map.bot_defended += 1;
                            }
                            ui.heading("Bot answered correctly, you didn't");
                        } else {
                            ui.heading("Both you and Bot answered incorrectly");
                        }
                        if ui.button("Confirm").clicked() {
                            self.serbia_map.show_question = self.player_correct && self.bot_correct;
                            if !self.serbia_map.show_question {
                                if self.serbia_map.bot_attacked {
                                    self.serbia_map.player = true;
                                    self.serbia_map.bot_attacked = false;
                                } else {
                                    self.serbia_map.player = false;
                                }
                                self.serbia_map.num_turns += 1;
                            }
                            self.confirm = false;
                            self.test_question += 1; // novo pitanje
                            self.question_generated = false;
                        }
                    } else {
                        if !self.question_generated {
                            let mut rng = rand::thread_rng();
                            self.correct_answer = rng.gen_range(1..5);
                            self.question_generated = true;
                            println!("{}", self.correct_answer);
                        }
                        ui.heading(format!("The question placeholder {}", self.test_question));
                        if ui.button("Option 1").clicked() {
                            self.bot_correct = self.serbia_map.state_change(self.correct_answer == 1);
                            self.confirm = true;
                            self.player_correct = self.correct_answer == 1;
                            //self.question_generated = false;
                        }
                        if ui.button("Option 2").clicked() {
                            self.bot_correct = self.serbia_map.state_change(self.correct_answer == 2);
                            self.confirm = true;
                            self.player_correct = self.correct_answer == 2;
                            //self.question_generated = false;
                        }
                        if ui.button("Option 3").clicked() {
                            self.bot_correct = self.serbia_map.state_change(self.correct_answer == 3);
                            self.confirm = true;
                            self.player_correct = self.correct_answer == 3;
                            //self.question_generated = false;
                        }
                        if ui.button("Option 4").clicked() {
                            self.bot_correct = self.serbia_map.state_change(self.correct_answer == 4);
                            self.confirm = true;
                            self.player_correct = self.correct_answer == 4;
                            //self.question_generated = false;
                        }
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