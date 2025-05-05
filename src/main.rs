#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

struct Category {
    name: &'static str,
    sub_features: Vec<&'static str>,
}

#[derive(Default)]
struct UI {
    categories: Vec<Category>,
    current_cat: Option<usize>,
    selected_feature: Option<String>,
    mode: String,
    inputs: Vec<String>,
    numbers: Vec<f64>,
    result: String,

    special: String
}

impl UI {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let uk = "Unknown".to_string();
        let category: Vec<Category> = vec![
            Category {
                name: "Basic",
                sub_features: vec!["Simple"],
            },
            Category {
                name: "Advanced",
                sub_features: vec!["Algebra", "Geometry"],
            },
        ];
        Self {
            categories: category,
            inputs: vec!["".to_string(), "".to_string()],
            numbers: vec![0.0, 0.0],
            mode: uk.clone(),
            special: uk.clone(),
            ..Default::default()
        }
    }
}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Category");
            ui.separator();

            for (i, cat) in self.categories.iter().enumerate() {
                let selected = Some(i) == self.current_cat;
                let response = ui.selectable_label(selected, cat.name);

                let popup_id = ui.make_persistent_id(format!("popup_{}", i));

                if response.clicked() {
                    self.current_cat = Some(i);
                    ctx.memory_mut(|mem| mem.open_popup(popup_id));
                }

                egui::popup_below_widget(
                    ui,
                    popup_id,
                    &response,
                    egui::PopupCloseBehavior::CloseOnClickOutside,
                    |ui| {
                        ui.set_min_width(150.0);
                        ui.label("Select sub-feature:");

                        if let Some(cat_idx) = self.current_cat {
                            let cat = &self.categories[cat_idx];
                            for &sub in &cat.sub_features {
                                let is_selected = self.selected_feature.as_deref() == Some(sub);
                                if ui.selectable_label(is_selected, sub).clicked() {
                                    self.selected_feature = Some(sub.to_string());
                                }
                            }
                        }

                        if ui.button("Close").clicked() {
                            ctx.memory_mut(|mem| mem.close_popup());
                        }
                    },
                );
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Selected Feature");
            ui.separator();

            fn num_converter(this: &mut UI) {
                for (num, string) in this.inputs.iter().enumerate() {
                    this.numbers[num] = match string.parse::<f64>() {
                        Ok(_) => string.parse().unwrap(),
                        Err(_) => 0.0
                    }
                }
            }

            if let Some(feat) = &self.selected_feature {
                ui.group(|ui| {
                    ui.label(feat);
                });

                fn but_mode_creator(ui: &mut egui::Ui, mode: &mut String, modes: &[&str]) {
                    for &m in modes {
                        if ui.button(m).clicked() {
                            *mode = m.to_string();
                        }
                    }
                }

                match feat.as_str() {
                    "Simple" => {
                        ui.label("Select Mode:");
                        ui.horizontal(|ui| {
                            let modes = vec!["Add", "Subtract", "Multiply", "Divide", "Pow", "Root"];
                            but_mode_creator(ui, &mut self.mode, &modes);
                        });
                        ui.label(format!("Current mode: {}", &self.mode));
                        ui.separator();
                        ui.label("Inputs:");
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.inputs[0]);
                            ui.text_edit_singleline(&mut self.inputs[1]);
                        });
                        ui.separator();
                        ui.label("Result:");
                        ui.horizontal(|ui| {
                            if ui.button("Get Result").clicked() {
                                if self.mode.is_empty() {
                                    self.result = "Unknown Mode".to_string();
                                } else {
                                    num_converter(self);
                                    self.result = format!("Result: {}", match self.mode.as_str() {
                                        "Add" => format!("{}", self.numbers[0] + self.numbers[1]),
                                        "Subtract" => format!("{}", self.numbers[0] - self.numbers[1]),
                                        "Multiply" => format!("{}", self.numbers[0] * self.numbers[1]),
                                        "Divide" => format!("{}", self.numbers[0] / self.numbers[1]),
                                        "Pow" => format!("{}", self.numbers[0].powf(self.numbers[1])),
                                        "Root" => format!("{}", self.numbers[0].powf(1.0 / self.numbers[1])),
                                        _ => "Wut?".to_string()
                                    });
                                }
                            }
                            ui.label(self.result.clone());
                        });
                        ui.separator();
                        ui.label("Insert result into...");
                        ui.horizontal(|ui| {
                            for i in 0..2 {
                                if ui.button(format!("Insert result into box {}", i + 1)).clicked() {
                                    self.inputs[i] = self.result.clone().replace("Result: ", "");
                                }
                            }
                        });
                        ui.separator();
                        ui.label("Special Numbers:");
                        ui.horizontal(|ui| {
                            for &label in ["PI", "e"].iter() {
                                if ui.button(label).clicked() {
                                    self.special = label.to_string();
                                }
                            }
                            ui.label(format!("Selected: {}", self.special));
                        });
                        ui.horizontal(|ui| {
                            for i in 0..self.inputs.len() {
                                if ui.button(format!("Insert Number to input box {}", i + 1)).clicked() {
                                    let v = &mut self.inputs[i];
                                    if self.special == "PI" {
                                        *v = std::f64::consts::PI.to_string();
                                    } else if self.special == "e" {
                                        *v = std::f64::consts::E.to_string();
                                    } else {
                                        *v = "0".to_string();
                                    }
                                }
                            }
                        });
                    }
                    
                    _ => {
                        ui.label("Unknown feature selected");
                    }
                }

            } else {
                ui.label("No feature selected");
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "MathHelper",
        options,
        Box::new(|cc| Ok(Box::new(UI::new(cc)))),
    );
}