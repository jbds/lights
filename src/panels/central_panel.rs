use crate::utilities;
use crate::LightsApp;

pub fn get_me(lights_app: &mut LightsApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        //ui.label("central_panel_placeholder");
        egui::ScrollArea::vertical().show(ui, |ui| {

            ui.add_space(5.);
            ui.horizontal(|ui| {
                let _response = ui.add(
                    egui::TextEdit::singleline(&mut lights_app.short_text).desired_width(f32::INFINITY),
                );
            });

            let mut count: usize = 0;
            // set the 'width' (height) of the sliders
            ui.spacing_mut().slider_width = 500.0;

            // slider rail height
            ui.spacing_mut().slider_rail_height = 2.0;

            //ui.label("");
            // paint all sliders 
            while count < (lights_app.slider_count) {
                // adjust the slider index so that we draw the Master slider BEFORE the Strobe and Fusion 120 sliders
                let mut slider_index: usize = count;
                if count == 14 {
                    slider_index = count + 10;
                }
                if count > 14 {
                    slider_index = count - 1;
                }
                let resp = utilities::get_slider(ui, lights_app, slider_index);
                if resp.changed() == true {
                    lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                        lights_app.values.clone(),
                        lights_app.is_master_adjusteds.clone(),
                        lights_app.slider_count,
                        lights_app.is_blackout,
                    )
                }
                count += 1;
            }
        });
    });
}
