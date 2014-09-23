use freetype::freetype::{FT_Face, FT_Get_Char_Index, FT_Load_Glyph, FT_Render_Glyph,
                         FT_LOAD_DEFAULT, FT_RENDER_MODE_NORMAL, FT_ULong, FT_GlyphSlotRec};


pub struct Character {
    dummy: int
}

impl Character {
    pub fn new(face: FT_Face, ch: char) -> Character {
        unsafe {
            let glyph_index = FT_Get_Char_Index(face, ch as FT_ULong);
            assert!(glyph_index != 0); // FIXME: Handle this case

            let mut error = FT_Load_Glyph(face, glyph_index, FT_LOAD_DEFAULT as i32);
            assert!(error == 0);

            error = FT_Render_Glyph((*face).glyph as *mut FT_GlyphSlotRec, FT_RENDER_MODE_NORMAL);
            assert!(error == 0);

            let bitmap = (*((*face).glyph as *const FT_GlyphSlotRec)).bitmap;
            println!("{} x {}", bitmap.width, bitmap.rows);

            Character{ dummy: 0 }
        }
    }
}
