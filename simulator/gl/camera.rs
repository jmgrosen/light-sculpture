
use glfw;

use std::uint;
use std::libc::c_int;
use std::hashmap::HashMap;

use math::{Vec3, Mat4};
use gl::Mesh;
use gl::shader::{Shader, AttribLocation, UniformLocation};

use glfw::Window;

pub struct Camera {
    window: ~Window,
    translation: Vec3<f32>,
    rotation: Vec3<f32>,
    eye: Vec3<f32>,
    center: Vec3<f32>,
    up: Vec3<f32>,
    fovy: f32,
    aspect: f32,
    z_near: f32,
    z_far: f32,
    program: ~Shader,
    attribs: ~HashMap<~str, AttribLocation>,
    uniforms: ~HashMap<~str, UniformLocation>,
    meshes: ~[Mesh],
}

impl Camera {
    pub fn new(window: ~Window, shader_name: &str) -> Camera {
        let (width, height) = window.get_size();
        let program = Shader::from_files(fmt!("%s.v.glsl", shader_name), fmt!("%s.f.glsl", shader_name));
        let zero_vec = Vec3::new(0.0f32, 0.0, 0.0);
        
        let mut attribs = HashMap::new();
        attribs.insert(~"v_coord", program.get_attrib_location("v_coord"));
        attribs.insert(~"v_normal", program.get_attrib_location("v_normal"));
        attribs.insert(~"v_color", program.get_attrib_location("v_color"));
        
        let mut uniforms = HashMap::new();
        uniforms.insert(~"m_orig", program.get_uniform_location("m_orig"));
        uniforms.insert(~"m", program.get_uniform_location("m"));
        uniforms.insert(~"v", program.get_uniform_location("v"));
        uniforms.insert(~"p", program.get_uniform_location("p"));
        uniforms.insert(~"m_inv_transp", program.get_uniform_location("m_inv_transp"));
        
        Camera {
            window: window,
            translation: zero_vec.clone(),
            rotation: zero_vec.clone(),
            eye: zero_vec.clone(),
            center: zero_vec.clone(),
            up: zero_vec.clone(),
            fovy: 0.0,
            aspect: (width as f32) / (height as f32),
            z_near: 0.0,
            z_far: 0.0,
            program: ~program,
            attribs: ~attribs,
            uniforms: ~uniforms,
            meshes: ~[],
        }
    }
    
    pub fn translate(&mut self, translation: Vec3<f32>) {
        self.translation = self.translation + translation;
    }
    
    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = self.rotation + Vec3::new(x, y, z);
    }
    
    pub fn calc_model(&self) -> Mat4<f32> {
        let mut mat = Mat4::ident().translate(self.translation);
        mat = mat.rotate(self.rotation.x, Vec3::new(1.0, 0.0, 0.0));
        mat = mat.rotate(self.rotation.y, Vec3::new(0.0, 1.0, 0.0));
        mat = mat.rotate(self.rotation.z, Vec3::new(0.0, 0.0, 1.0));
        mat
    }
    
    pub fn look_at(&mut self, eye: Vec3<f32>, center: Vec3<f32>, up: Vec3<f32>) {
        self.eye = eye;
        self.center = center;
        self.up = up;
    }
    
    pub fn calc_view(&self) -> Mat4<f32> {
        
        let f = (self.center - self.eye).normalize();
        let s = f.cross(&self.up.normalize()).normalize();
        let u = s.cross(&f);
        
        let mut result = Mat4::from_elem(1.0f32);
        result.data[0][0] =  s.x.clone();
        result.data[1][0] =  s.y.clone();
        result.data[2][0] =  s.z.clone();
        result.data[0][1] =  u.x.clone();
        result.data[1][1] =  u.y.clone();
        result.data[2][1] =  u.z.clone();
        result.data[0][2] = -f.x.clone();
        result.data[1][2] = -f.y.clone();
        result.data[2][2] = -f.z.clone();
        result.data[3][0] = -s.dot(&self.eye).clone();
        result.data[3][1] = -u.dot(&self.eye).clone();
        result.data[3][2] =  f.dot(&self.eye).clone();
        
        result
    }
    
    pub fn perspective(&mut self, fovy: f32, z_near: f32, z_far: f32) {
        self.fovy = fovy;
        self.z_near = z_near;
        self.z_far = z_far;
    }
    
    pub fn calc_projection(&self) -> Mat4<f32> {
        let tan_half_fovy = (self.fovy / 2.0).tan();
        
        let mut result = Mat4::from_elem(0.0f32);
        result.data[0][0] = 1.0 / (self.aspect * tan_half_fovy);
        result.data[1][1] = 1.0 / tan_half_fovy;
        result.data[2][2] = - (self.z_far + self.z_near) / (self.z_far - self.z_near);
        result.data[2][3] = -1.0;
        result.data[3][2] = - (2.0 * self.z_far * self.z_near) / (self.z_far - self.z_near);
        
        result
    }
    
    pub fn add_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
    
    pub fn draw(&mut self) {
        let model = self.calc_model();
        let view = self.calc_view();
        let projection = self.calc_projection();
        
        self.uniforms.find_equiv(&("v")).get().update_mat4_f32(view);
        self.uniforms.find_equiv(&("p")).get().update_mat4_f32(projection);
        
        for uint::range(0, self.meshes.len()) |i| {
            if !self.meshes[i].uploaded() { self.meshes[i].upload(); }
            self.meshes[i].draw(model, self.attribs, self.uniforms);
        }
        
        self.window.swap_buffers();
    }
    
    pub fn is_key_down(&self, key: c_int) -> bool {
        match self.window.get_key(key) {
            glfw::PRESS => true,
            _ => false,
        }
    }
    
    pub fn resize(&mut self, _size: (int, int)) {
        //let (width, height) = size;
    }
    
    
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
}
