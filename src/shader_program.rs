extern crate gl;

use shader::Shader;
use gl::types::{GLuint, GLint};

pub struct ShaderProgram {
    pub id: GLuint,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        unsafe {
            let id = gl::CreateProgram();
            assert!(id != 0);
            ShaderProgram{ id: id }
        }
    }
    pub fn attach(&self, shader: Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.id);
        }
    }
    pub fn link(&self) {
        unsafe {
            gl::LinkProgram(self.id);
        }
    }
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    pub fn get_attrib_location(&self, name: &str) -> GLuint {
        name.with_c_str(|s| {
            let location = unsafe { gl::GetAttribLocation(self.id, s) };
            assert!(location >= 0);
            location as GLuint
        })
    }
    pub fn get_uniform_location(&self, name: &str) -> GLint {
        name.with_c_str(|s| {
            let location = unsafe { gl::GetUniformLocation(self.id, s) };
            assert!(location != -1);
            location
        })
    }
}
