extern crate gl;

use shader::Shader;
use paths::Paths;
use gl::types::{GLuint, GLint};
use std::ffi::CString;
use nalgebra::Mat4;
use std::mem;

pub struct ShaderProgram {
    pub id: GLuint,
    projection_uniform: GLint,
    modelview_uniform: GLint,
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

            let mut this = ShaderProgram{ id: id, projection_uniform: -1, modelview_uniform: -1 };

            this.use_program();
            this.projection_uniform = this.get_uniform_location("projection");
            this.modelview_uniform = this.get_uniform_location("modelview");
            this
        }
    }
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
    pub fn get_attrib_location(&self, name: &str) -> GLuint {
        let location = unsafe {
            gl::GetAttribLocation(self.id, CString::new(name.as_bytes()).unwrap().as_ptr())
        };
        assert!(location >= 0);
        location as GLuint
    }
    pub fn get_uniform_location(&self, name: &str) -> GLint {
        let location = unsafe {
            gl::GetUniformLocation(self.id, CString::new(name.as_bytes()).unwrap().as_ptr())
        };
        assert!(location != -1);
        location
    }
    pub fn set_modelview_matrix(&self, matrix: &Mat4<f32>) {
        unsafe {
            gl::UseProgram(self.id);
            gl::UniformMatrix4fv(self.modelview_uniform, 1, 0,
                                 mem::transmute(matrix.as_ref()));
        }
    }
    pub fn set_projection_matrix(&self, matrix: &Mat4<f32>) {
        unsafe {
            gl::UseProgram(self.id);
            gl::UniformMatrix4fv(self.projection_uniform, 1, 0,
                                 mem::transmute(matrix.as_ref()));
        }
    }
}
