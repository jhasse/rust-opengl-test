use paths::Paths;
use shader_program::ShaderProgram;

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
}
