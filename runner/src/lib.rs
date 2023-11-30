use structopt::StructOpt;
use strum::{Display, EnumString};

mod context;
mod controller;
mod graphics;
mod render_pass;
mod shader;
mod state;
mod window;

#[derive(EnumString, Display, PartialEq, Eq, Copy, Clone)]
pub enum RustGPUShader {
    Mandelbrot,
    RayMarching,
    RayMarching2D,
    SierpinskiTriangle,
    KochSnowflake,
}

#[derive(StructOpt, Clone)]
#[structopt(name = "example-runner-wgpu")]
pub struct Options {
    #[structopt(short, long, default_value = "Mandelbrot")]
    shader: RustGPUShader,

    #[structopt(long)]
    force_spirv_passthru: bool,
}

pub fn main() {
    let options: Options = Options::from_args();

    graphics::start(&options);
}
