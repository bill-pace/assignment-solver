use std::sync::{Arc, Mutex};
use eframe::egui::Vec2;

mod solver;
mod solver_gui;

#[derive(Clone)]
pub enum Status {
    Success,
    Failure(String), // error message
    InProgress(f32), // fraction complete
    NotStarted
}

pub struct CurrentStatus {
    status: Mutex<Status>
}

impl CurrentStatus {
    pub fn new() -> Self {
        CurrentStatus {
            status: Mutex::new(Status::NotStarted)
        }
    }

    pub fn get_status(&self) -> Status {
        self.status.lock().unwrap().clone()
    }

    pub fn set_status(&self, new_status: Status) {
        *self.status.lock().unwrap() = new_status;
    }
}

pub fn launch_ui(status_tracker: Arc<CurrentStatus>) {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(Vec2 { x: 1024.0, y: 384.0 }),
        ..Default::default()
    };

    eframe::run_native(
        "Assignment Solver",
        options,
        Box::new(|cc| {
            Box::new(solver_gui::SolverGui::new(status_tracker, cc))
        })
    );
}
