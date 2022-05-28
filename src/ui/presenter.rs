use eframe::egui;
use crate::ui::model::Model;
use crate::ui::view::View;

pub struct Presenter {
    model: Model,
    view: View
}

impl Presenter {
    pub fn new(model: Model, view: View) -> Self {
        Presenter {
            model,
            view
        }
    }

    pub fn handle_solve_button_click(&self) {
        let infile_res = self.view.get_infile_name();
        if infile_res.is_err() {
            self.report_error(infile_res.err().unwrap().to_string());
            return;
        }
        let outfile_res = self.view.get_outfile_name();
        if outfile_res.is_err() {
            self.report_error(outfile_res.err().unwrap().to_string());
            return;
        }

        self.model.assign_workers(infile_res.unwrap(), outfile_res.unwrap(), &self);
    }

    pub fn report_error(&self, err: String) {

    }

    pub fn report_success(&self) {

    }
}

impl eframe::App for Presenter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.view.update(ctx, _frame)
    }
}
