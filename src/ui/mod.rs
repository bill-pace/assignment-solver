mod model;
mod view;
mod presenter;

pub fn launch_ui() {
    let view = view::View::new();
    let model = model::Model::new();
    let pres = presenter::Presenter::new(model, view);

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(
        "Assignment Solver",
        options,
        Box::new(|_cc| Box::new(pres.view)),
    );
}
