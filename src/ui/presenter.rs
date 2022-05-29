use std::sync::Arc;
use eframe::egui;
use crate::io::FileType;
use crate::ui::{CurrentStatus, Status};
use crate::ui::model::Model;
use crate::ui::view::View;

pub struct Presenter {
    view: View,
    cur_status: Arc<CurrentStatus>
}

impl Presenter {
    pub fn new(view: View, status_tracker: Arc<CurrentStatus>) -> Self {
        Presenter {
            view,
            cur_status: status_tracker
        }
    }

    fn start_solver_thread(&self) {
        let infile = match self.view.get_infile_name() {
            Ok(name) => name,
            Err(e) => {
                self.cur_status.set_status(Status::Failure(e.to_string()));
                return;
            }
        };
        let outfile = match self.view.get_outfile_name() {
            Ok(name) => name,
            Err(e) => {
                self.cur_status.set_status(Status::Failure(e.to_string()));
                return;
            }
        };
        let status_tracker = self.cur_status.clone();
        std::thread::spawn(|| make_assignment(Model::new(),
                                              infile,
                                              outfile,
                                              status_tracker));
    }
}

impl eframe::App for Presenter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.cur_status.get_status() {
            Status::Success => {
                self.view.update_input_output(ctx, _frame)
            },
            Status::InProgress(pct) => {
                self.view.update_running(ctx, _frame, pct)
            },
            Status::Failure(msg) => {
                self.view.update_input_output(ctx, _frame)
            },
            Status::NotStarted => {
                self.view.update_input_output(ctx, _frame)
            },
            Status::RequestStart => {
                self.start_solver_thread();
                self.view.update_running(ctx, _frame, 0.0)
            }
        }
    }
}

fn make_assignment(model: Model, input_file: String, output_file: String,
                   status_tracker: Arc<CurrentStatus>) {
    model.assign_workers(input_file, output_file, status_tracker);
}
