pub mod koch_snowflake;
pub mod mandelbrot;
pub mod ray_marching;
pub mod ray_marching_2d;
pub mod sierpinski_triangle;
pub mod sdfs_2d;

#[derive(Clone)]
pub struct Options {
    pub mandelbrot: mandelbrot::Options,
    pub koch_snowflake: koch_snowflake::Options,
    pub sierpinski_triangle: sierpinski_triangle::Options,
    pub ray_marching: ray_marching::Options,
    pub ray_marching_2d: ray_marching_2d::Options,
    pub sdfs_2d: sdfs_2d::Options,
}

impl Options {
    pub fn new() -> Self {
        Self {
            mandelbrot: mandelbrot::Options::new(),
            koch_snowflake: koch_snowflake::Options::new(),
            sierpinski_triangle: sierpinski_triangle::Options::new(),
            ray_marching: ray_marching::Options::new(),
            ray_marching_2d: ray_marching_2d::Options::new(),
            sdfs_2d: sdfs_2d::Options::new(),
        }
    }
}
