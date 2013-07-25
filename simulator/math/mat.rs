
use std::uint;
use std::cast::transmute;
use std::num::{Zero, One, Trigonometric};
use std::to_str::ToStr;

use math::{Vec3, Vec4};

macro_rules! z(
    ($Type:ty) => (Zero::zero::<$Type>())
)

macro_rules! o(
    ($Type:ty) => (One::one::<$Type>())
)


// Column-major!
pub struct Mat4<T> {
    data: [[T, ..4], ..4]
}

// impl<T: Clone> Clone for Mat4<T> {
//     #[inline]
//     pub fn clone(&self) -> Mat4<T> {
//         Mat4 { data:  }
//     }
// }

impl<T> Mat4<T> {
    #[inline(always)]
    pub fn new(col1: [T, ..4], col2: [T, ..4], col3: [T, ..4], col4: [T, ..4]) -> Mat4<T> {
        Mat4 { data: [col1, col2, col3, col4] }
    }

    #[inline(always)]
    pub fn to_flat(&self) -> &[T, ..16] {
        unsafe { transmute(&self.data) }
    }
}

impl<T: Zero + Clone> Zero for Mat4<T> {
    #[inline(always)]
    pub fn zero() -> Mat4<T> {
        Mat4 { data: [[z!(T), z!(T), z!(T), z!(T)],
                      [z!(T), z!(T), z!(T), z!(T)],
                      [z!(T), z!(T), z!(T), z!(T)],
                      [z!(T), z!(T), z!(T), z!(T)]] }
    }
    pub fn is_zero(&self) -> bool {
        for uint::range(0, 4) |i| {
            for uint::range(0, 4) |j| {
                if !self.data[i][j].is_zero() {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T: Zero + Clone> Mat4<T> {
    #[inline(always)]
    pub fn from_elem(elem: T) -> Mat4<T> {
        Mat4 { data: [[elem.clone(), z!(T), z!(T), z!(T)], [z!(T), elem.clone(), z!(T), z!(T)],
                      [z!(T), z!(T), elem.clone(), z!(T)], [z!(T), z!(T), z!(T), elem.clone()]]}
    }
}

impl<T: Zero + One + Clone> Mat4<T> {
    #[inline]
    pub fn ident() -> Mat4<T> {
        Mat4::from_elem(o!(T))
    }
}

impl<T: Clone> Mat4<T> {
    pub fn col(&self, i: u8) -> Vec4<T> {
        Vec4::new(self.data[i][0].clone(), self.data[i][1].clone(), self.data[i][2].clone(), self.data[i][3].clone())
    }
}

impl<T: Mul<T, T> + Add<T, T> + Zero + Clone + ToStr> Mat4<T> {
    pub fn translate(&self, v: Vec3<T>) -> Mat4<T> {
        let mut result = z!(Mat4<T>);
        result.data[0] = [self.data[0][0].clone(), self.data[0][1].clone(), self.data[0][2].clone(), self.data[0][3].clone()];
        result.data[1] = [self.data[1][0].clone(), self.data[1][1].clone(), self.data[1][2].clone(), self.data[1][3].clone()];
        result.data[2] = [self.data[2][0].clone(), self.data[2][1].clone(), self.data[2][2].clone(), self.data[2][3].clone()];
        result.data[3] = (self.col(0) * v.x + self.col(1) * v.y + self.col(2) * v.z + self.col(3)).to_vec();
        result
    }
    pub fn mul_with_log(&self, rhs: &Mat4<T>) -> Mat4<T> {
        let mut new = z!(Mat4<T>);
        for uint::range(0, 4) |i| {
            for uint::range(0, 4) |j| {
                println(fmt!("column %?, row %?:", i, j));
                println(fmt!("self.data[%?][0] = %?", i, self.data[i][0]));
                println(fmt!("self.data[%?][1] = %?", i, self.data[i][1]));
                println(fmt!("self.data[%?][2] = %?", i, self.data[i][2]));
                println(fmt!("self.data[%?][3] = %?", i, self.data[i][3]));

                println(fmt!("self.rhs[0][%?] = %?", j, rhs.data[0][j]));
                println(fmt!("self.rhs[1][%?] = %?", j, rhs.data[1][j]));
                println(fmt!("self.rhs[2][%?] = %?", j, rhs.data[2][j]));
                println(fmt!("self.rhs[3][%?] = %?", j, rhs.data[3][j]));
                new.data[j][i] = self.data[i][0] * rhs.data[0][j] +
                                 self.data[i][1] * rhs.data[1][j] +
                                 self.data[i][2] * rhs.data[2][j] + 
                                 self.data[i][3] * rhs.data[3][j];
                println(fmt!("%s", new.to_str()));
            }
        }
        new
    }
}

impl<T: Mul<T, T> + Add<T, T> + Sub<T, T> + Zero + One + Trigonometric + Clone> Mat4<T> {
    pub fn rotate(&self, angle: T, axis: Vec3<T>) -> Mat4<T> {
        let mut rotate = z!(Mat4<T>);
        let sin = angle.sin();
        let cos = angle.cos();
        let one = o!(T);

        rotate.data[0][0] = axis.x * axis.x * (one.clone() - cos) + cos;
        rotate.data[1][0] = axis.y * axis.x * (one.clone() - cos) - axis.z * sin;
        rotate.data[2][0] = axis.z * axis.x * (one.clone() - cos) + axis.y * sin;
        rotate.data[0][1] = axis.x * axis.y * (one.clone() - cos) + axis.z * sin;
        rotate.data[1][1] = axis.y * axis.y * (one.clone() - cos) + cos;
        rotate.data[2][1] = axis.z * axis.y * (one.clone() - cos) - axis.x * sin;
        rotate.data[0][2] = axis.x * axis.z * (one.clone() - cos) - axis.y * sin;
        rotate.data[1][2] = axis.y * axis.z * (one.clone() - cos) + axis.x * sin;
        rotate.data[2][2] = axis.z * axis.z * (one.clone() - cos) + cos;
        rotate.data[3][3] = one.clone();
        
        let mut result = z!(Mat4<T>);
        result.data[0] = (self.col(0) * rotate.data[0][0] + self.col(1) * rotate.data[0][1] + self.col(2) * rotate.data[0][2]).to_arr();
        result.data[1] = (self.col(0) * rotate.data[1][0] + self.col(1) * rotate.data[1][1] + self.col(2) * rotate.data[1][2]).to_arr();
        result.data[2] = (self.col(0) * rotate.data[2][0] + self.col(1) * rotate.data[2][1] + self.col(2) * rotate.data[2][2]).to_arr();
        result.data[3] = [self.data[3][0].clone(), self.data[3][1].clone(), self.data[3][2].clone(), self.data[3][3].clone()];
        
        result
    }
}

impl<T: Clone> Index<uint, [T, ..4]> for Mat4<T> {
    fn index(&self, rhs: &uint) -> [T, ..4] {
        [self.data[*rhs][0].clone(), self.data[*rhs][1].clone(), self.data[*rhs][2].clone(), self.data[*rhs][3].clone()]
    }
}

impl<T: Add<T, T> + Clone + Zero> Add<Mat4<T>, Mat4<T>> for Mat4<T> {
    fn add(&self, rhs: &Mat4<T>) -> Mat4<T> {
        let mut new = z!(Mat4<T>);
        for uint::range(0, 4) |i| {
            for uint::range(0, 4) |j| {
                new.data[i][j] = new.data[i][j] + rhs.data[i][j];
            }
        }
        new
    }
}

impl<T: Mul<T, T> + Add<T, T> + Clone + Zero> Mul<Mat4<T>, Mat4<T>> for Mat4<T> {
    fn mul(&self, m: &Mat4<T>) -> Mat4<T> {
        let mut result = Zero::zero::<Mat4<T>>();
        for uint::range(0, 4) |i| {
            for uint::range(0, 4) |j| {
                result.data[j][i] = self.data[0][i] * m.data[j][0] +
                                    self.data[1][i] * m.data[j][1] +
                                    self.data[2][i] * m.data[j][2] +
                                    self.data[3][i] * m.data[j][3];
            }
        }
        result
    }
}

impl<T: Eq> Eq for Mat4<T> {
    pub fn eq(&self, other: &Mat4<T>) -> bool {
        for uint::range(0, 4) |i| {
            for uint::range(0, 4) |j| {
                if self.data[i][j] != other.data[i][j] {
                    return false;
                }
            }
        }
        return true;
    }
    pub fn ne(&self, other: &Mat4<T>) -> bool {
        for uint::range(0, 4) |i| {
            for uint::range(0, 4) |j| {
                if self.data[i][j] == other.data[i][j] {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T: ToStr> ToStr for Mat4<T> {
    fn to_str(&self) -> ~str {
        fmt!("%? %? %? %?\n%? %? %? %?\n%? %? %? %?\n%? %? %? %?\n",
            self.data[0][0], self.data[1][0], self.data[2][0], self.data[3][0],
            self.data[0][1], self.data[1][1], self.data[2][1], self.data[3][1],
            self.data[0][2], self.data[1][2], self.data[2][2], self.data[3][2],
            self.data[0][3], self.data[1][3], self.data[2][3], self.data[3][3])
    }
}

#[test]
fn test_mult() {
    let foo = Mat4::new([9, 4, 9, 6], [0, 7, 6, 7], [8, 9, 1, 5], [0, 0, 9, 1]);
    let bar = Mat4::new([0, 7, 1, 6], [6, 6, 3, 6], [2, 2, 3, 9], [0, 9, 9, 1]);
    assert_eq!(foo * bar, Mat4::new([8, 58, 97, 60], [78, 93, 147, 99],
                                    [42, 49, 114, 50], [72, 144, 72, 109]));
}

/*
impl<T: Mul<T, T> + Add<T, T> + Clone> Mul<Vec4<T>, Vec4<T>> for Mat4<T> {
    fn mul(&self, rhs: &Vec4<T>) -> Vec4<T> {
        do Vec4::from_fn |i| {
            self.data[i][0] * rhs.x + self.data[i][0] * rhs.y +
                self.data[i][2] * rhs.z + self.data[i][3] * rhs.w
        }
    }
}
*/
type Mat4f = Mat4<f32>;


// Column-major!
pub struct Mat3<T> {
    data: [[T, ..3], ..3]
}

impl<T> Mat3<T> {
    #[inline(always)]
    pub fn to_flat(&self) -> &[T, ..9] {
        unsafe { transmute(&self.data) }
    }
}

impl<T: Eq> Eq for Mat3<T> {
    pub fn eq(&self, other: &Mat3<T>) -> bool {
        for uint::range(0, 3) |i| {
            for uint::range(0, 3) |j| {
                if self.data[i][j] != other.data[i][j] {
                    return false;
                }
            }
        }
        return true;
    }
    pub fn ne(&self, other: &Mat3<T>) -> bool {
        for uint::range(0, 3) |i| {
            for uint::range(0, 3) |j| {
                if self.data[i][j] == other.data[i][j] {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T: Clone> Mat3<T> {
    pub fn from_four(other: Mat4<T>) -> Mat3<T> {
        Mat3 { data: [
            [other.data[0][0].clone(), other.data[0][1].clone(), other.data[0][2].clone()],
            [other.data[1][0].clone(), other.data[1][1].clone(), other.data[1][2].clone()],
            [other.data[2][0].clone(), other.data[2][1].clone(), other.data[2][2].clone()],
        ]}
    }
}

impl<T: Div<T, T>> Div<T, Mat3<T>> for Mat3<T> {
    pub fn div(&self, rhs: &T) -> Mat3<T> {
        Mat3 { data: [
            [self.data[0][0] / *rhs, self.data[0][1] / *rhs, self.data[0][2] / *rhs],
            [self.data[1][0] / *rhs, self.data[1][1] / *rhs, self.data[1][2] / *rhs],
            [self.data[2][0] / *rhs, self.data[2][1] / *rhs, self.data[2][2] / *rhs],
        ]}
    }
}

impl<T: Clone + Mul<T, T> + Div<T, T> + Sub<T, T> + Add<T, T> + Neg<T>> Mat3<T> {
    pub fn trans_inv(&self) -> Mat3<T> {
        let (a, b, c, d, e, f, g, h, k) = (self.data[0][0].clone(), self.data[1][0].clone(), self.data[2][0].clone(),
                                          self.data[0][1].clone(), self.data[1][1].clone(), self.data[2][1].clone(),
                                          self.data[0][2].clone(), self.data[1][2].clone(), self.data[2][2].clone());
        
        let det = a * (e*k - f*h) - b * (k*d - f*g) + c * (d*h - e*g);
        
        let ap = (e*k - f*h);
        let bp = -(d*k - f*g);
        let cp = (d*h - e*g);
        let dp = -(b*k - c*h);
        let ep = (a*k - c*g);
        let fp = -(a*h - b*g);
        let gp = (b*f - c*e);
        let hp = -(a*f - c*d);
        let kp = (a*e -b*d);
        
        Mat3 { data: [
            [ap, dp, gp], [bp, ep, hp], [cp, fp, kp]
        ]} / det
    }
}

#[test]
fn test_trans_inv() {
    let mat: Mat3<f64> = Mat3::from_four(Mat4::new(
        [5.0, 2.0, 7.0, 0.0],
        [4.0, 7.0, 6.0, 0.0],
        [2.0, 3.0, 3.0, 0.0],
        [0.0, 0.0, 0.0, 0.0]
    ));
    let operated = mat.trans_inv();
    assert_eq!(operated, Mat3::from_four(Mat4::new(
        [3.0, -0.0, -2.0, 0.0],
        [15.0, 1.0, -11.0, 0.0],
        [-37.0, -2.0, 27.0, 0.0],
        [0.0, 0.0, 0.0, 0.0]
    )));
}