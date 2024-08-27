use crate::TemplateApp;

pub fn recalculate_lights_adjusted(template_app: &mut TemplateApp) {
    template_app.values_adjusted = template_app
        .values
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            if template_app.is_master_adjusteds[i] == true {
                ((f64::from(v) * f64::from(template_app.value_master)) / 255.0) as u8
            } else {
                v
            }
        })
        .collect();
}
