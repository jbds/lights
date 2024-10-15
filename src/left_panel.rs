use core::f32;

use crate::utilities;
use crate::LightsApp;

pub fn get_me(lights_app: &mut LightsApp, ctx: &egui::Context) {
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        //ui.label("left_panel_placeholder");

        ui.label("");
        ui.horizontal(|ui| {
            let _response = ui.add(
                egui::TextEdit::singleline(&mut lights_app.short_text).desired_width(f32::INFINITY),
            );
        });

        let mut count: usize = 0;
        // set the 'width' (height) of the sliders
        ui.spacing_mut().slider_width = 300.0;

        ui.label("");
        // paint all sliders except last one
        while count != (lights_app.slider_count - 1) {
            let resp = utilities::get_slider(ui, lights_app, count);
            if resp.changed() == true {
                lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                    lights_app.values.clone(),
                    lights_app.is_master_adjusteds.clone(),
                    lights_app.slider_count,
                )
            }
            count += 1;
        }

        // last slider, the master dimmer, is a special case UI layout
        ui.label("     ");
        let resp2 = ui.add(
            egui::Slider::new(&mut lights_app.values[lights_app.slider_count - 1], 0..=255)
                .integer()
                .text("Master")
                //.orientation(egui::SliderOrientation::Vertical),
                .orientation(egui::SliderOrientation::Horizontal),
        );
        if resp2.changed() == true {
            lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                lights_app.values.clone(),
                lights_app.is_master_adjusteds.clone(),
                lights_app.slider_count,
            )
        }

        // standard egui/eframe info
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    });
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
