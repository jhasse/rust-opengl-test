use font::ft_library;
use freetype::freetype::{FT_Face, FT_New_Face, FT_Set_Char_Size, FT_F26Dot6};
use paths::Paths;
use std::ptr;

pub struct Face {
    pub ft_face: FT_Face
}

impl Face {
    pub fn new(paths: &Paths, filename: &str, size: uint) -> Face {
        let mut face: FT_Face = ptr::null_mut();
        unsafe {
            let mut error = FT_New_Face(ft_library,
                                        paths.prefix.join(Path::new(
                                            format!("data/fonts/{}", filename)
                                        )).as_str().unwrap().to_c_str().as_ptr() as *mut i8,
                                        0, &mut face);
            assert!(error == 0);
            error = FT_Set_Char_Size(face, 0, size as FT_F26Dot6 * 64, 96, 96);
            assert!(error == 0);
        }
        return Face{ ft_face: face };
    }
}
