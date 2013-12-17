extern mod gl;
extern mod sdl2;

use gl::types::{GLfloat, GLuint, GLint, GLsizeiptr};

fn compileShader(shader: GLuint) {
	unsafe {
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
	}
}

static vertexSource: &'static str =
"#version 150\n\
in vec2 position;\n\
void main() {\n\
   gl_Position = vec4(position, 0.0, 1.0);\n\
}";

static fragmentSource: &'static str =
"#version 150\n\
uniform vec3 triangleColor;\n\
out vec4 outColor;\n\
void main() {\n\
   outColor = vec4(triangleColor, 1.0);\n\
}";

fn main() {
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

	static vertices: [GLfloat, ..6] = [
		0.0, 0.5,
		0.5, -0.5,
		-0.5, -0.5
	];

	let mut uniColor: GLint;

	unsafe {
		let mut vao: GLuint = -1;
		gl::GenVertexArrays(1, &mut vao);
		gl::BindVertexArray(vao);

		let mut vbo: GLuint = 0;
		gl::GenBuffers(1, &mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
		               std::cast::transmute(&vertices[0]), gl::STATIC_DRAW);

		let vertexShader: GLuint = gl::CreateShader(gl::VERTEX_SHADER);
		assert!(vertexShader != 0);
		vertexSource.with_c_str(|src| {
			gl::ShaderSource(vertexShader, 1, &src, std::ptr::null());
		});
		compileShader(vertexShader);

		let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
		fragmentSource.with_c_str(|src| {
			gl::ShaderSource(fragmentShader, 1, &src, std::ptr::null());
		});
		compileShader(fragmentShader);

		let shaderProgram: GLuint = gl::CreateProgram();
		assert!(shaderProgram != 0);
		gl::AttachShader(shaderProgram, vertexShader);
		gl::AttachShader(shaderProgram, fragmentShader);
		gl::LinkProgram(shaderProgram);
		gl::UseProgram(shaderProgram);

		"position".with_c_str(|s| {
			let posAttrib: GLint = gl::GetAttribLocation(shaderProgram, s);
			assert!(posAttrib >= 0);
			gl::VertexAttribPointer(posAttrib as GLuint, 2, gl::FLOAT, gl::FALSE, 0,
			                        std::ptr::null());
			gl::EnableVertexAttribArray(posAttrib as GLuint);
		});

		uniColor = "triangleColor".with_c_str(|s| gl::GetUniformLocation(shaderProgram, s));
 	}

	'main : loop {
		loop {
			match sdl2::event::poll_event() {
				sdl2::event::QuitEvent(_) => break 'main,
				sdl2::event::KeyDownEvent(_, _, key, _, _) => {
					if key == sdl2::keycode::EscapeKey {
						break 'main
					}
				}
				sdl2::event::NoEvent => break,
				_ => {}
			}
		}
		gl::Uniform3f(uniColor, std::num::sin(sdl2::get_ticks() as f32 * 0.01f32), 1.0, 0.0);
		gl::DrawArrays(gl::TRIANGLES, 0, 3);
		window.gl_swap_window();
	}

	sdl2::quit();
}
