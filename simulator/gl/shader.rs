
use std::io;
use es = opengles::gl2;

use math::{Mat4, Mat3};

#[macro_escape]
#[path = "../check.rs"]
mod check;

pub struct AttribLocation {raw: es::GLint}
pub struct UniformLocation {raw: es::GLint}

impl AttribLocation {
    pub fn enable_vertex_attrib_array(&self) {
        check!(es::enable_vertex_attrib_array(self.raw as es::GLuint));
    }

    pub fn vertex_attrib_pointer_f32(&self, size: es::GLint, normalized: bool,
                              stride: es::GLsizei, offset: es::GLuint) {
        check!(es::vertex_attrib_pointer_f32(self.raw as es::GLuint,
                                             size, normalized, stride, offset));
    }
    
    pub fn update_f32(&self, buf: u32, size: es::GLint) {
        self.enable_vertex_attrib_array();
        check!(es::bind_buffer(es::ARRAY_BUFFER, buf));
        self.vertex_attrib_pointer_f32(size, false, 0, 0);
    }
}

impl UniformLocation {
    pub fn update_f32(&self, val: f32) {
        check!(es::uniform_1f(self.raw, val));
    }
    pub fn update_mat4_f32(&self, mat: Mat4<f32>) {
        check!(es::uniform_matrix_4fv(self.raw, false, *mat.to_flat()));
    }
    pub fn update_mat3_f32(&self, mat: Mat3<f32>) {
        check!(es::uniform_matrix_3fv(self.raw, false, *mat.to_flat()));
    }
}

pub struct Shader {
    prog: es::GLuint,
    vert_obj: es::GLuint,
    frag_obj: es::GLuint,
}

fn load_shader(path_str: ~str, shader_type: es::GLuint) -> es::GLuint {
    let path = Path(path_str);
    let reader = match io::file_reader(&path) {
        Ok(reader) => reader,
        Err(error) => fail!(error)
    };
    
    let shader = check!(es::create_shader(shader_type));
    check!(es::shader_source(shader, [reader.read_whole_stream()]));
    check!(es::compile_shader(shader));

    match check!(es::get_shader_iv(shader, es::COMPILE_STATUS)) {
        0 => fail!(check!(es::get_shader_info_log(shader))),
        _ => shader
    }
}

impl Shader {
    pub fn from_files(vertex_file: &str, fragment_file: &str) -> Shader {
        let shader_program = check!(es::create_program());
        let vertex_shader = load_shader(vertex_file.to_owned(), es::VERTEX_SHADER);
        let fragment_shader = load_shader(fragment_file.to_owned(), es::FRAGMENT_SHADER);
        
        check!(es::attach_shader(shader_program, vertex_shader));
        check!(es::attach_shader(shader_program, fragment_shader));
        check!(es::link_program(shader_program));
        check!(es::use_program(shader_program));
        
        Shader {
            prog: shader_program,
            vert_obj: vertex_shader,
            frag_obj: fragment_shader,
        }
    }
    
    pub fn get_attrib_location(&self, name: &str) -> AttribLocation {
        let loc = check!(es::get_attrib_location(self.prog, name.to_owned()));
        AttribLocation {
            raw: match loc {
                -1 => { error!(fmt!("%s attrib not found", name)); loc },
                _ => loc
            }
        }
    }
    
    pub fn get_uniform_location(&self, name: &str) -> UniformLocation {
        let loc = check!(es::get_uniform_location(self.prog, name.to_owned()));
        UniformLocation {
            raw: match loc {
                -1 => { error!(fmt!("%s uniform not found", name)); loc },
                _ => loc
            }
        }
    }
}
