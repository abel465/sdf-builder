use egui::{pos2, Context, FullOutput, RawInput, TopBottomPanel};

pub struct UiState {
    pub width: u32,
    pub height: u32,
    pub fps: usize,
    show_fps: bool,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            fps: 0,
            show_fps: false,
        }
    }
}

pub struct Ui {
    pub context: Context,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            context: Context::default(),
        }
    }

    pub fn prepare(&self, raw_input: RawInput, ui_state: &mut UiState) -> FullOutput {
        self.context.run(raw_input, |ctx| {
            self.ui(ctx, ui_state);
        })
    }

    fn ui(&self, ctx: &Context, ui_state: &mut UiState) {
        egui::Window::new("main")
            .title_bar(false)
            .resizable(false)
            .default_width(100.0)
            .show(ctx, |ui| {
                ui.heading("Shaders");
                ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                    ui.button("Mandelbrot");
                    ui.button("SDF's");
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
