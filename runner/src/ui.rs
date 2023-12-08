use egui::{
    epaint::{textures::TexturesDelta, ClippedPrimitive},
    pos2, Context, CursorIcon, Layout,
};
use shared::push_constants::sdfs_2d::Shape;
use strum::IntoEnumIterator;
use winit::{event::WindowEvent, event_loop::EventLoopProxy};

use crate::{
    shaders,
    window::{UserEvent, Window},
    RustGPUShader,
};

pub struct UiState {
    pub width: u32,
    pub height: u32,
    pub fps: usize,
    pub show_fps: bool,
    pub vsync: bool,
    pub active_shader: RustGPUShader,
    pub cursor_icon: CursorIcon,
    pub options: shaders::Options,
}

impl UiState {
    pub fn new(active_shader: RustGPUShader) -> Self {
        Self {
            width: 0,
            height: 0,
            fps: 0,
            show_fps: false,
            vsync: true,
            active_shader,
            cursor_icon: CursorIcon::default(),
            options: shaders::Options::new(),
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

    fn shader_options(&self, ctx: &Context, ui_state: &mut UiState, ui: &mut egui::Ui) {
        match ui_state.active_shader {
            RustGPUShader::Mandelbrot => {
                let options = &mut ui_state.options.mandelbrot;
                ui.horizontal(|ui| {
                    ui.label("Exponent:");
                    ui.add(
                        egui::DragValue::new(&mut options.exponent)
                            .clamp_range(1.0..=6.0)
                            .speed(0.1),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Num Iterations:");
                    ui.add(
                        egui::DragValue::new(&mut options.num_iterations)
                            .clamp_range(2..=200)
                            .speed(1),
                    );
                });
            }
            RustGPUShader::RayMarching => {}
            RustGPUShader::RayMarching2D => {}
            RustGPUShader::SierpinskiTriangle => {}
            RustGPUShader::KochSnowflake => {
                let options = &mut ui_state.options.koch_snowflake;
                ui.radio_value(&mut options.use_antisnowflake, false, "Snowflake");
                ui.radio_value(&mut options.use_antisnowflake, true, "AntiSnowflake");
            }
            RustGPUShader::SDFs2D => {
                let options = &mut ui_state.options.sdfs_2d;
                ctx.set_cursor_icon(if options.is_dragging {
                    CursorIcon::Grabbing
                } else if options.can_drag {
                    CursorIcon::Grab
                } else {
                    CursorIcon::Default
                });
                for shape in Shape::iter() {
                    ui.radio_value(&mut options.shape, shape, shape.to_string());
                }
                let spec = options.shape.spec();
                if spec.num_dims > 0 {
                    let params = &mut options.params[options.shape as usize];
                    let (dim1_max, dim2_max, dim1_label, dim2_label) = {
                        if spec.is_radial {
                            (0.5, params.dim1, "Radius", "Radius2")
                        } else {
                            (
                                (ui_state.width as f32) / (ui_state.height as f32),
                                1.0,
                                "Width",
                                "Height",
                            )
                        }
                    };
                    ui.horizontal(|ui| {
                        ui.label(dim1_label);
                        ui.add(
                            egui::DragValue::new(&mut params.dim1)
                                .clamp_range(0.0..=dim1_max)
                                .speed(0.01),
                        );
                    });
                    if spec.num_dims > 1 {
                        ui.horizontal(|ui| {
                            ui.label(dim2_label);
                            ui.add(
                                egui::DragValue::new(&mut params.dim2)
                                    .clamp_range(0.0..=dim2_max)
                                    .speed(0.01),
                            );
                        });
                    }
                }
            }
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

    fn send_event(&self, event: UserEvent) {
        let _ = self.event_proxy.send_event(event);
    }

    fn ui(&self, ctx: &Context, ui_state: &mut UiState) {
        egui::Window::new("main")
            .title_bar(false)
            .resizable(false)
            .default_width(128.0)
            .show(ctx, |ui| {
                ui.heading("Shaders");
                ui.with_layout(Layout::default().with_cross_justify(true), |ui| {
                    for shader in RustGPUShader::iter() {
                        if ui
                            .selectable_label(ui_state.active_shader == shader, shader.to_string())
                            .clicked()
                        {
                            if ui_state.active_shader != shader {
                                self.send_event(UserEvent::SwitchShader(shader));
                            }
                        }
                    }
                });
                ui.separator();
                self.shader_options(ctx, ui_state, ui);
                ui.separator();
                ui.checkbox(&mut ui_state.show_fps, "fps counter");
                if ui.checkbox(&mut ui_state.vsync, "V-Sync").clicked() {
                    self.send_event(UserEvent::ToggleVSync(ui_state.vsync));
                }
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
