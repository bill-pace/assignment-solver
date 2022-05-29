use std::sync::{Arc, Mutex};
mod model;
mod presenter;

#[derive(Clone)]
pub enum Status {
    Success,
    Failure(String), // error message
    InProgress(f32), // fraction complete
    NotStarted,
    RequestStart
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
    let pres = presenter::Presenter::new(status_tracker);

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(
        "Assignment Solver",
        options,
        Box::new(|_cc| Box::new(pres)),
    );
}
