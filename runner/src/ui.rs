use egui::{
    epaint::{textures::TexturesDelta, ClippedPrimitive},
    pos2, Context, Layout,
};
use winit::{event::WindowEvent, event_loop::EventLoopProxy};

use crate::{
    window::{UserEvent, Window},
    RustGPUShader,
};

pub struct UiState {
    pub width: u32,
    pub height: u32,
    pub fps: usize,
    pub show_fps: bool,
    pub active_shader: RustGPUShader,
}

impl UiState {
    pub fn new(active_shader: RustGPUShader) -> Self {
        Self {
            width: 0,
            height: 0,
            fps: 0,
            show_fps: false,
            active_shader,
        }
    }
}

pub struct Ui {
    context: Context,
    egui_winit_state: egui_winit::State,
    event_proxy: EventLoopProxy<UserEvent>,
}

impl Ui {
    pub fn new(window: &Window) -> Self {
        let event_loop = &window.event_loop;
        let mut egui_winit_state = egui_winit::State::new(event_loop);
        egui_winit_state.set_pixels_per_point(window.window.scale_factor() as f32);

        Self {
            context: Context::default(),
            egui_winit_state,
            event_proxy: event_loop.create_proxy(),
        }
    }

    pub fn consumes_event(&mut self, event: &WindowEvent) -> bool {
        self.egui_winit_state
            .on_event(&self.context, &event)
            .consumed
    }

    pub fn prepare(
        &mut self,
        window: &winit::window::Window,
        ui_state: &mut UiState,
    ) -> (Vec<ClippedPrimitive>, TexturesDelta) {
        let raw_input = self.egui_winit_state.take_egui_input(&window);
        let full_output = self.context.run(raw_input, |ctx| {
            self.ui(ctx, ui_state);
        });
        self.egui_winit_state.handle_platform_output(
            &window,
            &self.context,
            full_output.platform_output,
        );
        let clipped_primitives = self.context.tessellate(full_output.shapes);
        (clipped_primitives, full_output.textures_delta)
    }

    fn switch_shader(&self, shader: RustGPUShader) {
        let _ = self.event_proxy.send_event(UserEvent::SwitchShader(shader));
    }

    fn ui(&self, ctx: &Context, ui_state: &mut UiState) {
        egui::Window::new("main")
            .title_bar(false)
            .resizable(false)
            .default_width(110.0)
            .show(ctx, |ui| {
                ui.heading("Shaders");
                ui.with_layout(Layout::default().with_cross_justify(true), |ui| {
                    for (shader, name) in [
                        (RustGPUShader::Mandelbrot, "Mandelbrot"),
                        (RustGPUShader::KochSnowflake, "Koch Snowflake"),
                        (RustGPUShader::SierpinskiTriangle, "Sierpinski Triangle"),
                    ] {
                        if ui
                            .selectable_label(ui_state.active_shader == shader, name)
                            .clicked()
                        {
                            if ui_state.active_shader != shader {
                                self.switch_shader(shader)
                            }
                        }
                    }
                });
                ui.separator();
                ui.checkbox(&mut ui_state.show_fps, "fps counter")
            });
        if ui_state.show_fps {
            egui::Window::new("fps")
                .title_bar(false)
                .resizable(false)
                .interactable(false)
                .fixed_pos(pos2(ui_state.width as f32 - 70.0, 10.0))
                .show(ctx, |ui| {
                    ui.label(format!("FPS: {}", ui_state.fps));
                });
        }
    }
}
