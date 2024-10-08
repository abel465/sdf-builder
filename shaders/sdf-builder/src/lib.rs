#![cfg_attr(target_arch = "spirv", no_std)]

use dfutils::gridref::*;
use push_constants::sdf_builder::ShaderConstants;
use sdf_wrapper::WrappedDistance;
use shared::*;
use spirv_std::glam::*;
#[cfg_attr(not(target_arch = "spirv"), allow(unused_imports))]
use spirv_std::num_traits::Float;
use spirv_std::spirv;

type T = WrappedDistance<u32>;

fn sdf(p: Vec2, grid: GridRef<T>) -> T {
    grid.signed_distance(p)
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(push_constant)] constants: &ShaderConstants,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] grid_buffer: &[T],
    output: &mut Vec4,
) {
    let uv = from_pixels(frag_coord.xy(), constants.size);

    let grid = GridRef::new(
        constants.size.width as usize,
        constants.size.height as usize,
        grid_buffer,
    );
    let T { d, data: id } = sdf(uv, grid);
    let mut col = if d < 0.0 {
        vec3(0.65, 0.85, 1.0)
    } else {
        vec3(0.9, 0.6, 0.3)
    };
    col *= 1.0 - (-20.0 * d.abs()).exp();
    col *= 0.8 + 0.2 * (300.0 * d).cos();
    if id == constants.selected_id {
        col = col.lerp(Vec3::ONE, 1.0 - smoothstep(0.0, 0.008, d.abs()));
    } else {
        col = col.lerp(Vec3::splat(0.5), 1.0 - smoothstep(0.0, 0.008, d.abs()));
    }

    *output = col.powf(2.2).extend(1.0);
}

#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    fullscreen_vs(vert_id, out_pos)
}
