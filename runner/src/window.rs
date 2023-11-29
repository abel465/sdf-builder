use crate::CompiledShaderModules;

use winit::{
    event_loop::{EventLoop, EventLoopBuilder},
    window::{self, WindowBuilder},
};

pub struct Window {
    pub event_loop: EventLoop<CompiledShaderModules>,
    pub window: window::Window,
}

impl Window {
    pub fn new() -> Self {
        let mut event_loop_builder = EventLoopBuilder::with_user_event();
        let event_loop = event_loop_builder.build();
        let window = WindowBuilder::new()
            .with_title("Rust GPU Shaders")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
            .build(&event_loop)
            .unwrap();

        Self { event_loop, window }
    }
}
