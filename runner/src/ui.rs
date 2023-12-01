use egui::{Context, FullOutput, RawInput, TopBottomPanel};

pub struct Ui {
    pub context: Context,
}

impl Ui {
    pub fn new() -> Self {
        let context = Context::default();

        Ui { context }
    }

    pub fn prepare(&self, raw_input: RawInput) -> FullOutput {
        self.context.run(raw_input, |ctx| {
            self.ui(ctx);
        })
    }

    fn ui(&self, ctx: &Context) {
        TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.heading("Menu");
        });
        egui::Window::new("Window").show(ctx, |ui| {
            ui.heading("window content");
        });
    }
}
