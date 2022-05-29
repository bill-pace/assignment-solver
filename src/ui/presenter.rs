use std::sync::Arc;
use eframe::egui;
use crate::io::FileType;
use crate::ui::CurrentStatus;
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

    }
}
