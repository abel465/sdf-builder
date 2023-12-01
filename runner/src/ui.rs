use egui::{pos2, Context, FullOutput, RawInput};
use winit::event_loop::EventLoopProxy;

use crate::{window::UserEvent, RustGPUShader};

pub struct UiState {
    pub width: u32,
    pub height: u32,
    pub fps: usize,
    pub show_fps: bool,
    pub active_shader: RustGPUShader,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            fps: 0,
            show_fps: false,
            active_shader: RustGPUShader::Mandelbrot,
        }
    }
}

pub struct Ui {
    pub context: Context,
    event_proxy: EventLoopProxy<UserEvent>,
}

impl Ui {
    pub fn new(event_proxy: EventLoopProxy<UserEvent>) -> Self {
        Self {
            context: Context::default(),
            event_proxy,
        }
    }

    pub fn prepare(&self, raw_input: RawInput, ui_state: &mut UiState) -> FullOutput {
        self.context.run(raw_input, |ctx| {
            self.ui(ctx, ui_state);
        })
    }

    fn switch_shader(&self, shader: RustGPUShader) {
        let _ = self.event_proxy.send_event(UserEvent::SwitchShader(shader));
    }

    fn switch_shader_button(
        &self,
        ui: &mut egui::Ui,
        ui_state: &UiState,
        shader: RustGPUShader,
        label: &str,
    ) {
        if ui
            .selectable_label(ui_state.active_shader == shader, label)
            .clicked()
        {
            self.switch_shader(shader)
        }
    }

    fn ui(&self, ctx: &Context, ui_state: &mut UiState) {
        egui::Window::new("main")
            .title_bar(false)
            .resizable(false)
            .default_width(110.0)
            .show(ctx, |ui| {
                ui.heading("Shaders");
                ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                    self.switch_shader_button(
                        ui,
                        ui_state,
                        RustGPUShader::Mandelbrot,
                        "Mandelbrot",
                    );
                    self.switch_shader_button(
                        ui,
                        ui_state,
                        RustGPUShader::KochSnowflake,
                        "Koch Snowflake",
                    );
                    self.switch_shader_button(
                        ui,
                        ui_state,
                        RustGPUShader::SierpinskiTriangle,
                        "Sierpinski Triangle",
                    );
                    // ui.selectable_label(
                    //     ui_state.active_shader == ,
                    //     "Mandelbrot",
                    // )
                    // .clicked(self.switch_shader(shader));
                    // ui.selectable_label(
                    //     ui_state.active_shader == RustGPUShader::KochSnowflake,
                    //     "Koch Snowflake",
                    // );
                    // ui.selectable_label(
                    //     ui_state.active_shader == RustGPUShader::SierpinskiTriangle,
                    //     "Sierpinski Triangle",
                    // );
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
