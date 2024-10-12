use crate::utilities;
use crate::LightsApp;

pub fn get_closure(
    lights_app: &mut LightsApp,
) -> impl FnOnce(&mut egui::UI) -> egui::Response + '_ {
    |ui| {
        ui.label("top_panel_here");
    }
}
