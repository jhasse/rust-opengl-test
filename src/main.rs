 #![allow(unstable)] // FIXME: Should be removed when Rust 1.0 is released

extern crate gl;
extern crate glfw;
#[macro_use] extern crate log;
extern crate freetype;
extern crate nalgebra;

use paths::Paths;
use shader_programs::ShaderPrograms;
use shader_program::ShaderProgram;
use gl::types::{GLfloat, GLuint, GLsizeiptr, GLint};
use std::cell::Cell;
use std::mem;
use glfw::Context;

mod texture;
mod paths;
mod shader;
mod shader_program;
mod shader_programs;
mod font;
mod modelview;

struct Triangle {
    vao: GLuint,
    vbo: GLuint,
}

fn create_triangle(shader_programs: &ShaderPrograms) -> Triangle {
    static VERTICES: [GLfloat; 6] = [
        0.0, 0.2,
        0.5, -0.5,
        -0.5, -0.5
    ];

    unsafe {
        let mut vao: GLuint = -1;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo: GLuint = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTICES.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTICES[0]), gl::STATIC_DRAW);

        shader_programs.simple.use_program();

        let pos_attrib = shader_programs.simple.get_attrib_location("position");
        gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(pos_attrib);

        let uni_color = shader_programs.simple.get_uniform_location("triangleColor");
        gl::Uniform3f(uni_color, 1.0, 1.0, 0.0);

        Triangle{vao: vao, vbo: vbo }
    }
}

fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
    error!("GLFW error {}: {}", error_count.get(), description);
    error_count.set(error_count.get() + 1);
}

fn main() {
	let paths = Paths::new();

    let mut modelview = modelview::Modelview::new();

	let width = 800;
	let height = 600;

    let mut glfw = glfw::init(Some(
        glfw::Callback {
            f: error_callback as fn(glfw::Error, String, &Cell<usize>),
            data: Cell::new(0),
        }
    )).unwrap();

    let (mut window, _) = glfw.create_window(width, height, "rust-opengl-test",
                                             glfw::WindowMode::Windowed)
        .expect("Failed to create window.");

    window.make_current();

    gl::load_with(|s| window.get_proc_address(s));

    let shader_programs = ShaderPrograms::new(&paths);
    let freetype = freetype::Library::init().unwrap();
    let mut face = font::face::Face::new(freetype, &paths, "Lato-Lig.otf", 16);
    let text = font::text::Text::new(&shader_programs, &mut face, "Hallo Welt!");

    let mut vao: GLuint = 0;
    let mut buffer: GLuint = 0;
    let mut fbo: GLuint = 0;
    let mut texture: GLuint = 0;
    let shader_program = ShaderProgram::new(&paths, "data/glsl/texture.vert",
                                            "data/glsl/texture.frag");

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // renderbuffer
        gl::GenRenderbuffers(1, &mut buffer);
        gl::BindRenderbuffer(gl::RENDERBUFFER, buffer);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::RGBA8, width as GLint, height as GLint);

        // framebuffer
        gl::GenFramebuffers(1, &mut fbo);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER,
                                    buffer);


        gl::GenTextures(1, &mut texture);
        assert!(texture != 0);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, width as GLint, height as GLint, 0,
                       gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        let vertexes: [GLfloat; 16] = [
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, // texture coordinates
            -1.0, -1.0,
            -1.0, 1.0,
            1.0, 1.0,
            1.0, -1.0
        ];

        let mut vbo: GLuint = 0;
        gl::GenBuffers(1, &mut vbo);
        assert!(vbo != 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertexes.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&vertexes[0]), gl::STATIC_DRAW);

        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D,
                                 texture, 0);

        assert!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE);

        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::BindVertexArray(vao);
        shader_program.use_program();

        let pos_attrib = shader_program.get_attrib_location("position");
        gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, 0,
                                mem::transmute(8 * std::mem::size_of::<GLfloat>()));
        gl::EnableVertexAttribArray(pos_attrib);

        let pos_attrib = shader_program.get_attrib_location("texcoord");
        gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, 0,
                                std::ptr::null());
        gl::EnableVertexAttribArray(pos_attrib);
        gl::BindVertexArray(0);

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    let triangle = create_triangle(&shader_programs);

    let mut last_time = glfw.get_time();
    let mut frames = 0.0;
    let mut counter = 0.0;

    let mut timer = std::io::timer::Timer::new();
    let joystick = glfw::Joystick{ id: glfw::JoystickId::Joystick1, glfw: glfw };

    shader_programs.simple.use_program();
    let modelview_uniform = shader_programs.simple.get_uniform_location("modelview");

    let projection_uniform = shader_programs.simple.get_uniform_location("projection");
    let projection_matrix: nalgebra::Mat4<f32> = nalgebra::new_identity(4);
    unsafe {
        gl::UniformMatrix4fv(projection_uniform, 1, 0,
                             mem::transmute(projection_matrix.as_array()));
    }

    while !window.should_close() {
        glfw.poll_events();

        let old = last_time;
        last_time = glfw.get_time();
        counter += last_time - old;
        frames += 1.0;
        if counter >= 1.0 {
            frames *= counter;
            counter -= 1.0;
            window.set_title(format!("clew - FPS: {}", frames as usize).as_slice());
            frames = 0.0;
        }
        loop {
            let dif = glfw.get_time() - last_time;
            if dif >= 0.008 {
                break;
            }
            match timer {
                Ok(ref mut t) =>
                    t.sleep(std::time::Duration::milliseconds(((0.008 - dif) * 1000.0) as i64)),
                Err(_) => ()
            }
        }

        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindRenderbuffer(gl::RENDERBUFFER, buffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::BindVertexArray(triangle.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, triangle.vbo);
        }

        modelview.reset();
        shader_programs.simple.use_program();

        if joystick.is_present() {
            unsafe {
                modelview.translate(joystick.get_axes()[0].clone(),
                                    joystick.get_axes()[1].clone());
                gl::ClearColor(joystick.get_axes()[2].clone(),
                               joystick.get_axes()[3].clone(),
                               0.5, 1.0);
            }
        } else {
            unsafe {
                gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            }
        }

        unsafe {
            gl::UniformMatrix4fv(modelview_uniform, 1, 0,
                                 mem::transmute(modelview.matrix.as_array()));
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        shader_program.use_program();
        text.draw();

        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::BindVertexArray(vao);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }

        window.swap_buffers();
    }
}
