use eframe::egui;
use crate::io::FileType;
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

    pub fn report_error(&self, err: String) {
        self.view.report_error(err);
    }

    pub fn report_success(&self) {
        self.view.report_success();
    }
}

impl eframe::App for Presenter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.view.begin_solving {
            self.view.begin_solving = false;
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

            self.model.set_file_types(FileType::CSV, FileType::CSV);

            self.model.assign_workers(infile_res.unwrap(), outfile_res.unwrap(), &self);
        } if self.model.is_solving() {
            self.view.update_running(ctx, _frame)
        } else {
            self.view.update_input_output(ctx, _frame)
        }
    }
}
