use egui::Ui;
use shared::fast_optional::Optional_f32;
use std::ops::RangeInclusive;

#[derive(Clone, Copy)]
pub struct EnabledNumber {
    value: f32,
    enabled: bool,
}

impl EnabledNumber {
    pub fn new(value: f32, enabled: bool) -> Self {
        Self { value, enabled }
    }

    pub fn ui(&mut self, ui: &mut Ui, clamp_range: RangeInclusive<f32>, speed: f32) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.enabled, "Onion");
            ui.add_enabled(
                self.enabled,
                egui::DragValue::new(&mut self.value)
                    .clamp_range(clamp_range)
                    .speed(speed),
            );
        });
    }
}

impl Into<Optional_f32> for EnabledNumber {
    fn into(self) -> Optional_f32 {
        Optional_f32::new(if self.enabled {
            self.value
        } else {
            Optional_f32::NONE_VALUE
        })
    }
}
