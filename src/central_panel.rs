use crate::utilities;
use crate::LightsApp;

pub fn get_me(lights_app: &mut LightsApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        //ui.label("central_panel_placeholder");
        let mut i = 0;
        for vals in lights_app.light_records.iter() {
            // display all records
            if ui
                .add(egui::SelectableLabel::new(
                    i == lights_app.light_records_index,
                    format!("No:{} {:?}", i, &vals),
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
                //qualify by masrer dimmer
                lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                    lights_app.values.clone(),
                    lights_app.is_master_adjusteds.clone(),
                    lights_app.slider_count,
                )
            }
            i += 1;
        }
    });
}
