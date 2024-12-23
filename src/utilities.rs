use crate::json_storage;
use crate::LightsApp;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

pub enum FaderSpeed {
    Fast,
    Slow,
}

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

// pub fn increment_master(lights_app: &mut LightsApp, fader_speed: FaderSpeed) {
//     let inc1 = 1;
//     let inc2 = 2;
//     let inc3 = 4;
//     let inc5 = 8;
//     let val = lights_app.values[lights_app.slider_count - 1];
//     if val > 255 - inc5 {
//         lights_app.values[lights_app.slider_count - 1] = 255
//     } else if val < 46 {
//         lights_app.values[lights_app.slider_count - 1] += inc1;
//     } else if val < 84 {
//         lights_app.values[lights_app.slider_count - 1] += inc2;
//     } else if val < 143 {
//         lights_app.values[lights_app.slider_count - 1] += inc3;
//     } else {
//         lights_app.values[lights_app.slider_count - 1] += inc5;
//     }
// }

pub fn increment_master(lights_app: &mut LightsApp, fader_speed: FaderSpeed) {
    let inc1 = match fader_speed {
        FaderSpeed::Fast => 1.0,
        FaderSpeed::Slow => 0.25,
    };
    let inc2 = match fader_speed {
        FaderSpeed::Fast => 2.0,
        FaderSpeed::Slow => 0.5,
    };
    let inc3 = match fader_speed {
        FaderSpeed::Fast => 4.0,
        FaderSpeed::Slow => 1.0,
    };
    let inc4 = match fader_speed {
        FaderSpeed::Fast => 8.0,
        FaderSpeed::Slow => 2.0,
    };
    //let val = lights_app.values[lights_app.slider_count - 1] as f64;
    let val = lights_app.master_value_f64;
    if val > 255.0 - inc4 {
        lights_app.values[lights_app.slider_count - 1] = 255.0 as u8;
        lights_app.master_value_f64 = 255.0;
    } else if val < 46.0 {
        lights_app.values[lights_app.slider_count - 1] = (lights_app.master_value_f64 + inc1) as u8;
        lights_app.master_value_f64 += inc1;
    } else if val < 84.0 {
        lights_app.values[lights_app.slider_count - 1] = (lights_app.master_value_f64 + inc2) as u8;
        lights_app.master_value_f64 += inc2;
    } else if val < 143.0 {
        lights_app.values[lights_app.slider_count - 1] = (lights_app.master_value_f64 + inc3) as u8;
        lights_app.master_value_f64 += inc3;
    } else {
        lights_app.values[lights_app.slider_count - 1] = (lights_app.master_value_f64 + inc4) as u8;
        lights_app.master_value_f64 += inc4;
    }
}

pub fn decrement_master(lights_app: &mut LightsApp, fader_speed: FaderSpeed) {
    let dec4 = match fader_speed {
        FaderSpeed::Fast => 8.0,
        FaderSpeed::Slow => 2.0,
    };
    let dec3 = match fader_speed {
        FaderSpeed::Fast => 4.0,
        FaderSpeed::Slow => 1.0,
    };
    let dec2 = match fader_speed {
        FaderSpeed::Fast => 2.0,
        FaderSpeed::Slow => 0.5,
    };
    let dec1 = match fader_speed {
        FaderSpeed::Fast => 1.0,
        FaderSpeed::Slow => 0.25,
    };
    //let val = lights_app.values[lights_app.slider_count - 1];
    let val = lights_app.master_value_f64;
    if val < 0.0 + dec1 {
        lights_app.values[lights_app.slider_count - 1] = 0.0 as u8;
        lights_app.master_value_f64 = 0.0;
    } else if val < 46.0 {
        lights_app.values[lights_app.slider_count - 1] = (lights_app.master_value_f64 - dec1) as u8;
        lights_app.master_value_f64 -= dec1;
    } else if val < 84.0 {
        lights_app.values[lights_app.slider_count - 1] = (lights_app.master_value_f64 - dec2) as u8;
        lights_app.master_value_f64 -= dec2;
    } else if val < 143.0 {
        lights_app.values[lights_app.slider_count - 1] = (lights_app.master_value_f64 - dec3) as u8;
        lights_app.master_value_f64 -= dec3;
    } else {
        lights_app.values[lights_app.slider_count - 1] = (lights_app.master_value_f64 - dec4) as u8;
        lights_app.master_value_f64 -= dec4;
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
    tweaked_values[lights_app.values.len() - 1] = 0;
    // adjust light records to match current values
    lights_app.light_records[lights_app.light_records_index] =
        (lights_app.short_text.clone(), tweaked_values);
    // persist the whole list of light records
    let _ = json_storage::write_to_file(&lights_app.light_records);
}

pub fn add_after_selected(lights_app: &mut LightsApp) {
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
