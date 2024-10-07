use crate::{
    context::GraphicsContext,
    controller::Controller,
    render_pass::RenderPass,
    shader::CompiledShaderModules,
    ui::{Ui, UiState},
    window::UserEvent,
    Options,
};
use egui_winit::winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::EventLoopProxy,
    window::Window,
};

pub struct State<'a> {
    rpass: RenderPass,
    ctx: GraphicsContext<'a>,
    controller: Controller,
    ui: Ui,
    ui_state: UiState,
}

impl<'a> State<'a> {
    pub async fn new(
        window: &'a Window,
        event_proxy: EventLoopProxy<UserEvent>,
        compiled_shader_modules: CompiledShaderModules,
        options: Options,
    ) -> Self {
        let ctx = GraphicsContext::new(window, &options).await;

        let ui = Ui::new(window, event_proxy);

        let ui_state = UiState::new();

        let controller = Controller::new(window.inner_size());

        let rpass = RenderPass::new(
            &ctx,
            compiled_shader_modules,
            options,
            &controller.buffers(),
        );

        Self {
            rpass,
            controller,
            ctx,
            ui,
            ui_state,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width != 0 && size.height != 0 {
            self.ctx.config.width = size.width;
            self.ctx.config.height = size.height;
            self.ctx
                .surface
                .configure(&self.ctx.device, &self.ctx.config);
            self.controller.resize(size);
        }
    }

    pub fn keyboard_input(&mut self, event: KeyEvent) {
        self.controller.keyboard_input(event);
    }

    pub fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        self.controller.mouse_input(state, button);
    }

    pub fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        self.controller.mouse_move(position);
    }

    pub fn mouse_delta(&mut self, _position: (f64, f64)) {
        // self.controller.mouse_delta(position);
    }

    pub fn mouse_scroll(&mut self, _delta: MouseScrollDelta) {
        // self.controller.mouse_scroll(delta);
    }

    pub fn update(&mut self) {
        self.controller.update();
    }

    pub fn render(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        self.rpass.render(
            &self.ctx,
            window,
            &mut self.ui,
            &mut self.ui_state,
            &mut self.controller,
        )
    }

    pub fn update_and_render(&mut self, window: &Window) -> Result<(), wgpu::SurfaceError> {
        self.update();
        self.render(window)
    }

    pub fn ui_consumes_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        self.ui.consumes_event(window, event)
    }

    pub fn new_module(&mut self, new_module: CompiledShaderModules) {
        let buffers = self.controller.buffers();
        self.rpass.new_module(&self.ctx, new_module, &buffers);
    }

    pub fn new_buffers(&mut self) {
        self.rpass
            .new_buffers(&self.ctx, &self.controller.buffers());
    }

    pub fn set_vsync(&mut self, enable: bool) {
        self.ctx.set_vsync(enable);
    }
}
