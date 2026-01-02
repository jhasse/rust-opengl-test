use crate::paths::Paths;
use crate::shader_program::ShaderProgram;
use nalgebra::Matrix4;
use crate::modelview::Modelview;

pub struct ShaderPrograms {
    pub simple: ShaderProgram,
    pub texture: ShaderProgram,
    pub modelview: Modelview,
}

impl ShaderPrograms {
    pub fn new(paths: &Paths) -> ShaderPrograms {
        ShaderPrograms {
            simple: ShaderProgram::new(paths, "data/glsl/simple.vert", "data/glsl/simple.frag"),
            texture: ShaderProgram::new(paths, "data/glsl/texture.vert", "data/glsl/texture.frag"),
            modelview: Modelview::new(),
        }
    }

    pub fn set_modelview_matrix(&self, matrix: &Matrix4<f32>) {
        self.simple.set_modelview_matrix(matrix);
        self.texture.set_modelview_matrix(matrix);
    }

    pub fn set_projection_matrix(&self, matrix: &Matrix4<f32>) {
        self.simple.set_projection_matrix(matrix);
        self.texture.set_projection_matrix(matrix);
    }

    pub fn set_uniform(&self, modelview: &Modelview) {
        self.set_modelview_matrix(&modelview.matrix);
    }
}
