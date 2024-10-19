use crate::LightsApp;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

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
    let inc3 = 4;
    let inc5 = 8;
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
    let dec5 = 8;
    let dec3 = 4;
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
        (lights_app.shimmer_master_value as f64 * (1.0 - ((y + 1.0) / amplitude_factor))) as u8;
    println!("{}", lights_app.values[lights_app.slider_count - 1]);
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
