#![warn(clippy::all, rust_2018_idioms)]

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            // buggy, have to set min_inner_size
            .with_inner_size([1300.0, 700.0])
            .with_min_inner_size([1300.0, 700.0])
            .with_max_inner_size([1302.0, 702.0]),
        ..Default::default()
    };
    eframe::run_native(
        "lights",
        native_options,
        Box::new(|cc| Ok(Box::new(lights::TemplateApp::new(cc)))),
    )
}
