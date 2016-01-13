use font::character::Character;
use font::face::Face;
use shader_programs::ShaderPrograms;
use engine::game_object::GameObject;

pub struct Text {
    characters: Vec<Character>,
}

impl Text {
    pub fn new(shader_programs: &ShaderPrograms, face: &mut Face, text: &str) -> Text {
        let mut characters: Vec<Character> = vec![];
        for ch in text.chars() {
            characters.push(Character::new(shader_programs, &mut face.ft_face, ch));
        }
        Text { characters: characters }
    }
}

impl GameObject for Text {
    fn step(&mut self) {}

    fn draw(&self, shader_programs: &mut ShaderPrograms) {
        for ch in self.characters.iter() {
            ch.draw(shader_programs);
        }
    }
}
