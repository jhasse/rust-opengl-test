use glutin;
use glutin::Event::Resized;
use gl::types::{GLint, GLuint, GLfloat, GLsizeiptr};
use paths::Paths;
use shader::Shader;
use rectangle::Rectangle;
use gl;
use std;
use std::ffi::CString;
use nalgebra;
use shader_programs::ShaderPrograms;
use freetype;
use font::face::Face;
use engine::game_object::GameObject;
use menu::Menu;

pub struct Window {
    glutin_window: glutin::Window,
    shader_programs: ShaderPrograms,
    buffer: GLuint,
    fbo: GLuint,
    vao: GLuint,
    texture: GLuint,
    shader_program: GLuint,
    width: u32,
    height: u32,
    work: Box<GameObject>,
}

impl Window {
    pub fn new(paths: &Paths) -> Window {
        let window = glutin::Window::new().unwrap();
        window.set_title("rust-opengl-test");
        let (width, height) = window.get_inner_size().unwrap();
        unsafe {
            window.make_current();
        }

        gl::load_with(|s| window.get_proc_address(s));

        let shader_programs = ShaderPrograms::new(&paths);

        let mut vao: GLuint = 0;
        let mut buffer: GLuint = 0;
        let mut fbo: GLuint = 0;
        let vertex_shader = Shader::new(&paths, "data/glsl/window.vert", gl::VERTEX_SHADER);
        let fragment_shader = Shader::new(&paths, "data/glsl/texture.frag", gl::FRAGMENT_SHADER);
        let mut shader_program;
        unsafe {
            shader_program = gl::CreateProgram();
            assert!(shader_program != 0);
            gl::AttachShader(shader_program, vertex_shader.id);
            gl::AttachShader(shader_program, fragment_shader.id);
            gl::LinkProgram(shader_program);
        }

        let freetype = freetype::Library::init().unwrap();
        let mut face = Face::new(&freetype, &paths, "Lato-Lig.otf", 48);
        let work = Box::new(Menu::new(&shader_programs, &mut face));

        let mut this = Window {
            glutin_window: window,
            shader_programs: shader_programs, buffer: buffer, fbo: fbo,
            vao: vao, texture: 0, shader_program: shader_program, width: width, height: height,
            work: work,
        };

        this.resize();

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
                           std::mem::transmute(&vertexes[0]), gl::STATIC_DRAW);

            assert!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE);

            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            gl::BindVertexArray(vao);
            gl::UseProgram(shader_program);


            let pos_attrib = gl::GetAttribLocation(shader_program,
                                                   CString::new("position").unwrap().as_ptr());
            assert!(pos_attrib >= 0);
            gl::VertexAttribPointer(pos_attrib as GLuint, 2, gl::FLOAT, gl::FALSE, 0,
                                    std::mem::transmute(8 * std::mem::size_of::<GLfloat>()));
            gl::EnableVertexAttribArray(pos_attrib as GLuint);

            let tex_attrib = gl::GetAttribLocation(shader_program,
                                                   CString::new("texcoord").unwrap().as_ptr());
            assert!(tex_attrib >= 0);
            gl::VertexAttribPointer(tex_attrib as GLuint, 2, gl::FLOAT, gl::FALSE, 0,
                                    std::ptr::null());
            gl::EnableVertexAttribArray(tex_attrib as GLuint);

            gl::BindVertexArray(0);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        this
    }

    fn resize(&mut self) {
        unsafe {
            if self.texture != 0 {
                gl::DeleteTextures(1, &mut self.texture)
            }

            gl::GenTextures(1, &mut self.texture);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, self.width as i32, self.height as i32,
                           0, gl::RGB, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D,
                                     self.texture, 0);

            assert!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE);

            gl::Viewport(0, 0, self.width as i32, self.height as i32);

            let mut projection: nalgebra::Mat4<f32> = nalgebra::new_identity(4);
            projection[(0,0)] = self.height as f32 / self.width as f32;
            self.shader_programs.set_projection_matrix(&projection);
        }
    }

    pub fn main_loop(&mut self) {
        let triangle = create_triangle(&self.shader_programs);
        let rect = Rectangle::new(&self.shader_programs);

/*        let mut last_time = self.glutin.get_time();
        let mut frames = 0.0;
        let mut counter = 0.0;

        let mut timer = std::old_io::timer::Timer::new();*/
        //let joystick = glfw::Joystick{ id: glfw::JoystickId::Joystick1, glfw: self.glfw };

        while !self.glutin_window.is_closed() {
            let mut should_resize = false;
            for event in self.glutin_window.poll_events() {
                match event {
                    Resized(width, height) => {
                        self.width = width as u32;
                        self.height = height as u32;
                        should_resize = true;
                    }
                    _ => {},
                }
            }
            if should_resize {
                self.resize();
            }

/*            let old = last_time;
            last_time = self.glutin.get_time();
            counter += last_time - old;
            frames += 1.0;
            if counter >= 1.0 {
                frames *= counter;
                counter -= 1.0;
                self.glutin_window.set_title(&*format!("clew - FPS: {}", frames as usize));
                frames = 0.0;
            }
            loop {
                let dif = self.glutin.get_time() - last_time;
                if dif >= 0.008 {
                    break;
                }
                match timer {
                    Ok(ref mut t) =>
                        t.sleep(std::time::Duration::milliseconds(((0.008 - dif) * 1000.0) as i64)),
                    Err(_) => ()
                }
            }*/

            unsafe {
                gl::BindRenderbuffer(gl::RENDERBUFFER, self.buffer);
                gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
                gl::ClearColor(0.5, 0.5, 0.5, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);

                gl::BindBuffer(gl::ARRAY_BUFFER, 123);
                gl::BindVertexArray(triangle.vao);
            }

            self.shader_programs.modelview.reset();
            self.shader_programs.simple.use_program();

            /*if joystick.is_present() {
                unsafe {
                    self.shader_programs.modelview.translate(joystick.get_axes()[0].clone(),
                                             joystick.get_axes()[1].clone());
                    gl::ClearColor(joystick.get_axes()[2].clone(),
                                   joystick.get_axes()[3].clone(),
                                   0.5, 1.0);
                }
            } else*/ {
                unsafe {
                    gl::ClearColor(0.5, 0.5, 0.5, 1.0);
                }
            }
            self.shader_programs.set_uniform(&self.shader_programs.modelview);
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }

            self.shader_programs.texture.use_program();
            self.shader_programs.modelview.translate(-0.5, 0.0);
            rect.draw(&mut self.shader_programs);
            self.work.draw(&mut self.shader_programs);

            unsafe {
                gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                gl::UseProgram(self.shader_program);
                gl::BindVertexArray(self.vao);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.texture);
                gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            }

            self.glutin_window.swap_buffers();
        }
    }
}


struct Triangle {
    vao: GLuint,
}

fn create_triangle(shader_programs: &ShaderPrograms) -> Triangle {
    static VERTICES: [GLfloat; 6] = [
        0.0, 0.2,
        0.5, -0.5,
        -0.5, -0.5
    ];

    unsafe {
        let mut vao: GLuint = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        let mut vbo: GLuint = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTICES.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                       std::mem::transmute(&VERTICES[0]), gl::STATIC_DRAW);

        shader_programs.simple.use_program();

        let pos_attrib = shader_programs.simple.get_attrib_location("position");
        gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(pos_attrib);

        let uni_color = shader_programs.simple.get_uniform_location("triangleColor");
        gl::Uniform3f(uni_color, 1.0, 1.0, 0.0);

        Triangle{vao: vao}
    }
}
