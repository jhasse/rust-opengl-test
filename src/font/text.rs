use font::character::Character;
use font::face::Face;

pub struct Text {
    characters: Vec<Character>
}

impl Text {
    pub fn new(face: &mut Face, text: &str) -> Text {
        let mut characters: Vec<Character> = vec![];
        for ch in text.chars() {
            characters.push(Character::new(&mut face.ft_face, ch));
        }
        Text{ characters: characters }
    }

    pub fn draw(&self) {
        for ch in self.characters.iter() {
            ch.draw();
        }
    }
}
