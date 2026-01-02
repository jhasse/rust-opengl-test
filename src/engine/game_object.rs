use crate::shader_programs::ShaderPrograms;

pub trait GameObject {
    fn step(&mut self);
    fn draw(&self, _: &mut ShaderPrograms);
}
