use crate::LightsApp;

pub fn recalculate_lights_adjusted(lights_app: &mut LightsApp) {
    lights_app.values_adjusted = lights_app
        .values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            if lights_app.is_master_adjusteds[i] == true {
                ((f64::from(v) * f64::from(lights_app.values[lights_app.slider_count - 1])) / 255.0)
                    as u8
            } else {
                v
            }
        })
        .collect();
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
