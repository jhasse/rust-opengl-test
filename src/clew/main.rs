extern mod gl;
extern mod glfw;

use paths::Paths;
use texture::Texture;
use shader::Shader;
use shader_program::ShaderProgram;
use gl::types::{GLfloat, GLuint, GLint, GLsizeiptr};

mod texture;
mod paths;
mod shader;
mod shader_program;

struct Triangle {
    vao: GLuint,
    vbo: GLuint,
    program: ShaderProgram,
    pos: GLint
}

fn drawTriangle(paths: &Paths) -> Triangle {
    static vertices: [GLfloat, ..6] = [
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
                       (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                       std::cast::transmute(&vertices[0]), gl::STATIC_DRAW);

        let vertexShader = Shader::new(paths, "data/glsl/simple.vert", gl::VERTEX_SHADER);
        let fragmentShader = Shader::new(paths, "data/glsl/simple.frag", gl::FRAGMENT_SHADER);

        let shaderProgram = ShaderProgram::new();
        shaderProgram.attach(vertexShader);
        shaderProgram.attach(fragmentShader);
        shaderProgram.link();
        shaderProgram.use_program();

        let posAttrib = shaderProgram.get_attrib_location("position");
        gl::VertexAttribPointer(posAttrib, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(posAttrib);

        let uniColor = shaderProgram.get_uniform_location("triangleColor");
        assert!(uniColor != -1);
        gl::Uniform3f(uniColor, 1.0, 1.0, 0.0);

        let uniPos = shaderProgram.get_uniform_location("pos");
        assert!(uniPos != -1);
        Triangle{vao: vao, vbo: vbo, program: shaderProgram, pos: uniPos }
    }
}

struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}

fn main() {
	let paths = Paths::new();
	let width = 800;
	let height = 600;

    glfw::set_error_callback(~ErrorContext);

    do glfw::start {
        let window = glfw::Window::create(width, height, "clew", glfw::Windowed)
            .expect("Failed to create window.");

        window.make_context_current();

        gl::load_with(glfw::get_proc_address);

        let mut vao: GLuint = 0;
        let mut buffer: GLuint = 0;
        let mut fbo: GLuint = 0;
        let mut texture: Texture;
        let shaderProgram = ShaderProgram::new();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // renderbuffer
            gl::GenRenderbuffers(1, &mut buffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, buffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::RGBA8, width as i32, height as i32);

            // framebuffer
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER,
                                        buffer);

            texture = Texture::new(width as i32, height as i32);

            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D,
                                     texture.id, 0);

            assert!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE);

            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
    //        drawTriangle(&paths);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, texture.vbo);

            let vertexShader = Shader::new(&paths, "data/glsl/texture.vert", gl::VERTEX_SHADER);
            let fragmentShader = Shader::new(&paths, "data/glsl/texture.frag", gl::FRAGMENT_SHADER);

            shaderProgram.attach(vertexShader);
            shaderProgram.attach(fragmentShader);
            shaderProgram.link();
            shaderProgram.use_program();

            let posAttrib = shaderProgram.get_attrib_location("position");
            gl::VertexAttribPointer(posAttrib, 2, gl::FLOAT, gl::FALSE, 0,
                                    std::cast::transmute(8 * std::mem::size_of::<GLfloat>()));
            gl::EnableVertexAttribArray(posAttrib);

            let posAttrib = shaderProgram.get_attrib_location("texcoord");
            gl::VertexAttribPointer(posAttrib, 2, gl::FLOAT, gl::FALSE, 0,
                                    std::ptr::null());
            gl::EnableVertexAttribArray(posAttrib);
        }

        let triangle = drawTriangle(&paths);

        let mut last_time = glfw::get_time();
        let mut frames = 0.0;
        let mut counter = 0.0;

        let mut timer = std::io::timer::Timer::new();
        let joystick = glfw::Joystick1;

        while !window.should_close() {
            glfw::poll_events();

            let old = last_time;
            last_time = glfw::get_time();
            counter += last_time - old;
            frames += 1.0;
            if counter >= 1.0 {
                frames *= counter;
                counter -= 1.0;
                window.set_title(format!("clew - FPS: {}", frames as int));
                frames = 0.0;
            }
            loop {
                let dif = glfw::get_time() - last_time;
                if dif >= 0.008 {
                    break;
                }
                match timer {
                    Some(ref mut t) => t.sleep(((0.008 - dif) * 1000.0) as u64),
                    None => ()
                }
            }

            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindRenderbuffer(gl::RENDERBUFFER, buffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::BindVertexArray(triangle.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, triangle.vbo);
            triangle.program.use_program();

            if joystick.is_present() {
                gl::Uniform2f(triangle.pos, joystick.get_axes()[0], joystick.get_axes()[1]);
                gl::ClearColor(joystick.get_axes()[2], joystick.get_axes()[3], 0.5, 1.0);
            } else {
                gl::Uniform2f(triangle.pos, 0.5, 0.5);
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            }

            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            // draw framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::BindVertexArray(vao);
            shaderProgram.use_program();

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);

            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);

            window.swap_buffers();
        }
    }
}
