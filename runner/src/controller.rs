use crate::model::Vertex;
use crate::window::UserEvent;
use crate::{shaders, RustGPUShader};
use egui::{Context, Ui};
use shaders::*;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, MouseScrollDelta};
use winit::event_loop::EventLoopProxy;
use winit::{dpi::PhysicalPosition, event::MouseButton};

#[derive(Clone, Copy)]
pub struct BufferData<'a> {
    pub vertex: Option<&'a [Vertex]>,
    pub index: Option<&'a [u32]>,
    pub uniform: Option<&'a [u8]>,
    pub use_depth_buffer: bool,
}

impl<'a> Default for BufferData<'a> {
    fn default() -> Self {
        Self {
            vertex: None,
            index: None,
            uniform: None,
            use_depth_buffer: false,
        }
    }
}

pub trait Controller {
    fn new(size: PhysicalSize<u32>) -> Self
    where
        Self: Sized;
    fn mouse_input(&mut self, _state: ElementState, _button: MouseButton) {}
    fn mouse_move(&mut self, _position: PhysicalPosition<f64>) {}
    fn mouse_scroll(&mut self, _delta: MouseScrollDelta) {}
    fn resize(&mut self, size: PhysicalSize<u32>);
    fn update(&mut self);
    fn push_constants(&self) -> &[u8];
    fn ui(&mut self, _ctx: &Context, _ui: &mut Ui, _event_proxy: &EventLoopProxy<UserEvent>) {}
    fn has_ui(&self) -> bool {
        false
    }
    fn buffers(&self) -> BufferData {
        BufferData::default()
    }
}

pub fn new_controller(shader: RustGPUShader, size: PhysicalSize<u32>) -> Box<dyn Controller> {
    match shader {
        RustGPUShader::Mandelbrot => Box::new(mandelbrot::Controller::new(size)),
        RustGPUShader::RayMarching => Box::new(ray_marching::Controller::new(size)),
        RustGPUShader::RayMarching2D => Box::new(ray_marching_2d::Controller::new(size)),
        RustGPUShader::SierpinskiTriangle => Box::new(sierpinski_triangle::Controller::new(size)),
        RustGPUShader::KochSnowflake => Box::new(koch_snowflake::Controller::new(size)),
        RustGPUShader::SDFs2D => Box::new(sdfs_2d::Controller::new(size)),
        RustGPUShader::SDFs3D => Box::new(sdfs_3d::Controller::new(size)),
        RustGPUShader::HydrogenWavefunction => {
            Box::new(hydrogen_wavefunction::Controller::new(size))
        }
        RustGPUShader::SphericalHarmonics => Box::new(spherical_harmonics::Controller::new(size)),
        RustGPUShader::SphericalHarmonicsShape => {
            Box::new(spherical_harmonics_shape::Controller::new(size))
        }
        RustGPUShader::FunRepDemo => Box::new(fun_rep_demo::Controller::new(size)),
        RustGPUShader::SdfsPixelGrid => Box::new(sdfs_pixel_grid::Controller::new(size)),
    }
}
