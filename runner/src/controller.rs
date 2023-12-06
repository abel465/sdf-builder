use crate::{shaders, RustGPUShader};
use shaders::*;
use winit::event::{ElementState, MouseScrollDelta};
use winit::{dpi::PhysicalPosition, event::MouseButton};

pub trait Controller {
    fn new() -> Self
    where
        Self: Sized;
    fn mouse_input(&mut self, state: ElementState, button: MouseButton);
    fn mouse_move(&mut self, position: PhysicalPosition<f64>);
    fn mouse_scroll(&mut self, delta: MouseScrollDelta);
    fn update(&mut self, width: u32, height: u32, options: Options);
    fn push_constants(&self) -> &[u8];
}

pub fn new_controller(shader: RustGPUShader) -> Box<dyn Controller> {
    match shader {
        RustGPUShader::Mandelbrot => Box::new(mandelbrot::Controller::new()),
        RustGPUShader::RayMarching => Box::new(ray_marching::Controller::new()),
        RustGPUShader::RayMarching2D => Box::new(ray_marching_2d::Controller::new()),
        RustGPUShader::SierpinskiTriangle => Box::new(sierpinski_triangle::Controller::new()),
        RustGPUShader::KochSnowflake => Box::new(koch_snowflake::Controller::new()),
        RustGPUShader::SDFs2D => Box::new(sdfs_2d::Controller::new()),
    }
}
