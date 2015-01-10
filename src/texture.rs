extern crate gl;
extern crate log;

use gl::types::{GLuint, GLfloat, GLsizeiptr, GLsizei, GLint};
use std;
use std::mem;

pub struct Texture {
    pub id: GLuint,
    pub vbo: GLuint
}

impl Texture {
    pub fn new(width: GLsizei, height: GLsizei) -> Texture {
        let mut texture: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            assert!(texture != 0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, width, height, 0, gl::RGB,
                           gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }
        let vertexes: [GLfloat; 16] = [
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, // texture coordinates
            -1.0, -1.0,
            -1.0, 1.0,
            1.0, 1.0,
            1.0, -1.0
        ];

        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            assert!(vbo != 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertexes.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                           mem::transmute(&vertexes[0]), gl::STATIC_DRAW);
        }

        Texture{ id: texture, vbo: vbo }
    }

    pub fn from_data(width: GLsizei, height: GLsizei, data: &[u8]) -> Texture {
        let texture = Texture::new(width, height);
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, width, height, gl::RGBA, gl::UNSIGNED_BYTE,
                              mem::transmute(&data[0]));
        }
        texture
    }

    pub fn draw(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }
}
