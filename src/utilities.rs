use crate::json_storage;
use crate::LightsApp;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

pub fn recalculate_lights_adjusted_no_borrow(
    values: Vec<f64>,
    is_master_adjusteds: Vec<bool>,
    slider_count: usize,
    is_blackout: bool,
) -> Vec<f64> {
    values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            if is_master_adjusteds[i] == true {
                v * values[slider_count - 1] * f64::from(!is_blackout) / 255.0
            } else {
                v
            }
        })
        .collect()
}

pub fn increment_master(lights_app: &mut LightsApp) {
    let inc1 = lights_app.fader_speed;
    let inc2 = lights_app.fader_speed * 2.0;
    let inc3 = lights_app.fader_speed * 4.0;
    let inc4 = lights_app.fader_speed * 8.0;
    let val = lights_app.values[lights_app.slider_count - 1];
    if val > 255.0 - inc4 {
        lights_app.values[lights_app.slider_count - 1] = 255.0;
    } else if val < 46.0 {
        lights_app.values[lights_app.slider_count - 1] += inc1;
    } else if val < 84.0 {
        lights_app.values[lights_app.slider_count - 1] += inc2;
    } else if val < 143.0 {
        lights_app.values[lights_app.slider_count - 1] += inc3;
    } else {
        lights_app.values[lights_app.slider_count - 1] += inc4;
    }
}

pub fn decrement_master(lights_app: &mut LightsApp) {
    let dec1 = lights_app.fader_speed;
    let dec2 = lights_app.fader_speed * 2.0;
    let dec3 = lights_app.fader_speed * 4.0;
    let dec4 = lights_app.fader_speed * 8.0;
    let val = lights_app.values[lights_app.slider_count - 1];
    if val < 0.0 + dec1 {
        lights_app.values[lights_app.slider_count - 1] = 0.0;
    } else if val < 46.0 {
        lights_app.values[lights_app.slider_count - 1] -= dec1;
    } else if val < 84.0 {
        lights_app.values[lights_app.slider_count - 1] -= dec2;
    } else if val < 143.0 {
        lights_app.values[lights_app.slider_count - 1] -= dec3;
    } else {
        lights_app.values[lights_app.slider_count - 1] -= dec4;
    }
}

pub fn shimmer_master(lights_app: &mut LightsApp) {
    // reset cycle every whole cycle - this number affects rate of shimmer
    let cycle_time_ms = 1000.0 / lights_app.shimmer_frequency_hertz;
    if lights_app.shimmer_instant.elapsed() > Duration::from_millis(cycle_time_ms as u64) {
        lights_app.shimmer_instant = Instant::now();
    }
    let x = lights_app.shimmer_instant.elapsed().as_secs_f64() * 1000.0;
    let y = f64::sin(x * PI * 2.0 / cycle_time_ms);
    //println!("{}", y);
    let amplitude_factor = 200.0 / lights_app.shimmer_amplitude_percent;
    lights_app.values[lights_app.slider_count - 1] =
        lights_app.shimmer_master_value * (1.0 - ((y + 1.0) / amplitude_factor));
    println!("{}", lights_app.values[lights_app.slider_count - 1]);
}

pub fn get_slider(ui: &mut egui::Ui, lights_app: &mut LightsApp, count: usize) -> egui::Response {
    // these magic numbers affect the UI layout only
    if count == 0 || count == 4 || count == 10 || count == 20{
        ui.label("     ");
    }
    ui.add(
        egui::Slider::new(&mut lights_app.values[count], 0.0..=255.0)
            .integer()
            .text(lights_app.labels[count].clone())
            //.orientation(egui::SliderOrientation::Vertical),
            .orientation(egui::SliderOrientation::Horizontal),
    )
}

pub fn delete_selected(lights_app: &mut LightsApp) {
    //do nothing if length of lighting records is zero
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

pub fn save_selected(lights_app: &mut LightsApp) {
    // store raw values, NOT the adjusted ones!
    let mut tweaked_values = lights_app.values.clone();
    // force the master value to zero
    tweaked_values[lights_app.values.len() - 1] = 0.0;
    // adjust light records to match current values
    lights_app.light_records[lights_app.light_records_index] =
        (lights_app.short_text.clone(), tweaked_values);
    // persist the whole list of light records
    let _ = json_storage::write_to_file(&lights_app.light_records);
}

pub fn add_after_selected(lights_app: &mut LightsApp) {
    let u8s = vec![0.0; lights_app.slider_count];
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

pub fn recalculate_ultra_violet(lights_app: &mut LightsApp) {
    // uv ON OFF
    for i in 20..=23 {
        if lights_app.is_ultra_violet && !lights_app.is_blackout {
            lights_app.array_of_u8[i] = 255;
        } else {
            lights_app.array_of_u8[i] = 0;
        }
    }
}
