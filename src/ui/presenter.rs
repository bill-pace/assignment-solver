use std::sync::Arc;
use eframe::egui;
use crate::io::FileType;
use crate::ui::{CurrentStatus, Status};
use crate::ui::model::Model;
use crate::ui::view::View;

pub struct Presenter {
    model: Model,
    view: View,
    cur_status: Arc<CurrentStatus>
}

impl Presenter {
    pub fn new(model: Model, view: View, status_tracker: Arc<CurrentStatus>) -> Self {
        Presenter {
            model,
            view,
            cur_status: status_tracker
        }
    }
}

impl eframe::App for Presenter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.cur_status.get_status() {
            Status::Success => {
                self.view.update_input_output(ctx, _frame)
            },
            Status::InProgress(pct) => {
                self.view.update_running(ctx, _frame)
            },
            Status::Failure(msg) => {
                self.view.update_input_output(ctx, _frame)
            },
            Status::NotStarted => {
                self.view.update_input_output(ctx, _frame)
            },
            Status::RequestStart => {
                self.view.update_input_output(ctx, _frame)
            }
        }
    }
}
