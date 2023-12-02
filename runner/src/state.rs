use crate::{
    context::GraphicsContext,
    controller::Controller,
    fps_counter::FpsCounter,
    render_pass::RenderPass,
    shader::{self, CompiledShaderModules},
    ui::{Ui, UiState},
    window::Window,
    Options, RustGPUShader,
};
use std::time::Instant;

use shared::ShaderConstants;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
};

pub struct BaseShaderConstants {
    pub width: u32,
    pub height: u32,
    pub time: f32,
}

pub struct State {
    rpass: RenderPass,
    ctx: GraphicsContext,
    controller: Controller,
    ui: Ui,
    ui_state: UiState,
    fps_counter: FpsCounter,
    start_time: Instant,
}

impl State {
    pub async fn new(
        window: &Window,
        compiled_shader_modules: CompiledShaderModules,
        options: Options,
    ) -> Self {
        let ctx = GraphicsContext::new(&window, &options).await;

        let active_shader = options.shader;

        Self {
            rpass: RenderPass::new(&ctx, compiled_shader_modules, options),
            ctx,
            controller: Controller::new(),
            ui: Ui::new(window),
            ui_state: UiState::new(active_shader),
            fps_counter: FpsCounter::new(),
            start_time: Instant::now(),
        }
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
        self.controller.mouse_input(state, button);
    }

    pub fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.controller.mouse_move(position);
    }

    pub fn mouse_scroll(&mut self, delta: MouseScrollDelta) {
        self.controller.mouse_scroll(delta);
    }

    pub fn update(&mut self) -> ShaderConstants {
        self.controller.update(BaseShaderConstants {
            width: self.ctx.config.width,
            height: self.ctx.config.height,
            time: self.start_time.elapsed().as_secs_f32(),
        })
    }

    pub fn render(
        &mut self,
        push_constants: ShaderConstants,
        window: &winit::window::Window,
    ) -> Result<(), wgpu::SurfaceError> {
        self.ui_state.width = self.ctx.config.width;
        self.ui_state.height = self.ctx.config.height;
        self.ui_state.fps = self.fps_counter.tick();

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
        let push_constants = self.update();
        self.render(push_constants, &window)
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
}
