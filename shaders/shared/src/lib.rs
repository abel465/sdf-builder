#![cfg_attr(target_arch = "spirv", no_std)]

pub mod push_constants;
pub mod sdf_interpreter;
pub mod sdf_wrapper;
pub mod stack;

use push_constants::Size;
use spirv_std::glam::{vec2, Vec2, Vec4};

pub const SQRT_3: f32 = 1.7320508075688772;
pub use core::f32::consts::PI;

pub fn fullscreen_vs(vert_id: i32, out_pos: &mut Vec4) {
    let uv = vec2(((vert_id << 1) & 2) as f32, (vert_id & 2) as f32);
    let pos = 2.0 * uv - Vec2::ONE;

    *out_pos = pos.extend(0.0).extend(1.0);
}

pub fn saturate(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    let x = saturate((x - edge0) / (edge1 - edge0));
    // Evaluate polynomial
    x * x * (3.0 - 2.0 * x)
}

pub fn from_pixels(Vec2 { x, y }: Vec2, Size { width, height }: Size) -> Vec2 {
    (vec2(x, -y) - 0.5 * vec2(width as f32, -(height as f32))) / height as f32
}
