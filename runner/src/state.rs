use crate::{
    context::GraphicsContext,
    controller::{new_controller, Controller},
    fps_counter::FpsCounter,
    render_pass::RenderPass,
    shader::{self, CompiledShaderModules},
    ui::{Ui, UiState},
    window::Window,
    Options, RustGPUShader,
};
use strum::IntoEnumIterator;

use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
};

pub struct State {
    rpass: RenderPass,
    ctx: GraphicsContext,
    controllers: Vec<Box<dyn Controller>>,
    ui: Ui,
    ui_state: UiState,
    fps_counter: FpsCounter,
}

impl State {
    pub async fn new(
        window: &Window,
        compiled_shader_modules: CompiledShaderModules,
        options: Options,
    ) -> Self {
        let ctx = GraphicsContext::new(&window.window, &options).await;

        let ui = Ui::new(window);

        let ui_state = UiState::new(options.shader);

        Self {
            rpass: RenderPass::new(&ctx, compiled_shader_modules, options),
            ctx,
            controllers: RustGPUShader::iter().map(|s| new_controller(s)).collect(),
            ui,
            ui_state,
            fps_counter: FpsCounter::new(),
        }
    }

    fn controller(&mut self) -> &mut Box<dyn Controller> {
        &mut self.controllers[self.ui_state.active_shader as usize]
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width != 0 && size.height != 0 {
            self.ctx.config.width = size.width;
            self.ctx.config.height = size.height;
            self.ctx
                .surface
                .configure(&self.ctx.device, &self.ctx.config);
        }
    }

    pub fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        self.controller().mouse_input(state, button);
    }

    pub fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.controller().mouse_move(position);
    }

    pub fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        self.controller().mouse_scroll(delta);
    }

    pub fn update(&mut self) {
        self.controllers[self.ui_state.active_shader as usize].update(
            self.ctx.config.width,
            self.ctx.config.height,
            &mut self.ui_state.options,
        );
    }

    pub fn render(&mut self, window: &winit::window::Window) -> Result<(), wgpu::SurfaceError> {
        self.ui_state.width = self.ctx.config.width;
        self.ui_state.height = self.ctx.config.height;
        self.ui_state.fps = self.fps_counter.tick();
        let push_constants =
            self.controllers[self.ui_state.active_shader as usize].push_constants();

        self.rpass.render(
            &self.ctx,
            push_constants,
            window,
            &mut self.ui,
            &mut self.ui_state,
        )
    }

    pub fn update_and_render(
        &mut self,
        window: &winit::window::Window,
    ) -> Result<(), wgpu::SurfaceError> {
        self.update();
        self.render(&window)
    }

    pub fn ui_consumes_event(&mut self, event: &WindowEvent) -> bool {
        self.ui.consumes_event(event)
    }

    pub fn new_module(&mut self, shader: RustGPUShader, new_module: CompiledShaderModules) {
        self.ui_state.active_shader = shader;
        self.rpass.new_module(&self.ctx, new_module);
    }

    pub fn switch_shader(&mut self, shader: RustGPUShader) {
        self.new_module(
            shader,
            shader::maybe_watch(
                &Options {
                    force_spirv_passthru: false,
                    shader,
                },
                None,
            ),
        )
    }

    pub fn toggle_vsync(&mut self, enabled: bool) {
        self.ctx.config.present_mode = if enabled {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        };
        self.ctx
            .surface
            .configure(&self.ctx.device, &self.ctx.config);
    }
}
