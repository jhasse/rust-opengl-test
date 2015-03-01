#![feature(path, io, std_misc, libc, env)]

extern crate gl;
extern crate glfw;
#[macro_use] extern crate log;
extern crate freetype;
extern crate nalgebra;

use paths::Paths;
use window::Window;

mod texture;
mod paths;
mod shader;
mod shader_program;
mod shader_programs;
mod font;
mod modelview;
mod window;
mod rectangle;

fn main() {
	let paths = Paths::new();
    let mut window = Window::new(&paths);
    window.main_loop(&paths);
}
