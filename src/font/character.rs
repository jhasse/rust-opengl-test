extern crate freetype;

use texture::Texture;

pub struct Character {
    texture: Texture
}

impl Character {
    pub fn new(face: &mut freetype::Face, ch: char) -> Character {
        (*face).load_char(ch as u32, freetype::face::RENDER).unwrap();

        let ref bitmap = face.glyph().bitmap();
        println!("{} x {}", bitmap.width(), bitmap.rows());

        Character{ texture: Texture::new(bitmap.width(), bitmap.rows()) }
    }
    pub fn draw(&self) {
    }
}
