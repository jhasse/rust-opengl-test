use shader_programs::ShaderPrograms;

pub trait GameObject {
    fn step(&mut self);
    fn draw(&self, &mut ShaderPrograms);
}
