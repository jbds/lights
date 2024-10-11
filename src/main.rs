#![warn(clippy::all, rust_2018_idioms)]

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // buggy, have to set min_inner_size
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([750.0, 550.0])
            .with_max_inner_size([850.0, 650.0]),
        ..Default::default()
    };

    eframe::run_native(
        "lights",
        native_options,
        Box::new(|cc| Ok(Box::new(lights::LightsApp::new(cc)))),
    )
}
