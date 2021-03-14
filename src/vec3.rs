use std::ops;

use fast_inv_sqrt::InvSqrt64;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn add(a: Vec3, b: Vec3) -> Self {
        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    pub fn sub(a: Vec3, b: Vec3) -> Self {
        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }

    pub fn normalize(&self) -> Self {
        let v = InvSqrt64::inv_sqrt64(self.len_squared());
        v * *self
    }

    pub fn len(&self) -> f64 {
        let sqrt = self.x * self.x + self.y * self.y + self.z * self.z;
        sqrt.sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(a: Vec3, b: Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Self {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn negate(v: Vec3) -> Self {
        Vec3 {
            x: -v.x,
            y: -v.y,
            z: -v.z,
        }
    }
}

fn nearly_equal(a: f64, b: f64) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < (f64::EPSILON * f64::MIN_POSITIVE)
    } else {
        // Use relative error.
        (diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::add(self, v)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::sub(self, v)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f64) -> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[test]
fn it_subs() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(1.0, 2.0, 3.0);

    let sub_1 = a - b;

    let c = Vec3::new(0.0, 0.0, 0.0);

    let result_1 = c;
    assert_eq!(result_1.x, sub_1.x);
    assert_eq!(result_1.y, sub_1.y);
    assert_eq!(result_1.z, sub_1.z);

    let d = Vec3::new(4.0, 5.5, 6.0);
    let sub_2 = a - d;

    let result_2 = Vec3::new(-3.0, -3.5, -3.0);
    assert_eq!(result_2.x, sub_2.x);
    assert_eq!(result_2.y, sub_2.y);
    assert_eq!(result_2.z, sub_2.z);
}

#[test]
fn it_mults() {
    let a = Vec3::new(1.0, 2.0, 3.0);

    let c = 4.32;
    let mult_1 = a * c;
    let mult_2 = c * a;

    let result_1 = Vec3::new(c * a.x, c * a.y, c * a.z);
    let result_2 = Vec3::new(4.32, 8.64, 12.96);

    assert_eq!(result_1.x, mult_1.x);
    assert_eq!(result_1.y, mult_1.y);
    assert_eq!(result_1.z, mult_1.z);

    assert_eq!(result_1.x, mult_2.x);
    assert_eq!(result_1.y, mult_2.y);
    assert_eq!(result_1.z, mult_2.z);

    assert_eq!(result_2.x, mult_1.x);
    assert_eq!(result_2.y, mult_1.y);
    assert_eq!(result_2.z, mult_1.z);

    assert_eq!(result_2.x, mult_2.x);
    assert_eq!(result_2.y, mult_2.y);
    assert_eq!(result_2.z, mult_2.z);
}

#[test]
fn it_cross() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(1.0, 2.0, 3.0);

    let c = Vec3::cross(a, b);

    let result_1 = Vec3::zero();

    assert_eq!(result_1.x, c.x);
    assert_eq!(result_1.y, c.y);
    assert_eq!(result_1.z, c.z);

    let d = Vec3::new(1.0, 2.0, 3.0);
    let e = Vec3::new(3.0, 2.0, 1.0);

    let f = Vec3::cross(d, e);
    let result_2 = Vec3::new(-4.0, 8.0, -4.0);

    assert_eq!(result_2.x, f.x);
    assert_eq!(result_2.y, f.y);
    assert_eq!(result_2.z, f.z);
}

#[test]
fn it_dot() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(1.0, 2.0, 3.0);

    let c = Vec3::new(3.0, 2.0, 1.0);
    let d = Vec3::new(5.0, 8.0, 0.0);

    let result_1 = 14.0;
    let result_2 = a.len_squared();
    let result_3 = a.x * b.x + a.y * b.y + a.z * b.z;

    let result_4 = 31.0;
    let result_5 = c.x * d.x + c.y * d.y + c.z * d.z;

    let dot_1 = Vec3::dot(a, b);
    let dot_2 = Vec3::dot(c, d);

    assert!(nearly_equal(dot_1, result_1));
    assert!(nearly_equal(dot_1, result_2));
    assert!(nearly_equal(dot_1, result_3));

    assert!(nearly_equal(dot_2, result_4));
    assert!(nearly_equal(dot_2, result_5));
}
