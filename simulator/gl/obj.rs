
use std::comm;
use std::{f32, u16};
use std::io;
use std::hashmap::HashMap;
use std::vec;

use math::{Mat4, Mat3, Vec4, Vec3};
use gl::shader::{AttribLocation, UniformLocation};

use es = opengles::gl2;

#[macro_escape]
#[path = "../check.rs"]
mod check;

pub struct Mesh {
    vbo_vertices: es::GLuint,
    vbo_normals: es::GLuint,
    vbo_colors: es::GLuint,
    ibo_elements: es::GLuint,
    object2world: Mat4<f32>,
    vertices: ~[Vec4<f32>],
    colors: ~[Vec4<f32>],
    normals: ~[Vec3<f32>],
    elements: ~[es::GLushort],
    port: Option<comm::Port<(f32, f32, f32)>>,
}

impl Mesh {
    pub fn load_from_obj(reader: @Reader) -> Mesh {
        let mut vertices: ~[Vec4<f32>] = ~[];
        let mut colors: ~[Vec4<f32>] = ~[];
        let mut normals: ~[Vec3<f32>] = ~[];
        let mut elements: ~[es::GLushort] = ~[];
        
        for reader.each_line |line| {
            if line.starts_with("v ") {
                let str_vals: ~[&str] = line.split_iter(' ').collect();
                let x = f32::from_str_radix(str_vals[1], 10);
                let y = f32::from_str_radix(str_vals[2], 10);
                let z = f32::from_str_radix(str_vals[3], 10);
                
                match (x, y, z) {
                    (Some(x), Some(y), Some(z)) => {
                        let v = Vec4::new(x, y, z, 1.0);
                        vertices.push(v);
                    },
                    _ => fail!("Invalid obj file")
                }
            } else if line.starts_with("f ") {
                let str_vals: ~[&str] = line.split_iter(' ').collect();
                let a = u16::from_str_radix(str_vals[1], 10);
                let b = u16::from_str_radix(str_vals[2], 10);
                let c = u16::from_str_radix(str_vals[3], 10);
                
                match (a, b, c) {
                    (Some(a), Some(b), Some(c)) => {
                        elements.push(a - 1);
                        elements.push(b - 1);
                        elements.push(c - 1);
                    },
                    _ => fail!("Invalid obj file")
                }
            } else {}
        }
        
        normals.grow(vertices.len(), &Vec3::new(0.0, 0.0, 0.0));
        colors.grow(vertices.len(), &Vec4::new(0.0, 0.0, 0.0, 1.0));
        println(fmt!("# of vertices: %?", vertices.len()));
        println(fmt!("# of elements: %?", elements.len()));
        let mut i = 0;
        while i < elements.len() {
            let (ia, ib, ic) = (elements[i], elements[i+1], elements[i+2]);
            let normal = (Vec3::from4(vertices[ib]) - Vec3::from4(vertices[ia]))
                .cross(&(Vec3::from4(vertices[ic]) - Vec3::from4(vertices[ia]))).normalize();
            normals[ia] = normal;
            normals[ib] = normal;
            normals[ic] = normal;
            i += 3;
        }
        
        Mesh {
            vbo_vertices: 0,
            vbo_colors: 0,
            vbo_normals: 0,
            ibo_elements: 0,
            object2world: Mat4::ident(),
            vertices: vertices,
            colors: colors,
            normals: normals,
            elements: elements,
            port: None,
        }
    }
    
    pub fn gen_base() -> Mesh {
        let vertices = ~[Vec4::new(0.25, -0.26, -0.25, 1.0), Vec4::new(0.25, -0.26, 0.25, 1.0),
                         Vec4::new(-0.25f32, -0.26, 0.25, 1.0), Vec4::new(-0.25, -0.26, -0.25, 1.0),
                         Vec4::new(0.25, 0.0, -0.25, 1.0), Vec4::new(0.25, 0.0, 0.25, 1.0),
                         Vec4::new(-0.25f32, 0.0, 0.25, 1.0), Vec4::new(-0.25, 0.0, -0.25, 1.0),];
        let mut colors = ~[];
        let elements = ~[0, 1, 2,
                         4, 7, 5,
                         0, 4, 1,
                         1, 5, 2,
                         2, 6, 3,
                         4, 0, 3,
                         3, 0, 2,
                         5, 6, 2,
                         7, 6, 5,
                         6, 7, 3,
                         7, 4, 3,
                         4, 5, 1,];
        let mut normals = ~[];
        
        normals.grow(vertices.len(), &Vec3::new(0.0f32, 0.0, 0.0));
        colors.grow(vertices.len(), &Vec4::new(0.0f32, 0.0, 0.0, 1.0));
        println(fmt!("# of vertices: %?", vertices.len()));
        println(fmt!("# of elements: %?", elements.len()));
        let mut i = 0;
        while i < elements.len() {
            let (ia, ib, ic) = (elements[i], elements[i+1], elements[i+2]);
            let normal = (Vec3::from4(vertices[ib]) - Vec3::from4(vertices[ia]))
                .cross(&(Vec3::from4(vertices[ic]) - Vec3::from4(vertices[ia]))).normalize();
            normals[ia] = normal;
            normals[ib] = normal;
            normals[ic] = normal;
            i += 3;
        }
        
        Mesh {
            vbo_vertices: 0,
            vbo_colors: 0,
            vbo_normals: 0,
            ibo_elements: 0,
            object2world: Mat4::ident(),
            vertices: vertices,
            colors: colors,
            normals: normals,
            elements: elements,
            port: None
        }
    }
    
