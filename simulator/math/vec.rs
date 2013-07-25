
use std::num::One;
use std::cast::transmute;

pub struct Vec4<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T: Clone> Clone for Vec4<T> {
    #[inline]
    pub fn clone(&self) -> Vec4<T> {
        Vec4 { x: self.x.clone(), y: self.y.clone(),
               z: self.z.clone(), w: self.w.clone() }
    }
}

impl<T> Vec4<T> {
    #[inline(always)]
    pub fn new(nx: T, ny: T, nz: T, nw: T) -> Vec4<T> {
        Vec4 {x: nx, y: ny, z: nz, w: nw}
    }
    
    pub fn from_fn(f: &fn(u8) -> T) -> Vec4<T> {
        Vec4 {x: f(0), y: f(1), z: f(2), w: f(3)}
    }
}

impl<T: Clone> Vec4<T> {
    #[inline(always)]
    pub fn to_vec(&self) -> [T, ..4] {
        [self.x.clone(), self.y.clone(), self.z.clone(), self.w.clone()]
    }
    
    #[inline]
    pub fn from_array(arr: [T, ..4]) -> Vec4<T> {
        unsafe { transmute(arr) }
    }
    
    #[inline]
    pub fn to_arr(&self) -> [T, ..4] {
        [self.x.clone(), self.y.clone(), self.z.clone(), self.w.clone()]
    }
}

impl<T: Add<T, T>> Add<Vec4<T>, Vec4<T>> for Vec4<T> {
    pub fn add(&self, rhs: &Vec4<T>) -> Vec4<T> {
        Vec4 {x: self.x + rhs.x, y: self.y + rhs.y,
             z: self.z + rhs.z, w: self.w + rhs.w}
    }
}

impl<T: Sub<T, T>> Sub<Vec4<T>, Vec4<T>> for Vec4<T> {
    pub fn sub(&self, rhs: &Vec4<T>) -> Vec4<T> {
        Vec4 {x: self.x - rhs.x, y: self.y - rhs.y,
             z: self.z - rhs.z, w: self.w - rhs.w}
    }
}

impl<T: Mul<T, T>> Mul<T, Vec4<T>> for Vec4<T> {
    pub fn mul(&self, rhs: &T) -> Vec4<T> {
        Vec4 {x: self.x * *rhs, y: self.y * *rhs,
             z: self.z * *rhs, w: self.w * *rhs}
    }
}

impl<T: Clone> Index<uint, T> for Vec4<T> {
    fn index(&self, rhs: &uint) -> T {
        match *rhs {
            0 => self.x.clone(),
            1 => self.y.clone(),
            2 => self.z.clone(),
            3 => self.w.clone(),
            _ => fail!("index out of bounds: the len is 4 but the index is %?", *rhs)
        }
    }
}

pub type Vec4f = Vec4<f32>;


pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T> {
    pub fn new(nx: T, ny: T, nz: T) -> Vec3<T> {
        Vec3 {x: nx, y: ny, z: nz}
    }
}

impl<T: Clone> Clone for Vec3<T> {
    pub fn clone(&self) -> Vec3<T> {
        Vec3 {x: self.x.clone(), y: self.y.clone(), z: self.z.clone()}
    }
}

impl<T: Clone> Vec3<T> {
    pub fn from4(other: Vec4<T>) -> Vec3<T> {
        Vec3 {x: other.x.clone(), y: other.y.clone(), z: other.z.clone()}
    }
}

impl<T: Sub<T, T>> Sub<Vec3<T>, Vec3<T>> for Vec3<T> {
    pub fn sub(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Sub<T, T> + Mul<T, T>> Vec3<T> {
    pub fn cross(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T: Add<T, T>> Add<Vec3<T>, Vec3<T>> for Vec3<T> {
    pub fn add(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Add<T, T> + Mul<T, T>> Vec3<T> {
    pub fn dot(&self, rhs: &Vec3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Add<T, T> + Mul<T, T> + Algebraic + One> Vec3<T> {
    pub fn normalize(&self) -> Vec3<T> {
        let sqr = self.x * self.x + self.y * self.y + self.z * self.z;
        *self * sqr.rsqrt()
    }
}

impl<T: Clone> Index<uint, T> for Vec3<T> {
    fn index(&self, rhs: &uint) -> T {
        match *rhs {
            0 => self.x.clone(),
            1 => self.y.clone(),
            2 => self.z.clone(),
            _ => fail!("index out of bounds: the len is 2 but the index is %?", *rhs)
        }
    }
}

impl<T: Mul<T, T>> Mul<T, Vec3<T>> for Vec3<T> {
    pub fn mul(&self, rhs: &T) -> Vec3<T> {
        Vec3 {x: self.x * *rhs, y: self.y * *rhs, z: self.z * *rhs}
    }
}

pub type Vec3f = Vec3<f32>;