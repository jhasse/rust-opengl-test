extern crate gl;
extern crate log;

use gl::types::{GLuint, GLfloat, GLsizeiptr, GLsizei, GLint};
use std;
use std::mem;
use shader_programs::ShaderPrograms;

pub struct Texture {
    pub id: GLuint,
    pub vao: GLuint,
}

impl Texture {
    pub fn new(shader_programs: &ShaderPrograms,
               width: GLsizei,
               height: GLsizei,
               data: &[u8])
               -> Texture {
        let mut texture: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            assert!(texture != 0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as GLint,
                           width,
                           height,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           mem::transmute(&data[0]));
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }
        let vertexes: [GLfloat; 16] = [0.0,
                                       1.0,
                                       0.0,
                                       0.0,
                                       1.0,
                                       0.0,
                                       1.0,
                                       1.0, // texture coordinates
                                       0.0,
                                       -height as GLfloat * 0.001,
                                       0.0,
                                       0.0,
                                       width as GLfloat * 0.001,
                                       0.0,
                                       width as GLfloat * 0.001,
                                       -height as GLfloat * 0.001];

        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            assert!(vao != 0);
            gl::BindVertexArray(vao);

            let mut vbo: GLuint = 0;
            gl::GenBuffers(1, &mut vbo);
            assert!(vbo != 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertexes.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&vertexes[0]),
                           gl::STATIC_DRAW);

            shader_programs.texture.use_program();

            let pos_attrib = shader_programs.texture.get_attrib_location("position");
            gl::VertexAttribPointer(pos_attrib,
                                    2,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    0,
                                    mem::transmute(8 * std::mem::size_of::<GLfloat>()));
            gl::EnableVertexAttribArray(pos_attrib);

            let pos_attrib = shader_programs.texture.get_attrib_location("texcoord");
            gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::EnableVertexAttribArray(pos_attrib);
            gl::BindVertexArray(0);
        }

        Texture {
            id: texture,
            vao: vao,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }
}
