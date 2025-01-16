use crate::utilities;
use crate::LightsApp;

pub fn get_me(lights_app: &mut LightsApp, ctx: &egui::Context) {
    egui::SidePanel::right("right_panel")
        .show_separator_line(true)
        .min_width(130.0)
        .resizable(false)
        .show(ctx, |ui| {
            //ui.label("right_panel_placeholder");

            //ui.label("");
            ui.add_space(10.);
            //if ui.button("Save Selected").clicked() {
            if ui
                .add_sized([170., 35.], egui::Button::new("Save Selected"))
                .clicked()
            {
                lights_app.show_confirmation_dialog_title = String::from("SAVE SELECTED");
                lights_app.show_confirmation_dialog = true;
            }

            //ui.label("");
            ui.add_space(10.);
            //if ui.button("Del Selected").clicked() {
            if ui
                .add_sized([170., 35.], egui::Button::new("Del Selected"))
                .clicked()
            {
                lights_app.show_confirmation_dialog_title = String::from("DELETE SELECTED");
                lights_app.show_confirmation_dialog = true;
            }

            //ui.label("");
            ui.add_space(10.);
            //if ui.button("Add After Selected").clicked {
            if ui
                .add_sized([170., 35.], egui::Button::new("Add After Selected"))
                .clicked()
            {
                lights_app.show_confirmation_dialog_title = String::from("ADD AFTER SELECTED");
                lights_app.show_confirmation_dialog = true;
            }

            //ui.label("");
            ui.add_space(10.);
            ui.horizontal(|ui| {
                //if ui.button("   Up   ").clicked {
                if ui.add_sized([81., 35.], egui::Button::new("Up")).clicked() {
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

                //if ui.button("Down").clicked {
                if ui
                    .add_sized([81., 35.], egui::Button::new("Down"))
                    .clicked()
                {
                    // avoid add with overflow panic
                    if lights_app.light_records_index == lights_app.light_records.len() - 1 {
                        lights_app.light_records_index = 0;
                    } else {
                        lights_app.light_records_index =
                            (lights_app.light_records_index + 1) % lights_app.light_records.len();
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
            });

            //ui.label("");
            ui.add_space(10.);
            //if ui.button("Fade Up").clicked() {
            if ui
                .add_sized([170., 35.], egui::Button::new("Fade Up Fast"))
                .clicked()
            {
                lights_app.fader_speed = 1.0;
                lights_app.is_fade_down = false;
                lights_app.is_fade_up = true;
            }

            ui.add_space(20.);
            if ui
                .checkbox(&mut lights_app.is_ultra_violet, "Ultra Violet")
                .clicked()
            {
                utilities::recalculate_ultra_violet(lights_app);
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
                utilities::recalculate_ultra_violet(lights_app);
            }

            ui.label("");
            if ui.checkbox(&mut lights_app.is_shimmer, "Shimmer").clicked() {
                // store current master value
                lights_app.shimmer_master_value = lights_app.values[lights_app.slider_count - 1];
            }

            // set the 'width' (height) of the sliders
            ui.spacing_mut().slider_width = 100.0;
            ui.horizontal(|ui| {
                let _resp = ui.add(
                    egui::Slider::new(&mut lights_app.shimmer_amplitude_percent, 0.0..=100.0)
                        .integer()
                        .text("A")
                        //.orientation(egui::SliderOrientation::Vertical),
                        .orientation(egui::SliderOrientation::Horizontal),
                );
            });
            ui.horizontal(|ui| {
                let _resp = ui.add(
                    egui::Slider::new(&mut lights_app.shimmer_frequency_hertz, 0.25..=4.0)
                        //.integer()
                        .text("F")
                        //.orientation(egui::SliderOrientation::Vertical),
                        .orientation(egui::SliderOrientation::Horizontal),
                );
            });

            ui.add_space(10.);
            if ui
                .add_sized([170., 35.], egui::Button::new("Next Jump"))
                .clicked()
            {
                lights_app.light_records_index =
                    (lights_app.light_records_index + 1) % lights_app.light_records.len();
                // set current values to this selected lights_record
                (lights_app.short_text, lights_app.values) =
                    lights_app.light_records[lights_app.light_records_index].clone();

                // force no fade up or down, and then master to full
                lights_app.is_fade_up = false;
                lights_app.is_fade_down = false;
                lights_app.values[lights_app.slider_count - 1] = 255.0;

                lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                    lights_app.values.clone(),
                    lights_app.is_master_adjusteds.clone(),
                    lights_app.slider_count,
                    lights_app.is_blackout,
                );
            }

            ui.add_space(10.);
            if ui
                .add_sized([170., 35.], egui::Button::new("Previous Jump"))
                .clicked()
            {
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

                // force no fade up or down, and then master to full
                lights_app.is_fade_up = false;
                lights_app.is_fade_down = false;
                lights_app.values[lights_app.slider_count - 1] = 255.0;

                lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                    lights_app.values.clone(),
                    lights_app.is_master_adjusteds.clone(),
                    lights_app.slider_count,
                    lights_app.is_blackout,
                );
            }

            // BIG SPACE
            ui.add_space(10.);

            if ui
                .add_sized([170., 70.], egui::Button::new("Fade Down Fast"))
                .clicked()
            {
                lights_app.fader_speed = 2.0;
                lights_app.is_fade_up = false;
                lights_app.is_fade_down = true;
            }

            //ui.label("");
            ui.add_space(10.);

            ui.horizontal(|ui| {
                if ui
                    .add_sized([81., 70.], egui::Button::new("Next\nSlow"))
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
                    // trigger an auto fade up slow
                    lights_app.fader_speed = 0.3;
                    lights_app.is_fade_down = false;
                    lights_app.is_fade_up = true;
                }

                if ui
                    .add_sized([81., 70.], egui::Button::new("Next\nFast"))
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
                    // trigger an auto fade up fast
                    lights_app.fader_speed = 2.0;
                    lights_app.is_fade_down = false;
                    lights_app.is_fade_up = true;
                }
            });
            //});
        });
}
