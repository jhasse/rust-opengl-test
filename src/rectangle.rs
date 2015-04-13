use gl::types::{GLuint, GLfloat, GLsizeiptr};
use gl;
use std;
use shader_programs::ShaderPrograms;

pub struct Rectangle {
    vao: GLuint
}

static VERTICES: [GLfloat; 8] = [
    0.0, 1.0,
    1.0, 1.0,
    1.0, 0.0,
    0.0, 0.0,
];

impl Rectangle {
    pub fn new(shader_programs: &ShaderPrograms) -> Rectangle {
        let mut vao: GLuint = 0;
        unsafe {
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
        }

        Rectangle {
            vao: vao
        }
    }

    pub fn draw(&self, shader_programs: &mut ShaderPrograms) {
        shader_programs.simple.use_program();
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }
}
