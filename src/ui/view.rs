use eframe::egui;
use crate::ui::presenter::Presenter;

pub struct View {
    infile: Option<String>,
    outfile: Option<String>,
    status: Option<String>
}

impl View {
    pub fn new() -> Self {
        View {
            infile: None,
            outfile: None,
            status: None
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

    pub fn report_error(&mut self, err: String) {
        self.status = Some(err);
    }

    pub fn report_success(&mut self) {
        self.status = Some("Success! Results have been saved.".to_string());
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
    }
}

/// Preview hovering files:
fn _preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;

    if !ctx.input().raw.hovered_files.is_empty() {
        let mut text = "Dropping files:\n".to_owned();
        for file in &ctx.input().raw.hovered_files {
            if let Some(path) = &file.path {
                text += &format!("\n{}", path.display());
            } else if !file.mime.is_empty() {
                text += &format!("\n{}", file.mime);
            } else {
                text += "\n???";
            }
        }

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.input().screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}
