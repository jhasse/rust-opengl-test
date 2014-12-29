extern crate freetype;

use paths::Paths;

pub struct Face {
    pub ft_face: freetype::Face
}

impl Face {
    pub fn new(freetype: freetype::Library, paths: &Paths, filename: &str, size: i32) -> Face {
        let mut face = freetype.new_face(paths.prefix.join(Path::new(
                                           format!("data/fonts/{}", filename)
                                           )).as_str().unwrap(), 0).unwrap();
        face.set_char_size(0, size * 64, 96, 96).unwrap();
        return Face{ ft_face: face };
    }
}
