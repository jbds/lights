use crate::TemplateApp;

pub fn recalculate_lights_dependent(template_app: &mut TemplateApp) {
    template_app.values_adjusted = template_app
        .values
        .iter()
        .map(|&v| ((f64::from(v) * f64::from(template_app.value_master)) / 255.0) as u8)
        .collect();
}
