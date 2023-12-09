use crate::{shaders, RustGPUShader};
use egui::Context;
use shaders::*;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseScrollDelta};
use winit::{dpi::PhysicalPosition, event::MouseButton};

pub trait Controller {
    fn new(size: PhysicalSize<u32>) -> Self
    where
        Self: Sized;
    fn mouse_input(&mut self, state: ElementState, button: MouseButton);
    fn mouse_move(&mut self, position: PhysicalPosition<f64>);
    fn mouse_scroll(&mut self, delta: MouseScrollDelta);
    fn resize(&mut self, size: PhysicalSize<u32>);
    fn update(&mut self);
    fn push_constants(&self) -> &[u8];
    fn ui(&mut self, _ctx: &Context, _ui: &mut egui::Ui) {}
}

pub fn new_controller(shader: RustGPUShader, size: PhysicalSize<u32>) -> Box<dyn Controller> {
    match shader {
        RustGPUShader::Mandelbrot => Box::new(mandelbrot::Controller::new(size)),
        RustGPUShader::RayMarching => Box::new(ray_marching::Controller::new(size)),
        RustGPUShader::RayMarching2D => Box::new(ray_marching_2d::Controller::new(size)),
        RustGPUShader::SierpinskiTriangle => Box::new(sierpinski_triangle::Controller::new(size)),
        RustGPUShader::KochSnowflake => Box::new(koch_snowflake::Controller::new(size)),
        RustGPUShader::SDFs2D => Box::new(sdfs_2d::Controller::new(size)),
    }
}
