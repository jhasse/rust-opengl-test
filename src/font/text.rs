use font::character::Character;
use font::face::Face;
use shader_programs::ShaderPrograms;
use modelview::Modelview;

pub struct Text {
    characters: Vec<Character>
}

impl Text {
    pub fn new(shader_programs: &ShaderPrograms, face: &mut Face, text: &str) -> Text {
        let mut characters: Vec<Character> = vec![];
        for ch in text.chars() {
            characters.push(Character::new(shader_programs, &mut face.ft_face, ch));
        }
        Text{ characters: characters }
    }

    pub fn draw(&self, modelview: &mut Modelview) {
        for ch in self.characters.iter() {
            ch.draw(modelview);
        }
    }
}
