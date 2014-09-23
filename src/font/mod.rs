extern crate libc;

use freetype::freetype::{FT_Library, FT_Init_FreeType};

static mut ft_library: FT_Library = 0 as *mut libc::c_void;

pub mod face;
pub mod text;
pub mod character;

pub fn init() {
    unsafe {
        let error = FT_Init_FreeType(&mut ft_library);
        assert!(error == 0);
    }
}
