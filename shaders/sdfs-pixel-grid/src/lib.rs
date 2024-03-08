#![cfg_attr(target_arch = "spirv", no_std)]

use push_constants::sdfs_pixel_grid::{Grid, ShaderConstants};
use shared::sdf_2d as sdf;
use shared::*;
use spirv_std::glam::{vec3, Vec2, Vec3, Vec4, Vec4Swizzles};
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

fn sdf(p: Vec2, grid: &Grid, smooth: bool) -> f32 {
    if smooth {
        grid.from_vec2_smooth(p)
    } else {
        grid.from_vec2(p)
    }
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] grid: &Grid,
    output: &mut Vec4,
) {
    let uv = constants.zoom * from_pixels(frag_coord.xy(), constants.size);
    let cursor = constants.zoom * from_pixels(constants.cursor.into(), constants.size);
    let smooth: bool = constants.smooth.into();

    let d = sdf(uv, grid, smooth);

    let mut col = if d < 0.0 {
        vec3(0.65, 0.85, 1.0)
    } else {
        vec3(0.9, 0.6, 0.3)
    };
    col *= 1.0 - (-6.0 * d.abs()).exp();
    col *= 0.8 + 0.2 * (150.0 * d).cos();
    col = col.lerp(Vec3::ONE, 1.0 - smoothstep(0.0, 0.01, d.abs()));

    if constants.mouse_button_pressed & 1 != 0 {
        let d = sdf(cursor, grid, smooth);
        let thickness = 1.0 / constants.size.height as f32;
        col = col
            .lerp(
                vec3(1.0, 1.0, 0.0),
                smoothstep(thickness, 0.0, sdf::disk(uv - cursor, 0.01)),
            )
            .lerp(
                vec3(1.0, 1.0, 0.0),
                smoothstep(
                    thickness,
                    0.0,
                    sdf::disk(uv - cursor, d.abs()).abs() - 0.0025,
                ),
            );
    }

    *output = col.extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
