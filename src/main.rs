mod network;
mod io;
mod ui;

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(
        "Assignment Solver",
        options,
        Box::new(|_cc| Box::new(ui::MyApp::default())),
    );
}
