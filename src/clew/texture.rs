extern mod gl;

use gl::types::{GLuint, GLfloat, GLsizeiptr};
use std;

fn nextPowerOf2(n: i32) -> i32 {
    if n == 1 {
        return 2;
    }
    let mut rval = 1;
    while rval < n {
        rval <<= 1;
    }
    rval
}

pub struct Texture {
    id: GLuint,
    vbo: GLuint
}

impl Texture {
    pub fn new(imgWidth: i32, imgHeight: i32) -> Texture {
        // texture
        let width = nextPowerOf2(imgWidth as i32);
        let height = nextPowerOf2(imgHeight as i32);
        let mut texture: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32,
                           height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            let x: GLfloat = imgWidth as GLfloat / width as GLfloat;
            let y: GLfloat = imgHeight as GLfloat / height as GLfloat;
            let vertexes: [GLfloat, ..16] = [
                0.0f32, 0.0f32, 0.0f32, y, x, y, x, 0.0f32, // texture coordinates
                0.0f32, 0.0f32,
                0.0f32, imgHeight as GLfloat,
                imgWidth as GLfloat, imgHeight as GLfloat,
                imgWidth as GLfloat, 0.0f32
            ];
            let mut vertexBuffer: GLuint = 0;
            gl::GenBuffers(1, &mut vertexBuffer);
            assert!(vertexBuffer != 0);
    //        gl::BindBuffer(gl::ARRAY_BUFFER, vertexBuffer);
            gl::BindTexture(gl::TEXTURE_2D, 0);

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
