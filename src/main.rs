extern crate gl;
extern crate glutin;
#[macro_use] extern crate log;
extern crate freetype;
extern crate nalgebra;
extern crate time;
extern crate libc;

use paths::Paths;
use window::Window;

mod texture;
mod paths;
mod shader;
mod shader_program;
mod shader_programs;
mod font;
mod engine;
mod menu;
mod modelview;
mod window;
mod rectangle;

fn main() {
    let paths = Paths::new();
    let mut window = Window::new(&paths);
    window.main_loop();
}
