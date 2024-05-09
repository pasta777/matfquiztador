mod map;

use eframe::egui;
use egui::{Align, Layout, Pos2};
use htmlentity::entity::decode;
use rand::Rng;
use serde::Deserialize;
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
    session_token_url: String,
    question_vector: Vec<Question>,
    question_generated: bool,
    question_text: String,
    correct_answer: String,
    incorrect_answers: Vec<String>
}

impl Default for MyApp {
    // ovo je default state (kada se ucita igrica itd)
    fn default() -> Self {
        Self {
            show_confirmation_dialog: false,
            main_menu: true,
            playing: false,
            serbia_map: SerbiaMap::new(),
            session_token_url: String::new(),
            question_vector: Vec::new(),
            question_generated: false,
            question_text: String::new(),
            correct_answer: String::new(),
            incorrect_answers: Vec::new()
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
                let url = tokio::runtime::Runtime::new().unwrap().block_on(async {
                    match CreateSessionToken() {
                        Ok(url) => Ok(url),
                        Err(err) => {
                            eprintln!("Error creating session token: {}", err);
                            Err("Error".to_string()) // Exit the function if an error occurs
                        }
                    }
                });

                ui.with_layout(Layout::top_down(Align::Center).with_cross_align(Align::Center), |ui| async {
                    // HACK ZA CENTRIRANJE TEKSTA NE KORISTITI NIGDE DRUGDE
                    // TREBA PRONACI BOLJI NACIN ALI TOP DOWN SA CROSS ALIGN NE RADI
                    // NE RADI NI KOMBINOVANJE HORIZONTAL CENTER I VERTICAL CENTER -- KNOWN BUG IN EGUI
                    for _i in 1..20 {
                        ui.label("");
                    }
                    // HACK SE OVDE ZAVRSAVA
                    match url {
                        Ok(session_token_url) => {
                            if ui.button("Play!").clicked() {
                                self.session_token_url = session_token_url;
                                self.main_menu = false;
                                self.playing = true;
                            }
                        }
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
                    if self.serbia_map.war_phase == 32 {
                        ui.heading("War phase");
                    }
                    self.serbia_map.draw(ui, Pos2::new(panel_size.x, panel_size.y));
                });
            }
        if self.serbia_map.show_question {
            if self.question_vector.is_empty() {
                self.question_vector = FillQuestionVector(self.session_token_url.clone()).await.unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    Vec::new()
                });
            }
            if !self.question_generated {
                let mut rng = rand::thread_rng();
                let random_number: usize = rng.gen_range(0..self.question_vector.len());
                GenerateQuestion(random_number, &mut self.question_text, &mut self.correct_answer, &mut self.incorrect_answers, &mut self.question_vector);
                self.question_generated = true;
            }
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
                    ui.heading(self.question_text.clone());
                    if ui.button("Option 1").clicked() {
                        self.serbia_map.state_change(false);
                        self.question_generated = false;
                    }
                    if ui.button("Option 2").clicked() {
                        self.serbia_map.state_change(true);
                        self.question_generated = false;
                    }
                    if ui.button("Option 3").clicked() {
                        self.serbia_map.state_change(false);
                        self.question_generated = false;
                    }
                    if ui.button("Option 4").clicked() {
                        self.serbia_map.state_change(false);
                        self.question_generated = false;
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

fn GenerateQuestion(index: usize, QuestionText: &mut String, CorrectAnswerText: &mut String,
                    IncorrectAnswersText: &mut Vec<String>, QuestionVector: &mut Vec<Question>) {
    let PulledQuestion = decode(QuestionVector[index].question.as_ref()).to_string();
    let PulledCorrect = decode(QuestionVector[index].correct_answer.as_ref()).to_string();
    let PulledIncorrects = QuestionVector[index].incorrect_answers.clone();
    for ia in PulledIncorrects {
        let ignusFatus = decode(ia.as_ref()).to_string();
        match ignusFatus {
            Ok(decoded_incorrect) => { IncorrectAnswersText.push(decoded_incorrect); },
            Err(e) => { eprintln!("Decoding Error {}", e); }
        }
    }
    match PulledQuestion {
        Ok(decoded_question) => { *QuestionText = decoded_question; },
        Err(e) => { eprintln!("Decoding Error {}", e); }
    }
    match PulledCorrect {
        Ok(decoded_correct) => { *CorrectAnswerText = decoded_correct; },
        Err(e) => { eprintln!("Decoding Error {}", e); }
    }
    QuestionVector.remove(index);
}


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

#[tokio::main]
async fn CreateSessionToken() -> Result<String, Box<dyn std::error::Error>> {
    let mut response = reqwest::get("https://opentdb.com/api_token.php?command=request").await?;

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Read the response body as text
        let body = response.text().await?;
        let data: SessionToken = serde_json::from_str(&body)?;
        let session_token_real = data.token;

        // Build the URL with the session token
        let URL: &str = "https://opentdb.com/api.php?amount=50&type=multiple&token=";
        let combined: String = format!("{}{}", URL, session_token_real);
        Ok(combined)
    } else {
        println!("Request failed with status code: {}", response.status());
        Err("Request failed".into())
    }
}


#[tokio::main]
async fn FillQuestionVector(SessionUrl: String) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
    let response = reqwest::get(&SessionUrl).await?;

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
