extern crate gl;

use shader::Shader;
use paths::Paths;
use gl::types::{GLuint, GLint};
use std::ffi::CString;

pub struct ShaderProgram {
    pub id: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}


impl ShaderProgram {
    pub fn new(paths: &Paths, vertex_path: &str, fragment_path: &str) -> ShaderProgram {
        unsafe {
            let id = gl::CreateProgram();
            assert!(id != 0);

            let vertex_shader = Shader::new(paths, vertex_path, gl::VERTEX_SHADER);
            let fragment_shader = Shader::new(paths, fragment_path, gl::FRAGMENT_SHADER);

            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);
            gl::LinkProgram(id);

            ShaderProgram{ id: id }
        }
    }
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    pub fn get_attrib_location(&self, name: &str) -> GLuint {
        let location = unsafe {
            gl::GetAttribLocation(self.id, CString::from_slice(name.as_bytes()).as_ptr())
        };
        assert!(location >= 0);
        location as GLuint
    }
    pub fn get_uniform_location(&self, name: &str) -> GLint {
        let location = unsafe {
            gl::GetUniformLocation(self.id, CString::from_slice(name.as_bytes()).as_ptr())
        };
        assert!(location != -1);
        location
    }
}
