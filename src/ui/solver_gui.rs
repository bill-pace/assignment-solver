use std::sync::Arc;
use eframe::egui;
use eframe::egui::Color32;
use eframe::egui::panel::TopBottomSide;
use crate::io::FileType;
use crate::ui::{CurrentStatus, Status};
use crate::ui::solver::Solver;

pub struct SolverGui {
    infile: Option<String>,
    outfile: Option<String>,
    cur_status: Arc<CurrentStatus>
}

impl SolverGui {
    pub fn new(status_tracker: Arc<CurrentStatus>) -> Self {
        SolverGui {
            infile: None,
            outfile: None,
            cur_status: status_tracker
        }
    }

    fn update_not_started(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut launch_frame = egui::containers::Frame::default();
        launch_frame.fill = Color32::LIGHT_GRAY;

        egui::TopBottomPanel::new(TopBottomSide::Top, "Select input and output files:")
            .frame(launch_frame.clone())
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| ui.heading("Select an input file:"));
                ui.horizontal(|ui| {
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
                });

                ui.vertical_centered(|ui| ui.heading("Select an output file:"));
                ui.horizontal(|ui| {
                    if ui.button("Select output file…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file() {
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
            });

        egui::CentralPanel::default().frame(launch_frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Click here to solve").clicked() {
                    self.start_solver_thread();
                }
            })
        });
    }

    fn update_in_progress(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame,
                          pct_complete: f32) {
        let mut progress_frame = egui::containers::Frame::default();
        progress_frame.fill = Color32::LIGHT_YELLOW;
        egui::CentralPanel::default().frame(progress_frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Running! Please be patient while the solver looks for optimal assignments.")
            });
            ui.add(egui::ProgressBar::new(pct_complete)
                .show_percentage()
                .animate(true));
            ui.label(format!("Input file: {}",
                             self.infile.as_ref().unwrap_or(&"".to_string())));
            ui.label(format!("Output file: {}",
                             self.outfile.as_ref().unwrap_or(&"".to_string())));
        });
    }

    fn update_success(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut success_frame = egui::containers::Frame::default();
        success_frame.fill = Color32::GREEN;
        egui::TopBottomPanel::new(TopBottomSide::Bottom, "Success")
            .frame(success_frame)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| ui.heading("Success! Output has been saved to disk."));
            });
        self.update_not_started(ctx, _frame);
    }

    fn update_failure(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, msg: String) {
        let mut failure_frame = egui::containers::Frame::default();
        failure_frame.fill = Color32::RED;
        egui::TopBottomPanel::new(TopBottomSide::Bottom, "Success")
            .frame(failure_frame)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| ui.heading("Failure! The solver encountered a problem:"));
                ui.label(msg);
            });
        self.update_not_started(ctx, _frame);
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
        std::thread::spawn(move || {
            let solver = Solver::new(FileType::CSV, FileType::CSV);
            solver.assign_workers(infile, outfile, status_tracker);
        });
    }
}

impl eframe::App for SolverGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.cur_status.get_status() {
            Status::Success => {
                self.update_success(ctx, _frame);
            },
            Status::InProgress(pct) => {
                self.update_in_progress(ctx, _frame, pct);
            },
            Status::Failure(msg) => {
                self.update_failure(ctx, _frame, msg);
            },
            Status::NotStarted => {
                self.update_not_started(ctx, _frame);
            }
        }
    }
}
