#![windows_subsystem = "windows"]

use std::sync::Arc;
mod network;
mod io;
mod ui;

fn main() {
    let cur_status = Arc::new(ui::CurrentStatus::new());
    ui::launch_ui(cur_status);
}
