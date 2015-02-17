extern crate gl;

use gl::types::{GLuint, GLint, GLenum};
use std::old_io::File;
use std;
use std::ffi::CString;
use paths::Paths;

pub struct Shader {
    pub id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub fn new(paths: &Paths, filename: &str, shader_type: GLenum) -> Shader {
        let mut reader = File::open(&paths.prefix.join(Path::new(filename))).unwrap();
        match reader.read_to_end() {
            Ok(content) => {
                let src = String::from_utf8(content).unwrap();
                unsafe {
                    let shader = gl::CreateShader(shader_type);
                    assert!(shader != 0);
                    gl::ShaderSource(shader, 1, &CString::from_slice(src.as_bytes()).as_ptr(),
                                     std::ptr::null());
                    gl::CompileShader(shader);
                    let mut status: GLint = gl::FALSE as GLint;
                    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
                    if status != gl::TRUE as GLint {
                        let mut buffer = [0u8; 512];
                        let mut length: i32 = 0;
                        gl::GetShaderInfoLog(shader, buffer.len() as i32, &mut length,
                                             buffer.as_mut_ptr() as *mut i8);
                        println!("Compiler log (length: {}):\n{}", length,
                                 std::str::from_utf8(std::ffi::c_str_to_bytes(
                                     std::mem::transmute(&buffer))
                                 ).unwrap());
                    }
                    Shader{ id: shader }
                }
            },
            Err(val) => {
                error!("Error reading file {}: {}", filename, val);
                Shader{ id: 0 } // FIXME: Proper error handling
            }
        }
    }
}
