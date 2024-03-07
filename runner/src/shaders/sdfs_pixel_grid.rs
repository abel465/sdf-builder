use bytemuck::Zeroable;
use glam::{vec2, Vec2};
use shared::{
    push_constants::sdfs_pixel_grid::{Grid, ShaderConstants, NUM_X, NUM_Y},
    sdf_2d as sdf,
};
use std::time::{Duration, Instant};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta},
};

use crate::controller::BufferData;

pub struct Controller {
    size: PhysicalSize<u32>,
    start: Instant,
    elapsed: Duration,
    cursor: Vec2,
    mouse_button_pressed: bool,
    can_drag: Option<usize>,
    drag_point: Option<usize>,
    shader_constants: ShaderConstants,
    zoom: f32,
    buffer: Grid,
}

impl crate::controller::Controller for Controller {
    fn new(size: PhysicalSize<u32>) -> Self {
        let mut buffer = Grid::zeroed();

        for i in 0..NUM_X {
            for j in 0..NUM_Y / 4 {
                for k in 0..4 {
                    let x = (i as f32 / NUM_X as f32 - 0.5) * (NUM_X as f32 / NUM_Y as f32);
                    let y = (j * 4 + k) as f32 / NUM_Y as f32 - 0.5;
                    let p = vec2(x, y);
                    buffer[i][j][k] = sdf(p);
                    println!("{},{},{} {} {}", i, j, k, p, buffer[i][j][k]);
                    let ii = ((p.x + 0.5) * NUM_X as f32).round() as usize;
                    let jj = ((p.y + 0.5) * NUM_Y as f32).round() as usize;
                    println!("{},{},{}", ii, jj / 4, jj % 4);
                }
            }
        }

        Self {
            size,
            start: Instant::now(),
            elapsed: Duration::ZERO,
            cursor: Vec2::ZERO,
            mouse_button_pressed: false,
            can_drag: None,
            drag_point: None,
            shader_constants: ShaderConstants::zeroed(),
            zoom: 1.0,
            buffer,
        }
    }

    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            match state {
                ElementState::Pressed => {
                    self.drag_point = self.can_drag;
                    self.mouse_button_pressed = true;
                }
                ElementState::Released => {
                    self.drag_point = None;
                    self.mouse_button_pressed = false;
                }
            }
        }
    }

    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.cursor = vec2(position.x as f32, position.y as f32);
    }

    fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        self.zoom *= match delta {
            MouseScrollDelta::LineDelta(_, y) => {
                let v = 1.0 + 0.1 * y.abs();
                if y < 0.0 {
                    v
                } else {
                    1.0 / v
                }
            }
            MouseScrollDelta::PixelDelta(p) => {
                let v = 1.0 + 0.02 * (1.0 + p.y.abs() as f32).ln();
                if p.y < 0.0 {
                    v
                } else {
                    1.0 / v
                }
            }
        };
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size
    }

    fn update(&mut self) {
        self.elapsed = self.start.elapsed();
        self.shader_constants = ShaderConstants {
            size: self.size.into(),
            time: self.elapsed.as_secs_f32(),
            cursor: self.cursor.into(),
            mouse_button_pressed: !(1
                << (self.mouse_button_pressed && self.drag_point.is_none()) as u32),
            zoom: self.zoom,
        };
    }

    fn push_constants(&self) -> &[u8] {
        bytemuck::bytes_of(&self.shader_constants)
    }

    fn has_ui(&self) -> bool {
        false
    }

    fn buffers(&self) -> BufferData {
        BufferData {
            uniform: Some(bytemuck::cast_slice(&self.buffer)),
            ..Default::default()
        }
    }
}

fn sdf(p: Vec2) -> f32 {
    sdf::disk(p, 0.3)
}
