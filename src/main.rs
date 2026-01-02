extern crate gl;
extern crate glutin;
#[macro_use] extern crate log;
extern crate freetype;
extern crate nalgebra;
extern crate time;

use paths::Paths;
use window::Window;
use std::time::Instant;

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
    use glutin::event::{Event, WindowEvent};
    use glutin::event_loop::ControlFlow;

    let paths = Paths::new();
    let events_loop = glutin::event_loop::EventLoop::new();
    let mut window = Window::new(&paths, &events_loop);

    let mut last_time = Instant::now();
    let mut frames = 0.0;
    let mut counter = 0.0;

    events_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(size) => {
                        window.handle_resize(size);
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                window.render_frame(&mut last_time, &mut frames, &mut counter);
            }
            _ => {}
        }
    });
}
