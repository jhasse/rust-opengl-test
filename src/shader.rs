extern crate gl;

use gl::types::{GLuint, GLint, GLenum};
use std::io::File;
use std;
use paths::Paths;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(paths: &Paths, filename: &str, shaderType: GLenum) -> Shader {
        let mut reader = File::open(&paths.prefix.join(Path::new(filename))).unwrap();
        match reader.read_to_end() {
            Ok(content) => {
                let src = std::str::from_utf8_owned(content).unwrap();
                unsafe {
                    let shader = gl::CreateShader(shaderType);
                    assert!(shader != 0);
                    src.with_c_str(|src| {
                        gl::ShaderSource(shader, 1, &src, std::ptr::null());
                    });
                    gl::CompileShader(shader);
                    let mut status: GLint = gl::FALSE as GLint;
                    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
                    if status != gl::TRUE as GLint {
                        let mut buffer = [0i8, ..512];
                        let mut length: i32 = 0;
                        gl::GetShaderInfoLog(shader, buffer.len() as i32, &mut length,
                                             &mut buffer[0]);
                        println!("Compiler log (length: {}):\n{}", length,
                                 std::str::raw::from_c_str(&buffer[0]));
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
