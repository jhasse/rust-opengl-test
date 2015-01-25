use paths::Paths;
use shader_program::ShaderProgram;
use nalgebra::Mat4;

pub struct ShaderPrograms {
    pub simple: ShaderProgram,
    pub texture: ShaderProgram
}

impl ShaderPrograms {
    pub fn new(paths: &Paths) -> ShaderPrograms {
        ShaderPrograms {
            simple: ShaderProgram::new(paths, "data/glsl/simple.vert", "data/glsl/simple.frag"),
            texture: ShaderProgram::new(paths, "data/glsl/texture.vert", "data/glsl/texture.frag")
        }
    }

    pub fn set_modelview_matrix(&self, matrix: &Mat4<f32>) {
        self.simple.set_modelview_matrix(matrix);
        self.texture.set_modelview_matrix(matrix);
    }

    pub fn set_projection_matrix(&self, matrix: &Mat4<f32>) {
        self.simple.set_projection_matrix(matrix);
        self.texture.set_projection_matrix(matrix);
    }
}
