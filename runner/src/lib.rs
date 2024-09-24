use structopt::StructOpt;

mod app;
mod bind_group_buffer;
mod context;
mod egui_components;
mod fps_counter;
mod render_pass;
mod shader;
mod controller;
mod state;
mod ui;
mod window;

#[derive(StructOpt, Clone, Copy)]
#[structopt(name = "sdf-builder")]
pub struct Options {
    // Default to true after the following is fixed
    // https://github.com/gfx-rs/wgpu/issues/5128
    #[structopt(long)]
    validate_spirv: bool,
}

pub fn main() {
    let options: Options = Options::from_args();

    app::start(options);
}
