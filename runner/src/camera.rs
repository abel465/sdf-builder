use core::f32::consts::PI;
use glam::{Mat4, Quat, Vec2, Vec2Swizzles, Vec3};
use winit::dpi::PhysicalSize;

#[derive(Clone)]
pub struct RotationCamera {
    pub aspect: f32,
    pub zoom: f32,
    pub rot: Quat,
}

impl RotationCamera {
    pub fn new(aspect: f32, zoom: f32) -> Self {
        RotationCamera {
            aspect,
            zoom,
            rot: Quat::IDENTITY,
        }
    }

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(Vec3::Z * self.zoom, Vec3::ZERO, Vec3::Y);
        let proj = Mat4::perspective_rh(PI / 4.0, self.aspect, 0.01, 100.0);
        let rot = Mat4::from_quat(self.rot);
        proj * view * rot
    }

    pub fn rotate(&mut self, v: Vec2) {
        let p = (v.yx() * 4.0).extend(0.0);
        self.rot = Quat::from_scaled_axis(p).mul_quat(self.rot).normalize();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.aspect = size.width as f32 / size.height as f32;
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.zoom *= zoom;
    }
}
