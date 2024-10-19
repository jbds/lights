use crate::central_panel;
use crate::json_storage;
use crate::left_panel;
use crate::right_panel;
use crate::top_panel;
use crate::utilities;

#[cfg(target_arch = "aarch64")]
use dmx::{self, DmxTransmitter};
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::time::{Duration, Instant};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LightsApp {
    //#[serde(skip)] // This how you opt-out of serialization of a field
    pub slider_count: usize,
    pub values: Vec<u8>, //stores the current array of light values including master dimmer
    pub is_master_adjusteds: Vec<bool>,
    pub labels: Vec<String>,
    pub values_adjusted: Vec<u8>,
    pub instant: Instant,   // we need this to check timing
    pub duration: Duration, // ditto
    #[cfg(target_arch = "aarch64")]
    pub dmx_port: dmx_serial::posix::TTYPort, //dmx_serial::Result<dmx_serial::posix::TTYPort>, // valid for life of the app
    pub light_records: Vec<(String, Vec<u8>)>, // a list of scene names plus all the slider values before any adjustment by the master slider and master alaways zero
    pub light_records_index: usize,            // initialized to zero
    pub is_fade_up: bool,
    pub is_fade_down: bool,
    pub short_text: String,
    pub is_blackout: bool,
    pub is_shimmer: bool,
    pub shimmer_instant: Instant,
    pub shimmer_duration: Duration,
    pub shimmer_master_value: u8,
    pub shimmer_amplitude_percent: f64,
    pub shimmer_frequency_hertz: f64,
}

fn configure_text_styles(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(30.0, Proportional)),
        (Body, FontId::new(14.0, Proportional)),
        (Monospace, FontId::new(18.0, Proportional)),
        (Button, FontId::new(18.0, Proportional)),
        (Small, FontId::new(18.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

impl Default for LightsApp {
    fn default() -> Self {
        let slider_count: usize = 21;
        Self {
            slider_count: slider_count,
            // set all sliders to zero
            values: vec![0; slider_count],
            // make sure list has length equal to slider_count
            is_master_adjusteds: vec![
                true, true, true, true, true, true, true, true, true, true, true, true, false,
                false, false, false, true, true, true, true, false,
            ],
            // make sure list has length equal to slider_count
            labels: vec![
                "P 1".to_string(),
                "P 2".to_string(),
                "P 3".to_string(),
                "P 4".to_string(),
                "F Red".to_string(),
                "F Grn".to_string(),
                "F Blu".to_string(),
                "F Wht".to_string(),
                "W Red".to_string(),
                "W Grn".to_string(),
                "W Blu".to_string(),
                "W Wht".to_string(),
                "Pan".to_string(),
                "Tilt".to_string(),
                "Zoom".to_string(),
                "Mstr".to_string(),
                "Red".to_string(),
                "Grn".to_string(),
                "Blu".to_string(),
                "Wht".to_string(),
                "Master".to_string(),
            ],
            values_adjusted: vec![0; slider_count],
            instant: Instant::now(), // func is only called once, so this value will be fixed
            duration: Duration::from_secs(0), // store elapsed time on each screen repaint
            #[cfg(target_arch = "aarch64")]
            dmx_port: dmx::open_serial("/dev/serial0").unwrap(), // create the serial port
            // light_records: vec![
            //     vec![0; slider_count],
            //     vec![128; slider_count],
            //     vec![255; slider_count],
            // ],
            light_records: json_storage::read_from_file().unwrap(),
            light_records_index: 0,
            is_fade_up: false,
            is_fade_down: false,
            short_text: "".to_string(),
            //long_text: "Last Scene of Pantomime".to_string(),
            is_blackout: false,
            is_shimmer: false,
            shimmer_instant: Instant::now(),
            shimmer_duration: Duration::from_secs(0), //store elapsed time until time for repeat cycle
            shimmer_master_value: 0,
            shimmer_amplitude_percent: 25.0,
            shimmer_frequency_hertz: 3.0,
        }
    }
}

impl LightsApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        configure_text_styles(&cc.egui_ctx);

        Default::default()
    }
}

impl eframe::App for LightsApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // TURN THIS OFF - we want our own light state
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        // we want to update continuously, so that we can fade by incrementing master slider value up and dowm
        ctx.request_repaint();

        top_panel::get_me(self, ctx);

        left_panel::get_me(self, ctx);

        right_panel::get_me(self, ctx);

        central_panel::get_me(self, ctx);

        if (self.instant.elapsed() - self.duration) > Duration::from_millis(50) {
            // println!(
            //     ">50 ms elapsed since last repaint at {:?}",
            //     &self.instant.elapsed()
            // );
            //println!("light_records: {:?}", &self.light_records);
            // for vals in self.light_records.iter() {
            //     println!("vals: {:?}", &vals);
            // }
            // send a dmx packet, &Vec<u8> can be coerced to &[u8]
            #[cfg(target_arch = "aarch64")]
            let _ = self.dmx_port.send_dmx_packet(&self.values_adjusted);
            self.duration = self.instant.elapsed();
        } else {
            // leave duration as is to accumulate time
        }

        // increment the master dimmer, beware of overflow, clamp to 255 max
        if (self.values[self.slider_count - 1] < 255) && (self.is_fade_up == true) {
            utilities::increment_master(self);
            self.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                self.values.clone(),
                self.is_master_adjusteds.clone(),
                self.slider_count,
                self.is_blackout,
            )
        } else {
            self.is_fade_up = false;
        }

        // decrement the master dimmer, clamp to zero minimum
        if self.values[self.slider_count - 1] > 0 && self.is_fade_down == true {
            utilities::decrement_master(self);
            self.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                self.values.clone(),
                self.is_master_adjusteds.clone(),
                self.slider_count,
                self.is_blackout,
            )
        } else {
            self.is_fade_down = false;
        }

        // shimmer
        if self.is_shimmer == true {
            utilities::shimmer_master(self);
            self.values_adjusted = utilities::recalculate_lights_adjusted_no_borrow(
                self.values.clone(),
                self.is_master_adjusteds.clone(),
                self.slider_count,
                self.is_blackout,
            )
        }
    }
}
