extern mod gl;

use gl::types::{GLuint, GLfloat, GLsizeiptr, GLsizei, GLint};
use std;

pub struct Texture {
    id: GLuint,
    vbo: GLuint
}

impl Texture {
    pub fn new(width: GLsizei, height: GLsizei) -> Texture {
        let mut texture: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as GLint, width, height, 0, gl::RGB,
                           gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            let vertexes: [GLfloat, ..16] = [
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, // texture coordinates
                -1.0, -1.0,
                -1.0, 1.0,
                1.0, 1.0,
                1.0, -1.0
            ];
            let mut vertexBuffer: GLuint = 0;
            gl::GenBuffers(1, &mut vertexBuffer);
            assert!(vertexBuffer != 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertexBuffer);

            let mut vbo: GLuint = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertexes.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                           std::cast::transmute(&vertexes[0]), gl::STATIC_DRAW);

            Texture{ id: texture, vbo: vbo }
        }
    }
}
