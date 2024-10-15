use crate::utilities;
use crate::LightsApp;

pub fn get_me(lights_app: &mut LightsApp, ctx: &egui::Context) {
    // egui::CentralPanel::default().show(ctx, |ui| {
    //     //ui.label("central_panel_placeholder");
    //     let mut i = 0;
    //     for vals in lights_app.light_records.iter() {
    //         if ui
    //             .add(egui::SelectableLabel::new(
    //                 i == lights_app.light_records_index,
    //                 format!("No:{} Payload: {:?}", i, &vals),
    //             ))
    //             .clicked()
    //         {
    //             lights_app.light_records_index = i;
    //             // set current values to this selected lights_record
    //             lights_app.values =
    //                 lights_app.light_records[lights_app.light_records_index].clone();
    //             lights_app.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
    //                 lights_app.values.clone(),
    //                 lights_app.is_master_adjusteds.clone(),
    //                 lights_app.slider_count,
    //             )
    //         }
    //         i += 1;
    //     }
    // });
}
