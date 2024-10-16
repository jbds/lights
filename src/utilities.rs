use crate::LightsApp;

pub fn recalculate_lights_adjusted_no_borrow(
    values: Vec<u8>,
    is_master_adjusteds: Vec<bool>,
    slider_count: usize,
    is_blackout: bool,
) -> Vec<u8> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            if is_master_adjusteds[i] == true {
                ((f64::from(v) * f64::from(values[slider_count - 1]) * f64::from(!is_blackout))
                    / 255.0) as u8
            } else {
                v
            }
        })
        .collect()
}

pub fn increment_master(lights_app: &mut LightsApp) {
    let inc1 = 1;
    let inc2 = 2;
    let inc3 = 3;
    let inc5 = 5;
    let val = lights_app.values[lights_app.slider_count - 1];
    if val > 255 - inc5 {
        lights_app.values[lights_app.slider_count - 1] = 255
    } else if val < 46 {
        lights_app.values[lights_app.slider_count - 1] += inc1;
    } else if val < 84 {
        lights_app.values[lights_app.slider_count - 1] += inc2;
    } else if val < 143 {
        lights_app.values[lights_app.slider_count - 1] += inc3;
    } else {
        lights_app.values[lights_app.slider_count - 1] += inc5;
    }
}

pub fn decrement_master(lights_app: &mut LightsApp) {
    let dec5 = 5;
    let dec3 = 3;
    let dec2 = 2;
    let dec1 = 1;
    let val = lights_app.values[lights_app.slider_count - 1];
    if val < 0 + dec1 {
        lights_app.values[lights_app.slider_count - 1] = 0;
    } else if val < 46 {
        lights_app.values[lights_app.slider_count - 1] -= dec1;
    } else if val < 84 {
        lights_app.values[lights_app.slider_count - 1] -= dec2;
    } else if val < 143 {
        lights_app.values[lights_app.slider_count - 1] -= dec3;
    } else {
        lights_app.values[lights_app.slider_count - 1] -= dec5;
    }
}

pub fn get_slider(ui: &mut egui::Ui, lights_app: &mut LightsApp, count: usize) -> egui::Response {
    // these magic numbers affect the UI layout only
    if count % 4 == 0 && count < 16 && count > 0 {
        ui.label("     ");
    }
    ui.add(
        egui::Slider::new(&mut lights_app.values[count], 0..=255)
            .integer()
            .text(lights_app.labels[count].clone())
            //.orientation(egui::SliderOrientation::Vertical),
            .orientation(egui::SliderOrientation::Horizontal),
    )
}
