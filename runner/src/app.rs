use crate::CompiledShaderModules;

use winit::{
    event_loop::{EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder},
};

pub struct Options {
    pub force_spirv_passthru: bool,
}

pub struct App {
    pub event_loop: EventLoop<CompiledShaderModules>,
    pub window: Window,
    pub options: Options,
}

impl App {
    pub fn new() -> Self {
        let mut event_loop_builder = EventLoopBuilder::with_user_event();
        let event_loop = event_loop_builder.build();
        let window = WindowBuilder::new()
            .with_title("Rust GPU Shaders")
            .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
            .build(&event_loop)
            .unwrap();
        let options = Options {
            force_spirv_passthru: false,
        };

        Self {
            event_loop,
            window,
            options,
        }
    }
}
