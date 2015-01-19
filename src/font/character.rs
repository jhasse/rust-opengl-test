extern crate freetype;

use freetype::ffi::FT_ULong;
use texture::Texture;
use shader_programs::ShaderPrograms;

pub struct Character {
    texture: Option<Texture>
}

impl Character {
    pub fn new(shader_programs: &ShaderPrograms, face: &mut freetype::Face, ch: char) -> Character {
        (*face).load_char(ch as FT_ULong, freetype::face::RENDER).unwrap();

        let ref bitmap = face.glyph().bitmap();

        if bitmap.width() == 0 {
            return Character{ texture: None };
        }
        Character {
            texture: Some(Texture::new(shader_programs, bitmap.width(), bitmap.rows(),
                                       bitmap.buffer()))
        }
    }
    pub fn draw(&self) {
        match self.texture {
            Some(ref t) => t.draw(),
            _ => ()
        }
    }
}
