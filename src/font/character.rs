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
        let mut buffer: Vec<u8> = Vec::new();
        for y in range(0, bitmap.rows()) {
            for x in range(0, bitmap.width()) {
                buffer.push(255);
                buffer.push(255);
                buffer.push(255);
                buffer.push(bitmap.buffer()[(bitmap.width() * y + x) as usize]);
            }
        }
        Character {
            texture: Some(Texture::new(shader_programs, bitmap.width(), bitmap.rows(),
                                       buffer.as_slice()))
        }
    }
    pub fn draw(&self) {
        match self.texture {
            Some(ref t) => t.draw(),
            _ => ()
        }
    }
}
