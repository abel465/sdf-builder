use crate::{
    context::GraphicsContext, controller::Controller, render_pass::RenderPass,
    shader::CompiledShaderModules, ui::Ui, window::Window, Options,
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
    egui_winit_state: egui_winit::State,
    ui: Ui,
    start_time: Instant,
}

impl State {
    pub async fn new(
        window: &Window,
        compiled_shader_modules: CompiledShaderModules,
        options: Options,
    ) -> Self {
        let ctx = GraphicsContext::new(&window, &options).await;

        let mut egui_state = egui_winit::State::new(&window.event_loop);
        egui_state.set_pixels_per_point(window.window.scale_factor() as f32);

        Self {
            rpass: RenderPass::new(&ctx, compiled_shader_modules, options),
            ctx,
            controller: Controller::new(),
            egui_winit_state: egui_state,
            ui: Ui::new(),
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
        self.rpass
            .render(&self.ctx, push_constants, &mut self.egui_winit_state, &window, &self.ui)
    }

    pub fn update_and_render(
        &mut self,
        window: &winit::window::Window,
    ) -> Result<(), wgpu::SurfaceError> {
        let push_constants = self.update();
        self.render(push_constants, &window)
    }

    pub fn ui_consumes_event(&mut self, event: &WindowEvent) -> bool {
        self.egui_winit_state.on_event(&self.ui.context, event).consumed
    }

    pub fn new_module(&mut self, new_module: CompiledShaderModules) {
        self.rpass.new_module(&self.ctx, new_module);
    }
}
