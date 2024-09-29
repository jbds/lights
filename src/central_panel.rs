use crate::json_storage;
use crate::utilities;
use crate::LightsApp;

pub fn get_closure(
    lights_app: &mut LightsApp,
) -> impl FnOnce(&mut egui::Ui) -> egui::Response + '_ {
    |ui| {
        let mut count: usize = 0;
        ui.horizontal(|ui| {
            // lhs margin
            ui.label("                      ");
            // set the 'width' (height) of the slider
            ui.spacing_mut().slider_width = 300.0;
            // last slider, the master dimmer, is a special case UI layout
            while count != (lights_app.slider_count - 1) {
                let resp = get_slider(ui, lights_app, count);
                if resp.changed() == true {
                    lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                        lights_app.values.clone(),
                        lights_app.is_master_adjusteds.clone(),
                        lights_app.slider_count,
                    )
                }
                count += 1;
            }
        });

        ui.label("a");
        ui.label("b");
        let mut i = 0;
        for vals in lights_app.light_records.iter() {
            if ui
                .add(egui::SelectableLabel::new(
                    i == lights_app.light_records_index,
                    format!("No:{} Payload: {:?}", i, &vals),
                ))
                .clicked()
            {
                lights_app.light_records_index = i;
                // set current values to this selected lights_record
                lights_app.values =
                    lights_app.light_records[lights_app.light_records_index].clone();
                lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                    lights_app.values.clone(),
                    lights_app.is_master_adjusteds.clone(),
                    lights_app.slider_count,
                )
            }
            i += 1;
        }
        ui.label("x");
        ui.label("y");
        if ui.button("Next>").clicked() {
            lights_app.light_records_index =
                (lights_app.light_records_index + 1) % lights_app.light_records.len();
            // set current values to this selected lights_record
            lights_app.values = lights_app.light_records[lights_app.light_records_index].clone();
            lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                lights_app.values.clone(),
                lights_app.is_master_adjusteds.clone(),
                lights_app.slider_count,
            )
        }

        if ui.button("Fade Up").clicked() {
            lights_app.is_fade_down = false;
            lights_app.is_fade_up = true;
        }

        if ui.button("Fade Down").clicked() {
            lights_app.is_fade_up = false;
            lights_app.is_fade_down = true;
        }

        if ui.button("Save to selected").clicked() {
            // store raw values, NOT the adjusted ones!
            lights_app.light_records[lights_app.light_records_index] = lights_app.values.clone();
            // persist
            let _ = json_storage::write_to_file(&lights_app.light_records);
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });

        // dummy component to return correct type
        ui.heading(" ")
    }
}

fn get_slider(ui: &mut egui::Ui, lights_app: &mut LightsApp, count: usize) -> egui::Response {
    // these magic numbers affect the UI layout only
    if count % 4 == 0 && count < 16 && count > 0 {
        ui.label("               ");
    }
    ui.add(
        egui::Slider::new(&mut lights_app.values[count], 0..=255)
            .integer()
            .text(lights_app.labels[count].clone())
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
