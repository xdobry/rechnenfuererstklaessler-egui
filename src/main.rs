use eframe::egui;
use eframe::egui::Sense;
use eframe::egui::Vec2;
use std::time::Duration;
use std::time::Instant;

mod task;

fn main() -> Result<(), eframe::Error> {
    // Create the application
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rechnen Für Erstklässler", // App title
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

// Define a global constant array of strings
const EXERCISE_TITLES: &[&str] = &[
    "1+2", "6+7", "1+?=4", "6+?=14", "5-2", "16-7", "10-?=4", "14-?=6", "Mix",
];

enum TaskScore {
    ScorePerfect,
    ScoreSecond,
    ScoreWrong,
}

// Define the application structure
struct MyApp {
    exercise: i32,
    task_type: task::TaskType,
    task: task::Task,
    user_result: i32,
    ok_results: i32,
    wrong_results: i32,
    result_time: Option<Instant>,
    task_scores: Vec<TaskScore>,
    tries: i32,
    start_time: Instant,
    has_abacus_help: bool,
    show_abacus: bool,
    abacus1: Abacus,
    abacus2: Abacus,
    show_dialog: bool,
    show_message: String,
}

struct Abacus {
    red: i32,
    blue: i32,
    red2: i32,
}

impl Abacus {
    fn set_red_blue_red(&mut self, red: i32, blue: i32, red2: i32) {
        self.red = red;
        self.blue = blue;
        self.red2 = red2;
    }
}

// Implement default values for MyApp
impl Default for MyApp {
    fn default() -> Self {
        Self {
            exercise: -1,
            task: task::Task::gen_task(task::TaskType::Add10),
            user_result: 0,
            task_type: task::TaskType::Add10,
            ok_results: 0,
            wrong_results: 0,
            result_time: None,
            task_scores: Vec::new(),
            tries: 0,
            start_time: Instant::now(),
            has_abacus_help: false,
            show_abacus: false,
            abacus1: Abacus {
                red: 0,
                blue: 0,
                red2: 0,
            },
            abacus2: Abacus {
                red: 0,
                blue: 0,
                red2: 0,
            },
            show_dialog: false,
            show_message: "".to_string(),
        }
    }
}

impl MyApp {
    fn start_exercise(&mut self, task_type: task::TaskType) {
        self.task_type = task_type;
        self.wrong_results = 0;
        self.ok_results = 0;
        self.task_scores.clear();
        self.new_task();
    }

    fn new_task(&mut self) {
        let mut new_task = task::Task::gen_task(self.task_type);
        let mut i = 0;
        while new_task == self.task && i < 10 {
            new_task = task::Task::gen_task(self.task_type);
            i += 1;
        }
        self.start_time = Instant::now();
        self.tries = 0;
        self.show_abacus = false;
        self.has_abacus_help = false;
        self.abacus1.set_red_blue_red(0, 0, 0);
        self.abacus2.set_red_blue_red(0, 0, 0);
        self.task = new_task;
        self.user_result = 0;
        self.result_time = None;
    }

    fn get_score(&self) -> f32 {
        100.0 * self.ok_results as f32 / (self.wrong_results + self.ok_results) as f32
    }

    fn get_final_message(&self) -> String {
        let score = self.get_score();
        if score >= 100.0 {
            "😀 Perfekt! Kein Fehler".to_string()
        } else if score >= 90.0 {
            format!(
                "😃 Aufgabe beendet richtig: {} falsch: {}",
                self.ok_results, self.wrong_results
            )
        } else if score >= 70.0 {
            format!(
                "😒 Aufgabe beendet richtig: {} falsch: {}",
                self.ok_results, self.wrong_results
            )
        } else if score >= 50.0 {
            format!(
                "😓 Aufgabe beendet richtig: {} falsch: {}",
                self.ok_results, self.wrong_results
            )
        } else if score >= 40.0 {
            format!(
                "😞 Aufgabe beendet richtig: {} falsch: {}",
                self.ok_results, self.wrong_results
            )
        } else if score >= 20.0 {
            format!(
                "😢 Aufgabe beendet richtig: {} falsch: {}",
                self.ok_results, self.wrong_results
            )
        } else {
            "🐵 War da ein Affe dran?".to_string()
        }
    }

    fn was_partially_result(&self) -> bool {
        self.task.task_type.get_max_sum() > 10
            && self.user_result <= self.task.task_type.get_max_sum() / 10
            && self.user_result > 0
    }

    fn is_final_result(&self) -> bool {
        self.task.task_type.get_max_sum() <= 10
            || self.user_result > self.task.task_type.get_max_sum() / 10
            || (self.user_result <= self.task.task_type.get_max_sum() / 10
                && self.task.check_result(self.user_result))
    }

    fn is_partially_result(&self) -> bool {
        self.user_result > 0
            && self.task.task_type.get_max_sum() > 10
            && self.user_result <= self.task.task_type.get_max_sum() / 10
            && !self.task.check_result(self.user_result)
    }

    fn click_number(&mut self, number: i32, ctx: &egui::Context) {
        if self.result_time.is_none() {
            self.user_result = if self.was_partially_result() {
                self.user_result * 10 + number
            } else {
                number
            };
            if self.task.check_result(self.user_result) {
                self.result_time = Some(Instant::now());
                if self.tries == 0 {
                    self.ok_results += 1;
                    if !self.show_abacus && !self.has_abacus_help {
                        self.task_scores.push(TaskScore::ScorePerfect);
                    } else {
                        self.task_scores.push(TaskScore::ScoreSecond);
                    }
                }
                ctx.request_repaint_after(Duration::from_millis(1000));
            } else {
                if self.is_final_result() {
                    if self.tries == 0 {
                        self.wrong_results += 1;
                        self.task_scores.push(TaskScore::ScoreWrong);
                        self.abacus_help();
                    } else {
                        if self.tries >= 3 {
                            self.user_result = self.task.user_expected_result();
                            self.result_time = Some(Instant::now());
                            ctx.request_repaint_after(Duration::from_millis(1000));
                        }
                    }
                    self.tries = self.tries + 1;
                }
            }
        }
    }

    fn display_abacus(&mut self) {
        self.show_abacus = true;
        match self.task_type {
            task::TaskType::Add10 | task::TaskType::Add20 | task::TaskType::Sub10 => {
                self.abacus1.set_red_blue_red(self.task.parameter1, 0, 0);
                self.abacus2.set_red_blue_red(0, self.task.parameter2, 0);
            }
            task::TaskType::AddQuest10 | task::TaskType::AddQuest20 => {
                self.display_abacus_minus(self.task.task_result, self.task.parameter1);
            }
            task::TaskType::Sub20 => {
                self.display_abacus_minus(self.task.parameter1, self.task.task_result);
            }
            task::TaskType::SubQuest10 | task::TaskType::SubQuest20 => {
                self.display_abacus_minus(self.task.parameter1, self.task.task_result);
            }
            _ => {}
        }
    }

    fn abacus_help(&mut self) {
        self.has_abacus_help = true;
        self.show_abacus = true;
        match self.task_type {
            task::TaskType::Sub10 => {
                self.display_abacus_minus(self.task.parameter1, self.task.task_result);
                return;
            }
            task::TaskType::Add10 | task::TaskType::Add20 => {}
            _ => {
                self.display_abacus();
                return;
            }
        }
        if self.task.parameter1 <= 5 && self.task.parameter2 <= 5 {
            if self.task.parameter1 >= self.task.parameter2 {
                let to_move = (5 - self.task.parameter1).min(self.task.parameter2);
                self.abacus1
                    .set_red_blue_red(self.task.parameter1, to_move, 0);
                self.abacus2
                    .set_red_blue_red(0, self.task.parameter2 - to_move, 0);
            } else {
                let to_move = (5 - self.task.parameter2).min(self.task.parameter1);
                self.abacus1
                    .set_red_blue_red(self.task.parameter1 - to_move, 0, 0);
                self.abacus2
                    .set_red_blue_red(0, self.task.parameter2, to_move);
            }
        } else {
            if self.task.parameter1 >= self.task.parameter2 {
                let to_move = (10 - self.task.parameter1).min(self.task.parameter2);
                self.abacus1
                    .set_red_blue_red(self.task.parameter1, to_move, 0);
                self.abacus2
                    .set_red_blue_red(0, self.task.parameter2 - to_move, 0);
            } else {
                let to_move = (10 - self.task.parameter2).min(self.task.parameter1);
                self.abacus1
                    .set_red_blue_red(self.task.parameter1 - to_move, 0, 0);
                self.abacus2
                    .set_red_blue_red(0, self.task.parameter2, to_move);
            }
        }
    }

    fn display_abacus_minus(&mut self, sum: i32, par: i32) {
        self.abacus1
            .set_red_blue_red(par, (sum - par).min(10 - par), 0);
        self.abacus2.set_red_blue_red(0, (sum - 10).max(0), 0);
    }

    fn draw_abacus(&mut self, ui: &mut egui::Ui) -> bool {
        let radius = 32.0;
        let gap = 8.0;
        let size = Vec2::new(10.0 * radius + 9.0 * gap + gap, radius * 2.0 + gap);
        let (id, rect) = ui.allocate_space(size);
        let painter = ui.painter();
        let draw_raw = |abacus: &Abacus, yoffset: f32| {
            for i in 0..10 {
                let add_gap = if i >= 5 { gap } else { 0.0 };
                let x = (i as f32) * (radius + gap) + add_gap + rect.left() + radius / 2.0;
                let y = rect.top() + yoffset + radius / 2.0;
                let (fill, color) = if i < abacus.red {
                    (true, egui::Color32::from_rgb(255, 0, 0))
                } else if i < abacus.blue + abacus.red {
                    (true, egui::Color32::from_rgb(0, 0, 255))
                } else if i < abacus.blue + abacus.red + abacus.red2 {
                    (true, egui::Color32::from_rgb(255, 0, 0))
                } else {
                    (false, egui::Color32::from_rgb(255, 0, 0))
                };
                if fill {
                    painter.circle_filled(
                        egui::pos2(x, y), // Position of the circle's center
                        radius / 2.0,     // Radius of the circle
                        color,            // Circle color (white)
                    );
                } else {
                    painter.circle_stroke(
                        egui::pos2(x, y),   // Position of the circle's center
                        radius / 2.0 - 2.0, // Radius of the circle
                        egui::Stroke {
                            width: 4.0,                  // Outline width
                            color: egui::Color32::BLACK, // Outline color (black)
                        },
                    );
                }
            }
        };
        draw_raw(&self.abacus1, 0.0);
        draw_raw(&self.abacus2, radius + gap);
        return ui.allocate_rect(rect, Sense::click()).clicked();
    }
}

// Implement the main GUI logic
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.exercise < 0 {
                ui.heading("Rechnen für Erstklässler");
                ui.horizontal(|ui| {
                    for (index, &label) in EXERCISE_TITLES.iter().enumerate() {
                        if ui.button(label).clicked() {
                            self.exercise = index as i32;
                            self.task_type = task::TaskType::from(self.exercise);
                            self.start_exercise(self.task_type);
                        }
                    }
                });
            } else {
                match self.result_time {
                    Some(result_time) => {
                        let ellapsed: u64 = result_time.elapsed().as_millis() as u64;
                        if ellapsed >= 1000 {
                            if self.wrong_results + self.ok_results >= 10 {
                                self.show_dialog = true;
                                self.show_message = self.get_final_message();
                            } else {
                                self.new_task();
                            }
                        } else {
                            ctx.request_repaint_after(Duration::from_millis(1000 - ellapsed));
                        }
                    }
                    _ => {}
                }
                ui.label(format!(
                    "Exercise: {}",
                    EXERCISE_TITLES[self.exercise as usize]
                ));
                ui.horizontal(|ui| {
                    let result = if self.task.task_type.is_parameter_quest() {
                        &self.task.task_result.to_string()
                    } else {
                        if self.user_result > 0 {
                            if self.is_partially_result() {
                                &format!("{}?", self.user_result)
                            } else {
                                &self.user_result.to_string()
                            }
                        } else {
                            "?"
                        }
                    };
                    let par2 = if self.task.task_type.is_parameter_quest() {
                        if self.user_result > 0 {
                            if self.is_partially_result() {
                                &format!("{}?", self.task.parameter2)
                            } else {
                                &self.user_result.to_string()
                            }
                        } else {
                            "?"
                        }
                    } else {
                        &self.task.parameter2.to_string()
                    };
                    let is_wrong = self.user_result > 0 && !self.task.check_result(self.user_result) && self.is_final_result();
                    ui.horizontal(|ui| {
                        ui.label(self.task.parameter1.to_string());
                        ui.label(self.task.task_type.task_op());
                        if self.task.task_type.is_parameter_quest() && is_wrong {
                            ui.colored_label(egui::Color32::RED,par2);
                        } else {
                            ui.label(par2);
                        }
                        ui.label("=");
                        if !self.task.task_type.is_parameter_quest() && is_wrong {
                            ui.colored_label(egui::Color32::RED,result);
                        } else {
                            ui.label(result);    
                        }
                    });
                });
                ui.horizontal(|ui| {
                    for i in 1..=5 {
                        if ui.button(i.to_string()).clicked() {
                            self.click_number(i, ctx);
                        }
                    }
                });
                ui.horizontal(|ui| {
                    for i in 6..=10 {
                        let realnumber = if self.task.task_type.get_max_sum() > 10 && i == 10 {
                            0
                        } else {
                            i
                        };
                        if ui.button(realnumber.to_string()).clicked() {
                            self.click_number(realnumber, ctx);
                        }
                    }
                    if self.task.task_type.get_max_sum() > 10 {
                        if ui.button("\u{232B}").clicked() {
                            self.user_result = 0;
                        }
                    }
                });
                if !self.show_abacus &&!self.show_dialog {
                    let elapsed_start_time = self.start_time.elapsed();
                    if  elapsed_start_time >= self.task.task_type.timeDisplayAbacus()
                    {
                        self.display_abacus();
                    } else {
                        ctx.request_repaint_after(self.task.task_type.timeDisplayAbacus()-elapsed_start_time);
                    }   
                }
                if self.draw_abacus(ui) {
                    if !self.has_abacus_help {
                        self.abacus_help();
                    }
                }
                ui.horizontal(|ui| {
                    for score in self.task_scores.iter() {
                        match score {
                            TaskScore::ScorePerfect => {
                                ui.label("✅");
                            }
                            TaskScore::ScoreSecond => {
                                ui.label("⏳");
                            }
                            TaskScore::ScoreWrong => {
                                ui.label("❌");
                            }
                        }
                    }
                });
                if ui.button("back").clicked() {
                    self.exercise = -1;
                }
                if self.show_dialog {
                    egui::Window::new("Aufgabe beendet")
                        .collapsible(false) // Prevent collapsing
                        .resizable(false) // Prevent resizing
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Center the dialog
                        .show(ctx, |ui| {
                            ui.label(&self.show_message);
                            if ui.button("OK").clicked() {
                                self.show_dialog = false;
                                self.exercise = -1;
                            }
                        });
                }            
            }
        });
    }
}
