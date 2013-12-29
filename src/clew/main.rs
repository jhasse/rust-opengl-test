extern mod gl;
extern mod sdl2;

use paths::Paths;
use texture::Texture;
use gl::types::{GLfloat, GLuint, GLint, GLsizeiptr, GLenum};
use std::io::File;

mod texture;
mod paths;

fn compileShader(paths: &Paths, filename: &str, shaderType: GLenum) -> GLuint {
	let mut reader = File::open(&paths.prefix.join(Path::new(filename))).unwrap();
	let src = std::str::from_utf8_owned(reader.read_to_end());
	unsafe {
		let shader = gl::CreateShader(shaderType);
		assert!(shader != 0);
		src.with_c_str(|src| {
			gl::ShaderSource(shader, 1, &src, std::ptr::null());
		});
		gl::CompileShader(shader);
		let mut status: GLint = gl::FALSE as GLint;
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
		if status != gl::TRUE as GLint {
			let mut buffer = [0i8, ..512];
			let mut length: i32 = 0;
			gl::GetShaderInfoLog(shader, buffer.len() as i32, &mut length,
			                     &mut buffer[0]);
			println(format!("Compiler log (length: {}):\n{}", length,
			                std::str::raw::from_c_str(&buffer[0])));
		}
		shader
	}
}

struct Triangle {
    vao: GLuint,
    vbo: GLuint,
    program: GLuint,
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

        let vertexShader = compileShader(paths, "data/glsl/simple.vert", gl::VERTEX_SHADER);
        let fragmentShader = compileShader(paths, "data/glsl/simple.frag", gl::FRAGMENT_SHADER);

        let shaderProgram: GLuint = gl::CreateProgram();
        assert!(shaderProgram != 0);
        gl::AttachShader(shaderProgram, vertexShader);
        gl::AttachShader(shaderProgram, fragmentShader);
        gl::LinkProgram(shaderProgram);
        gl::UseProgram(shaderProgram);

        "position".with_c_str(|s| {
            let posAttrib = gl::GetAttribLocation(shaderProgram, s);
            assert!(posAttrib >= 0);
            gl::VertexAttribPointer(posAttrib as GLuint, 2, gl::FLOAT, gl::FALSE, 0,
                                    std::ptr::null());
            gl::EnableVertexAttribArray(posAttrib as GLuint);
        });

        let uniColor = "triangleColor".with_c_str(|s| gl::GetUniformLocation(shaderProgram, s));
        assert!(uniColor != -1);
        gl::Uniform3f(uniColor, 1.0, 1.0, 0.0);

        let uniPos = "pos".with_c_str(|s| gl::GetUniformLocation(shaderProgram, s));
        assert!(uniPos != -1);
        Triangle{vao: vao, vbo: vbo, program: shaderProgram, pos: uniPos }
    }
}

fn main() {
	let paths = Paths::new();

	sdl2::init([sdl2::InitVideo]);
	let width = 800;
	let height = 600;

	let window = match sdl2::video::Window::new("clew", sdl2::video::PosUndefined,
	                                            sdl2::video::PosUndefined, width, height,
	                                            [sdl2::video::OpenGL]) {
		Ok(window) => window,
		Err(err) => fail!(format!("failed to create window: {}", err))
	};

	let context = match window.gl_create_context() {
		Ok(context) => context,
		Err(err) => fail!(format!("failed to create context: {}", err))
	};
	window.gl_make_current(context);

	gl::load_with(sdl2::video::gl_get_proc_address);

    let mut vao: GLuint = 0;
    let mut vbo: GLuint;
    let mut buffer: GLuint = 0;
    let mut fbo: GLuint = 0;
    let mut texture: Texture;
    let shaderProgram: GLuint = gl::CreateProgram();
    assert!(shaderProgram != 0);

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
        assert!(texture.id != 0);
        vbo = texture.vbo;
        assert!(vbo != 0);

        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture.id,
                                 0);

        assert!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE);

        gl::ClearColor(1.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
//        drawTriangle(&paths);

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, texture.vbo);

        let vertexShader = compileShader(&paths, "data/glsl/texture.vert", gl::VERTEX_SHADER);
        let fragmentShader = compileShader(&paths, "data/glsl/texture.frag", gl::FRAGMENT_SHADER);

        gl::AttachShader(shaderProgram, vertexShader);
        gl::AttachShader(shaderProgram, fragmentShader);
        gl::LinkProgram(shaderProgram);
        gl::UseProgram(shaderProgram);

        "position".with_c_str(|s| {
            let posAttrib = gl::GetAttribLocation(shaderProgram, s);
            assert!(posAttrib >= 0);
            gl::VertexAttribPointer(posAttrib as GLuint, 2, gl::FLOAT, gl::FALSE, 0,
                                    std::cast::transmute(8 * std::mem::size_of::<GLfloat>()));
            gl::EnableVertexAttribArray(posAttrib as GLuint);
        });

        "texcoord".with_c_str(|s| {
            let posAttrib = gl::GetAttribLocation(shaderProgram, s);
            assert!(posAttrib >= 0);
            gl::VertexAttribPointer(posAttrib as GLuint, 2, gl::FLOAT, gl::FALSE, 0,
                                    std::ptr::null());
            gl::EnableVertexAttribArray(posAttrib as GLuint);
        });
    }

    let triangle = drawTriangle(&paths);
    let mut mouseX = 0.0;
    let mut mouseY = 0.0;

    let mut last_ticks = sdl2::get_ticks();
    let mut frames = 0;
    let mut counter = 0;

	'main : loop {
		loop {
			match sdl2::event::poll_event() {
				sdl2::event::QuitEvent(_) => break 'main,
				sdl2::event::KeyDownEvent(_, _, key, _, _) => {
					if key == sdl2::keycode::EscapeKey {
						break 'main
					}
				},
                sdl2::event::MouseMotionEvent(_, _, _, _, x, y, _, _) => {
                    mouseX = x as GLfloat / width as GLfloat - 0.5;
                    mouseY = -y as GLfloat / height as GLfloat + 0.5;
                },
				sdl2::event::NoEvent => break,
				_ => {}
			}
		}

        let old = last_ticks;
        last_ticks = sdl2::get_ticks();
        counter += last_ticks - old;
        frames += 1;
        if counter >= 1000 {
            counter -= 1000;
            window.set_title(format!("clew - FPS: {}", frames));
            frames = 0;
        }
        loop {
            let dif = sdl2::get_ticks() - last_ticks;
            if dif >= 8 {
                break;
            }
            sdl2::timer::delay(8 - dif);
        }

        gl::ClearColor(0.5, 0.5, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::BindRenderbuffer(gl::RENDERBUFFER, buffer);
        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

        gl::BindVertexArray(triangle.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, triangle.vbo);
        gl::UseProgram(triangle.program);
        gl::Uniform2f(triangle.pos, mouseX, mouseY);

        gl::ClearColor(mouseX, mouseY, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);

        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        // draw framebuffer
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::BindVertexArray(vao);
        gl::UseProgram(shaderProgram);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture.id);

        gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);

		window.gl_swap_window();
	}

	sdl2::quit();
}
