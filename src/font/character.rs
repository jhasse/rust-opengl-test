extern crate freetype;

use crate::texture::Texture;
use crate::shader_programs::ShaderPrograms;
use gl::types::GLfloat;

pub struct Character {
    width: GLfloat,
    left: GLfloat,
    top: GLfloat,
    texture: Option<Texture>
}

impl Character {
    pub fn new(shader_programs: &ShaderPrograms, face: &freetype::Face, ch: char) -> Character {
        (*face).load_char(ch as usize, freetype::face::RENDER).unwrap();

        let ref glyph = face.glyph();
        let ref bitmap = glyph.bitmap();

        Character {
            width: (glyph.advance().x >> 6) as GLfloat * 0.001,
            left: glyph.bitmap_left() as GLfloat * 0.001,
            top: glyph.bitmap_top() as GLfloat * 0.001,
            texture: if bitmap.width() == 0 { None } else {
                let mut buffer: Vec<u8> = Vec::new();
                for y in 0..bitmap.rows() {
                    for x in 0..bitmap.width() {
                        buffer.push(255);
                        buffer.push(255);
                        buffer.push(255);
                        buffer.push(bitmap.buffer()[(bitmap.width() * y + x) as usize]);
                    }
                }
                Some(Texture::new(shader_programs, bitmap.width(), bitmap.rows(),
                                  &*buffer))
            }
        }
    }
    pub fn draw(&self, shader_programs: &mut ShaderPrograms) {
        match self.texture {
            Some(ref t) => {
                let mut tmp = shader_programs.modelview.clone();
                tmp.translate(self.left, self.top);
                shader_programs.set_uniform(&tmp);
                t.draw();
            },
            _ => ()
        }
        shader_programs.modelview.translate(self.width, 0.0);
    }
}
