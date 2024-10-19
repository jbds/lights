use crate::json_storage;
use crate::utilities;
use crate::LightsApp;

pub fn get_me(lights_app: &mut LightsApp, ctx: &egui::Context) {
    egui::SidePanel::right("right_panel")
        .show_separator_line(true)
        .min_width(130.0)
        .resizable(false)
        .show(ctx, |ui| {
            //ui.label("right_panel_placeholder");

            ui.label("");
            if ui.button("Edit Selected").clicked() {
                // store raw values, NOT the adjusted ones!
                let mut tweaked_values = lights_app.values.clone();
                // force the master value to zero
                tweaked_values[lights_app.values.len() - 1] = 0;
                // adjust light records to match current values
                lights_app.light_records[lights_app.light_records_index] =
                    (lights_app.short_text.clone(), tweaked_values);
                // persist the whole list of light records
                let _ = json_storage::write_to_file(&lights_app.light_records);
            }

            ui.label("");
            if ui.button("Del Selected").clicked() {
                // do nothing if length of lighting records is zero
                if lights_app.light_records.len() != 0 {
                    lights_app
                        .light_records
                        .remove(lights_app.light_records_index);
                    // adjust index if end of records
                    if lights_app.light_records.len() != 0
                        && lights_app.light_records.len() == lights_app.light_records_index
                    {
                        lights_app.light_records_index -= 1;
                    }
                    let _ = json_storage::write_to_file(&lights_app.light_records);
                }
            }

            ui.label("");
            if ui.button("Add After Selected").clicked {
                let u8s = vec![0; lights_app.slider_count];
                if lights_app.light_records.len() == 0 {
                    lights_app.light_records.push(("Scene".to_string(), u8s));
                } else {
                    lights_app.light_records.insert(
                        lights_app.light_records_index + 1,
                        ("Scene".to_string(), u8s),
                    );
                }
                let _ = json_storage::write_to_file(&lights_app.light_records);
            }

            ui.label("");
            if ui.button("< Back").clicked {
                // avoid subtract with overflow panic
                if lights_app.light_records_index == 0 {
                    lights_app.light_records_index = lights_app.light_records.len() - 1;
                } else {
                    lights_app.light_records_index =
                        (lights_app.light_records_index - 1) % lights_app.light_records.len();
                }

                // set current values to this selected lights_record
                (lights_app.short_text, lights_app.values) =
                    lights_app.light_records[lights_app.light_records_index].clone();
                // sync adjusted values
                lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                    lights_app.values.clone(),
                    lights_app.is_master_adjusteds.clone(),
                    lights_app.slider_count,
                    lights_app.is_blackout,
                );
            }

            ui.label("");
            if ui.button("Fade Up").clicked() {
                lights_app.is_fade_down = false;
                lights_app.is_fade_up = true;
            }

            ui.label("");
            if ui
                .checkbox(&mut lights_app.is_blackout, "Blackout")
                .clicked()
            {
                lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                    lights_app.values.clone(),
                    lights_app.is_master_adjusteds.clone(),
                    lights_app.slider_count,
                    lights_app.is_blackout,
                );
            }

            ui.label("");
            if ui.checkbox(&mut lights_app.is_shimmer, "Shimmer").clicked() {
                // store current master value
                lights_app.shimmer_master_value = lights_app.values[lights_app.slider_count - 1];
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                //egui::warn_if_debug_build(ui);
                ui.label("");
                if ui
                    .add_sized([110., 80.], egui::Button::new("Fade Down"))
                    .clicked()
                {
                    lights_app.is_fade_up = false;
                    lights_app.is_fade_down = true;
                }

                ui.label("");
                if ui
                    .add_sized([110., 80.], egui::Button::new("Next >"))
                    .clicked()
                {
                    lights_app.light_records_index =
                        (lights_app.light_records_index + 1) % lights_app.light_records.len();
                    // set current values to this selected lights_record
                    (lights_app.short_text, lights_app.values) =
                        lights_app.light_records[lights_app.light_records_index].clone();

                    lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                        lights_app.values.clone(),
                        lights_app.is_master_adjusteds.clone(),
                        lights_app.slider_count,
                        lights_app.is_blackout,
                    );
                    // trigger an auto fade up
                    lights_app.is_fade_down = false;
                    lights_app.is_fade_up = true;
                }
            });
        });
}
