use egui::emath::Numeric;
use egui::Ui;
use shared::fast_optional::Optional_f32;
use std::ops::RangeInclusive;

#[derive(Clone, Copy, PartialEq)]
pub struct EnabledNumber<T: Numeric> {
    pub value: T,
    pub enabled: bool,
}

impl<T: Numeric> EnabledNumber<T> {
    pub fn new(value: T, enabled: bool) -> Self {
        Self { value, enabled }
    }

    pub fn ui(&mut self, ui: &mut Ui, text: &str, clamp_range: RangeInclusive<T>, speed: f32) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.enabled, text);
            ui.add_enabled(
                self.enabled,
                egui::DragValue::new(&mut self.value)
                    .clamp_range(clamp_range)
                    .speed(speed),
            );
        });
    }
}

impl Into<Optional_f32> for EnabledNumber<f32> {
    fn into(self) -> Optional_f32 {
        Optional_f32::new(if self.enabled {
            self.value
        } else {
            Optional_f32::NONE_VALUE
        })
    }
}

// #[derive(Clone, Copy)]
// pub struct TwoEnabledNumbers<T: Numeric> {
//     pub value1: T,
//     pub value2: T,
//     pub enabled: bool,
// }
//
// impl<T: Numeric> TwoEnabledNumbers<T> {
//     pub fn new(value1: T, value2: T, enabled: bool) -> Self {
//         Self {
//             value1,
//             value2,
//             enabled,
//         }
//     }
//
//     pub fn ui(
//         &mut self,
//         ui: &mut Ui,
//         text: &str,
//         clamp_range: RangeInclusive<T>,
//         speed: impl Into<f64>,
//     ) {
//         let speed: f64 = speed.into();
//         ui.horizontal(|ui| {
//             ui.checkbox(&mut self.enabled, text);
//             ui.add_enabled(
//                 self.enabled,
//                 egui::DragValue::new(&mut self.value1)
//                     .clamp_range(clamp_range.clone())
//                     .speed(speed),
//             );
//             ui.add_enabled(
//                 self.enabled,
//                 egui::DragValue::new(&mut self.value2)
//                     .clamp_range(clamp_range)
//                     .speed(speed),
//             );
//         });
//     }
// }
//
// impl Into<[Optional_f32; 2]> for TwoEnabledNumbers<f32> {
//     fn into(self) -> [Optional_f32; 2] {
//         if self.enabled {
//             [
//                 Optional_f32::new(self.value1),
//                 Optional_f32::new(self.value2),
//             ]
//         } else {
//             [Optional_f32::NONE; 2]
//         }
//     }
// }
