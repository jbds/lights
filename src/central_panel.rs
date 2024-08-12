use crate::TemplateApp;

pub fn get_closure(
    template_app: &mut TemplateApp,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response + '_ {
    |ui| {
        ui.heading("jonb b811111111111bb sales@jbds.co.uk");
        ui.add(
            egui::Slider::new(&mut template_app.value, 0.0..=10.0)
                .text("value")
                .orientation(egui::SliderOrientation::Vertical),
        )
    }
}
