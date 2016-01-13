use engine::game_object::GameObject;
use shader_programs::ShaderPrograms;
use font::text::Text;
use font::face::Face;

pub struct Menu {
    text: Text,
}

impl Menu {
    pub fn new(shader_programs: &ShaderPrograms, face: &mut Face) -> Menu {
        Menu { text: Text::new(&shader_programs, face, "Hallo Welt!") }
    }
}

impl GameObject for Menu {
    fn step(&mut self) {}

    fn draw(&self, shader_programs: &mut ShaderPrograms) {
        self.text.draw(shader_programs);
    }
}
