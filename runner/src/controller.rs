use glam::{vec2, Vec2};
use shared::ShaderConstants;
use winit::event::{ElementState, MouseScrollDelta};
use winit::{dpi::PhysicalPosition, event::MouseButton};

use crate::state::BaseShaderConstants;

#[derive(Clone, Copy)]
pub struct Controller {
    cursor: Vec2,
    drag_start: Vec2,
    drag_end: Vec2,
    camera: Vec2,
    zoom: f32,
    mouse_button_pressed: bool,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            cursor: Vec2::ZERO,
            drag_start: Vec2::ZERO,
            drag_end: Vec2::ZERO,
            camera: Vec2::ZERO,
            zoom: 1.0,
            mouse_button_pressed: false,
        }
    }

    pub fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            match state {
                ElementState::Pressed => {
                    self.mouse_button_pressed = true;
                }
                ElementState::Released => {
                    self.camera += self.drag_start - self.drag_end;
                    self.mouse_button_pressed = false;
                }
            }

            self.drag_start = self.cursor;
            self.drag_end = self.cursor;
        }
    }

    pub fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
        if self.mouse_button_pressed {
            self.drag_end = self.cursor;
        }
    }

    pub fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        let scalar: f32 = match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                if y < 0.0 {
                    1.0 - 0.1 * y
                } else {
                    1.0 / (1.0 + 0.1 * y)
                }
            }
            MouseScrollDelta::PixelDelta(p) => {
                let v = 1.0 + 0.1 * (1.0_f32 + p.y.abs() as f32).ln();
                if p.y < 0.0 {
                    v
                } else {
                    1.0 / v
                }
            }
        };
        self.zoom *= scalar;
        self.camera *= 1.0 / scalar;
    }

    pub fn update(&mut self, constants: BaseShaderConstants) -> ShaderConstants {
        ShaderConstants {
            width: constants.width,
            height: constants.height,
            time: constants.time,
            cursor_x: self.cursor.x,
            cursor_y: self.cursor.y,
            drag_start_x: self.drag_start.x,
            drag_start_y: self.drag_start.y,
            drag_end_x: self.drag_end.x,
            drag_end_y: self.drag_end.y,
            zoom: self.zoom,
            translate_x: self.camera.x,
            translate_y: self.camera.y,
            mouse_button_pressed: !(1 << self.mouse_button_pressed as u32),
            mouse_button_press_time: [0.0, 0.0, 0.0],
        }
    }
}
