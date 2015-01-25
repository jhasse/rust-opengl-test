use nalgebra::*;
use shader_programs::ShaderPrograms;

#[derive(Clone)]
pub struct Modelview<'a> {
    matrix: Mat4<f32>,
    shader_programs: &'a ShaderPrograms,
}

impl<'a> Modelview<'a> {
    pub fn new(shader_programs: &ShaderPrograms) -> Modelview {
        Modelview{ matrix: new_identity(4), shader_programs: shader_programs }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.matrix = self.matrix * Mat4::new(1f32, 0f32, 0f32, x,
                                              0f32, 1f32, 0f32, y,
                                              0f32, 0f32, 1f32, 0f32,
                                              0f32, 0f32, 0f32, 1f32);
    }

    pub fn reset(&mut self) {
        self.matrix = new_identity(4);
    }

    pub fn set_uniform(&self) {
        self.shader_programs.set_modelview_matrix(&self.matrix);
    }
}
