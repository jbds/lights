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
