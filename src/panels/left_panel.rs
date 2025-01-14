use crate::utilities;
use crate::LightsApp;

pub fn get_me(lights_app: &mut LightsApp, ctx: &egui::Context) {
    egui::SidePanel::left("left_panel")
        .min_width(450.0)
        .show(ctx, |ui| {
            //ui.label("left_panel_placeholder");

            ui.label("");
            let mut i = 0;
            for vals in lights_app.light_records.iter() {
                // display all records
                if ui
                    .add(egui::SelectableLabel::new(
                        i == lights_app.light_records_index,
                        //format!("No:{} {:?}", i, &vals),
                        &vals.0,
                    ))
                    .clicked()
                {
                    // show this record as selected
                    lights_app.light_records_index = i;
                    // set current values to this selected lights_record
                    let temp = lights_app.light_records[lights_app.light_records_index].clone();
                    lights_app.values = temp.1;
                    // set scene desc to this selected record
                    lights_app.short_text = temp.0;
                    //qualify by master dimmer
                    lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                        lights_app.values.clone(),
                        lights_app.is_master_adjusteds.clone(),
                        lights_app.slider_count,
                        lights_app.is_blackout,
                    )
                }
                i += 1;
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
        ui.label(format!("Lights version {}     ", env!("CARGO_PKG_VERSION")));
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
