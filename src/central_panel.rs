use crate::utilities::recalculate_lights_dependent;
use crate::TemplateApp;

pub fn get_closure(
    template_app: &mut TemplateApp,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response + '_ {
    |ui| {
        //ui.heading("jonb b811111111111bb sales@jbds.co.uk");

        let mut count: usize = 0;
        ui.horizontal(|ui| {
            // lhs margin
            ui.label("                      ");
            // set the 'width' (height) of the slider
            ui.spacing_mut().slider_width = 300.0;
            while count != 20 {
                let resp = get_slider(ui, template_app, count);
                if resp.changed() == true {
                    recalculate_lights_dependent(template_app);
                }
                count += 1;
            }
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });

        // dummy component to return correct type
        ui.heading(" ")
    }
}

fn get_slider(ui: &mut egui::Ui, template_app: &mut TemplateApp, count: usize) -> egui::Response {
    if count % 4 == 0 && count < 16 && count > 0 {
        ui.label("               ");
    }
    ui.add(
        egui::Slider::new(&mut template_app.values[count], 0..=255)
            .integer()
            .text(format!("ch{}", count + 1))
            .orientation(egui::SliderOrientation::Vertical),
    )
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
