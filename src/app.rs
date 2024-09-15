use crate::central_panel;
use crate::utilities::recalculate_lights_adjusted;
use dmx::{self, DmxTransmitter};
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::time::{Duration, Instant};

// #[derive(Clone, Debug)]
// pub struct Light {
//     pub value: u8,
//     pub is_master_adjusted: bool,
//     pub label: String,
// }

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LightsApp {
    // Example stuff:
    _label: String,

    //#[serde(skip)] // This how you opt-out of serialization of a field
    pub value: f32,      //made this public
    pub values: Vec<u8>, //Vec<u8>, //stores the current array of light values
    pub value_master: u8,
    pub is_master_adjusteds: Vec<bool>,
    pub labels: Vec<String>,
    pub values_adjusted: Vec<u8>,
    pub instant: Instant,   // we need this to check timing
    pub duration: Duration, // ditto
    #[cfg(target_arch = "aarch64")]
    pub dmx_port: dmx_serial::posix::TTYPort, //dmx_serial::Result<dmx_serial::posix::TTYPort>, // valid for life of the app
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
        Self {
            // Example stuff:
            _label: "Hello World!".to_owned(),
            value: 2.7,
            // initialize 20 lights with 3 attributes each
            values: vec![0; 20],
            is_master_adjusteds: vec![
                true, true, true, true, true, true, true, true, true, true, true, true, false,
                false, false, false, true, true, true, true,
            ],
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
            ],
            value_master: 255,
            values_adjusted: vec![0; 20],
            instant: Instant::now(), // func is only called once, so this value will be fixed
            duration: Duration::from_secs(0), // store elapsed time on each screen repaint
            #[cfg(target_arch = "aarch64")]
            dmx_port: dmx::open_serial("/dev/serial0").unwrap(), // create the serial port
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Shrink").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);

                ui.add_space(16.0);
                ui.label(format!(
                    "{:?}",
                    ctx.input(|i: &egui::InputState| i.screen_rect())
                ));
                ui.label(format!("{:?}", self.values));
                ui.add_space(16.0);
                ui.label(format!("{:?}", self.values_adjusted));
            });
        });

        egui::SidePanel::right("rhs_panel")
            .show_separator_line(false)
            .min_width(70.0)
            .resizable(false)
            .show(ctx, |ui| {
                //ui.label("MM");
                // set the 'width' (height) of the slider
                ui.spacing_mut().slider_width = 600.0;
                let resp = ui.add(
                    egui::Slider::new(&mut self.value_master, 0..=255)
                        .integer()
                        .text("Master")
                        .orientation(egui::SliderOrientation::Vertical),
                );
                if resp.changed() == true {
                    recalculate_lights_adjusted(self);
                }
            });

        //let my_closure = |ui: &mut egui::Ui| ui.heading("jonb zzzzz sales@jbds.co.uk");
        egui::CentralPanel::default().show(ctx, central_panel::get_closure(self));

        if (self.instant.elapsed() - self.duration) > Duration::from_millis(50) {
            println!(
                ">50 ms elapsed since last repaint at {:?}",
                &self.instant.elapsed()
            );
            // send a dmx packet, &Vec<u8> can be coerced to &[u8]
            #[cfg(target_arch = "aarch64")]
            let _ = self.dmx_port.send_dmx_packet(&self.values_adjusted);
            self.duration = self.instant.elapsed();
        } else {
            // leave duration as is to accumulate time
        }
    }
}