    pub fn better_rod(x: f32, y: f32, height: f32, port: comm::Port<(f32, f32, f32)>) -> Mesh {
        let mut basic = Mesh::load_from_obj_file("cylinder.obj");
        basic.vertices = do basic.vertices.map |v| {
            Vec4::new(v.x * 0.02 + x, v.y * height, v.z * 0.02 + y, v.w)
        };
        basic.colors = vec::from_elem(basic.vertices.len(), Vec4::new(1.0f32, 0.0, 0.0, 0.6));
        basic.port = Some(port);
        basic
    }
    
    #[inline]
    pub fn load_from_obj_file(path_str: &str) -> Mesh {
        Mesh::load_from_obj(match io::file_reader(&Path(path_str)) {
            Ok(reader) => reader,
            Err(error) => fail!(error)
        })
    }
    
    pub fn upload(&mut self) {
        if self.vertices.len() > 0 {
            self.vbo_vertices = check!(es::gen_buffers(1)[0]);
            check!(es::bind_buffer(es::ARRAY_BUFFER, self.vbo_vertices));
            check!(es::buffer_data(es::ARRAY_BUFFER, self.vertices, es::STATIC_DRAW));
        }
        
        if self.colors.len() > 0 {
            self.vbo_colors = check!(es::gen_buffers(1)[0]);
            check!(es::bind_buffer(es::ARRAY_BUFFER, self.vbo_colors));
            check!(es::buffer_data(es::ARRAY_BUFFER, self.colors, es::DYNAMIC_DRAW));
        }
        
        if self.normals.len() > 0 {
            self.vbo_normals = check!(es::gen_buffers(1)[0]);
            check!(es::bind_buffer(es::ARRAY_BUFFER, self.vbo_normals));
            check!(es::buffer_data(es::ARRAY_BUFFER, self.normals, es::STATIC_DRAW));
        }
        
        if self.elements.len() > 0 {
            self.ibo_elements = check!(es::gen_buffers(1)[0]);
            check!(es::bind_buffer(es::ELEMENT_ARRAY_BUFFER, self.ibo_elements));
            check!(es::buffer_data(es::ELEMENT_ARRAY_BUFFER, self.elements, es::STATIC_DRAW));
        }
    }
    
    pub fn reload_colors(&self) {
        if self.colors.len() > 0 {
            check!(es::bind_buffer(es::ARRAY_BUFFER, self.vbo_colors));
            check!(es::buffer_sub_data(es::ARRAY_BUFFER, 0, self.colors));
        }
    }
    
    pub fn translate(&mut self, translation: Vec3<f32>) {
        self.object2world = self.object2world.translate(translation);
    }
    
    pub fn draw(&mut self, model:Mat4<f32>, attribs: &HashMap<~str, AttribLocation>, uniforms: &HashMap<~str, UniformLocation>) {
        if !self.uploaded() {
            fail!("Hey! You haven't uploaded this mesh yet!'");
        }
        
        match self.port {
            Some(ref port) => {
                if port.peek() {
                    let (r, g, b) = port.recv();
                    self.colors = vec::from_elem(self.colors.len(), Vec4::new(r, g, b, 0.6));
                    self.reload_colors();
                }
            },
            None => {}
        }
        
        attribs.find_equiv(&("v_coord")).get().update_f32(self.vbo_vertices, 4);
        attribs.find_equiv(&("v_normal")).get().update_f32(self.vbo_normals, 3);
        attribs.find_equiv(&("v_color")).get().update_f32(self.vbo_colors, 4);
        
        //let angle = (glfw::get_time() * 3.14159 / 4.0) as f32;
        //let my_model = self.object2world.rotate(angle, Vec3::new(0.0f32, 1.0, 0.0));
        let final_model = model /* * my_model */;
        let m_inv_transp = Mat3::from_four(final_model).trans_inv();
        
        uniforms.find_equiv(&("m_orig")).get().update_mat4_f32(final_model);
        uniforms.find_equiv(&("m")).get().update_mat4_f32(final_model);
        uniforms.find_equiv(&("m_inv_transp")).get().update_mat3_f32(m_inv_transp);
        
        check!(es::bind_buffer(es::ELEMENT_ARRAY_BUFFER, self.ibo_elements));
        check!(es::draw_elements(es::TRIANGLES, self.elements.len() as es::GLint, es::UNSIGNED_SHORT, None));
    }
    
    #[inline]
    pub fn uploaded(&self) -> bool {
        self.vbo_vertices != 0 && self.vbo_normals != 0 && self.ibo_elements != 0
    }
}

impl Drop for Mesh {
    fn drop(&self) {
        es::delete_buffers([self.vbo_vertices, self.vbo_normals, self.ibo_elements]);
    }
}