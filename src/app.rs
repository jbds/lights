use crate::central_panel;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    _label: String,

    //#[serde(skip)] // This how you opt-out of serialization of a field
    pub value: f32,      //made this public
    pub values: Vec<u8>, //stores the current array of light valuyes
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            _label: "Hello World!".to_owned(),
            value: 2.7,
            values: vec![0; 20],
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
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
            });
        });

        egui::SidePanel::right("rhs_panel")
            .show_separator_line(false)
            .min_width(20.0)
            .show(ctx, |ui| {
                //ui.label("MM");
                // set the 'width' (height) of the slider
                ui.spacing_mut().slider_width = 600.0;
                ui.add(
                    egui::Slider::new(&mut self.value, 0.0..=255.0)
                        .integer()
                        .text("Master")
                        .orientation(egui::SliderOrientation::Vertical),
                )
            });

        //let my_closure = |ui: &mut egui::Ui| ui.heading("jonb zzzzz sales@jbds.co.uk");
        egui::CentralPanel::default().show(ctx, central_panel::get_closure(self));
    }
}
