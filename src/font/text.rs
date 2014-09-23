use font::character::Character;
use font::face::Face;

pub struct Text {
    characters: Vec<Character>
}

impl Text {
    pub fn new(face: &Face, text: &str) -> Text {
        let mut characters: Vec<Character> = vec![];
        for ch in text.chars() {
            characters.push(Character::new(face.ft_face, ch));
        }
        Text{ characters: characters }
    }
}
