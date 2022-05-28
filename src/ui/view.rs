use std::cell::Cell;
use eframe::egui;
use eframe::egui::panel::TopBottomSide;

pub struct View {
    infile: Option<String>,
    outfile: Option<String>,
    status: Cell<Option<String>>,
    pub(crate) begin_solving: bool
}

impl View {
    pub fn new() -> Self {
        View {
            infile: None,
            outfile: None,
            status: Cell::new(None),
            begin_solving: false
        }
    }

    pub fn get_infile_name(&self) -> Result<String, std::io::Error> {
        match &self.infile {
            Some(s) => Ok(s.clone()),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound,
                                            "You must select an input file!"))
        }
    }

    pub fn get_outfile_name(&self) -> Result<String, std::io::Error> {
        match &self.outfile {
            Some(s) => Ok(s.clone()),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound,
                                            "You must select an output file!"))
        }
    }

    pub fn report_error(&self, err: String) {
        self.status.set(Some(err));
    }

    pub fn report_success(&self) {
        self.status.set(Some("Success! Results have been saved.".to_string()));
    }

    pub fn update_input_output(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                self.begin_solving = true;
            }
        });
    }

    pub fn update_running(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Running!");
        });
    }
}
