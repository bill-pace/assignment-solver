use std::sync::Arc;
use eframe::egui;
use eframe::egui::panel::TopBottomSide;
use crate::io::FileType;
use crate::ui::{CurrentStatus, Status};
use crate::ui::model::Model;

pub struct Presenter {
    infile: Option<String>,
    outfile: Option<String>,
    cur_status: Arc<CurrentStatus>
}

impl Presenter {
    pub fn new(status_tracker: Arc<CurrentStatus>) -> Self {
        Presenter {
            infile: None,
            outfile: None,
            cur_status: status_tracker
        }
    }

    fn update_input_output(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::new(TopBottomSide::Top, "Select input and output files:")
            .show(ctx, |ui| {
                ui.label("Select an input file:");
                if ui.button("Select input file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.infile = Some(path.display().to_string());
                    }
                }
                if let Some(picked_path) = &self.infile {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                    });
                }

                ui.label("Select an output file:");
                if ui.button("Select output file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.outfile = Some(path.display().to_string());
                    }
                }
                if let Some(picked_path) = &self.outfile {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                    });
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Click here to solve").clicked() {
                self.start_solver_thread();
            }
        });
    }

    fn update_running(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame,
                          pct_complete: f32) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Running!");
        });
    }

    fn start_solver_thread(&self) {
        let infile = match &self.infile {
            Some(name) => name.to_string(),
            None => {
                self.cur_status.set_status(Status::Failure("You must select an input file!".to_string()));
                return;
            }
        };
        let outfile = match &self.outfile {
            Some(name) => name.to_string(),
            None => {
                self.cur_status.set_status(Status::Failure("You must select an output file!".to_string()));
                return;
            }
        };

        let status_tracker = self.cur_status.clone();
        std::thread::spawn(move || make_assignment(Model::new(FileType::CSV, FileType::CSV),
                                                   infile,
                                                   outfile,
                                                   status_tracker));
    }
}

impl eframe::App for Presenter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.cur_status.get_status() {
            Status::Success => {
                self.update_input_output(ctx, _frame)
            },
            Status::InProgress(pct) => {
                self.update_running(ctx, _frame, pct)
            },
            Status::Failure(msg) => {
                self.update_input_output(ctx, _frame)
            },
            Status::NotStarted => {
                self.update_input_output(ctx, _frame)
            }
        }
    }
}

fn make_assignment(model: Model, input_file: String, output_file: String,
                   status_tracker: Arc<CurrentStatus>) {
    model.assign_workers(input_file, output_file, status_tracker);
}
