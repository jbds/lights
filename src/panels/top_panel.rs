use crate::LightsApp;

pub fn get_me(lights_app: &mut LightsApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //ui.label("top_panel_placeholder");
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Minimize").clicked() {
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
            ui.label(format!("{:?}", lights_app.values));
            ui.add_space(16.0);
            ui.label(format!("{:?}", lights_app.values_adjusted));
            ui.add_space(16.0);
            ui.label(format!("{:?}", lights_app.master_value_f64));
        });
    });
}
