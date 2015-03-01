extern crate freetype;

use paths::Paths;
use std::path::AsPath;
use std::path::Path;
use std;

pub struct Face {
    pub ft_face: freetype::Face
}

impl Face {
    pub fn new(freetype: freetype::Library, paths: &Paths, filename: &str, size: isize) -> Face {
        let mut path = paths.prefix;
        path.push("data");
        path.push("font");
        path.push(filename);
        let tmp = std::old_path::posix::Path::new(path.to_str().unwrap());
        let face = freetype.new_face(&tmp, 0).unwrap();
        face.set_char_size(0, size * 64, 96, 96).unwrap();
        return Face{ ft_face: face };
    }
}
