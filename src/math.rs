#[cfg(test)]
use core::f32::consts::PI;

use std::ops::*;

#[derive(Debug)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

/// A point in 2D space.
impl Point2 {
    pub fn new(x: i32, y: i32) -> Point2 {
        Point2 { x, y }
    }
}

/// A point in 3D space.
#[derive(Clone, Copy, Debug)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }
}

impl Default for Point3 {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Mul<f32> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: f32) -> Point3 {
        Point3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Point3 {
        self * -1.
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    pub x: [f32; 4],
    pub y: [f32; 4],
    pub z: [f32; 4],
    pub w: [f32; 4],
}

#[allow(dead_code)]
impl Matrix4 {
    pub fn from_cols(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }

    pub fn from_scale(scale: f32) -> Self {
        Self {
            x: [scale, 0., 0., 0.],
            y: [0., scale, 0., 0.],
            z: [0., 0., scale, 0.],
            w: [0., 0., 0., 1.],
        }
    }

    pub fn from_translation(translation: Point3) -> Self {
        Self {
            x: [1., 0., 0., 0.],
            y: [0., 1., 0., 0.],
            z: [0., 0., 1., 0.],
            w: [translation.x, translation.y, translation.z, 1.],
        }
    }

    pub fn from_rotation_x(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: [1., 0., 0., 0.],
            y: [0., cos, sin, 0.],
            z: [0., -sin, cos, 0.],
            w: [0., 0., 0., 1.],
        }
    }

    pub fn from_rotation_y(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: [cos, 0., -sin, 0.],
            y: [0., 1., 0., 0.],
            z: [sin, 0., cos, 0.],
            w: [0., 0., 0., 1.],
        }
    }

    pub fn from_rotation_z(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: [cos, sin, 0., 0.],
            y: [-sin, cos, 0., 0.],
            z: [0., 0., 1., 0.],
            w: [0., 0., 0., 1.],
        }
    }

    pub fn zero() -> Self {
        Self {
            x: [0., 0., 0., 0.],
            y: [0., 0., 0., 0.],
            z: [0., 0., 0., 0.],
            w: [0., 0., 0., 0.],
        }
    }

    pub fn identity() -> Self {
        Self {
            x: [1., 0., 0., 0.],
            y: [0., 1., 0., 0.],
            z: [0., 0., 1., 0.],
            w: [0., 0., 0., 1.],
        }
    }

    pub fn mul_m(&self, other: Matrix4) -> Matrix4 {
        Matrix4::from_cols(
            self.mul_v(other.x.into()),
            self.mul_v(other.y.into()),
            self.mul_v(other.z.into()),
            self.mul_v(other.w.into()),
        )
    }

    pub fn mul_v(&self, v: Vector4) -> Vector4 {
        Vector4::new(
            self.x[0] * v.x + self.y[0] * v.y + self.z[0] * v.z + self.w[0] * v.w,
            self.x[1] * v.x + self.y[1] * v.y + self.z[1] * v.z + self.w[1] * v.w,
            self.x[2] * v.x + self.y[2] * v.y + self.z[2] * v.z + self.w[2] * v.w,
            self.x[3] * v.x + self.y[3] * v.y + self.z[3] * v.z + self.w[3] * v.w,
        )
    }

    pub fn transpose(&self) -> Matrix4 {
        Matrix4::from_cols(
            Vector4::new(self.x[0], self.y[0], self.z[0], self.w[0]),
            Vector4::new(self.x[1], self.y[1], self.z[1], self.w[1]),
            Vector4::new(self.x[2], self.y[2], self.z[2], self.w[2]),
            Vector4::new(self.x[3], self.y[3], self.z[3], self.w[3]),
        )
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        self.mul_m(other)
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, other: Vector4) -> Vector4 {
        self.mul_v(other)
    }
}

impl Mul<Point3> for Matrix4 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        let v = self * Vector4::new(other.x, other.y, other.z, 1.);
        Point3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

/// A vector in 3D space with homogeneous coordinates.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Vector4> for [f32; 4] {
    fn from(v: Vector4) -> Self {
        [v.x, v.y, v.z, v.w]
    }
}

impl From<[f32; 4]> for Vector4 {
    fn from(v: [f32; 4]) -> Self {
        Vector4::new(v[0], v[1], v[2], v[3])
    }
}

// Convention: values of the independent variable i are always integers, as
// they represent pixels, while the values of the dependent variable d
// are always floating point values, as they represent values of a generic
// linear function.
pub fn interpolate(i0: i32, d0: f32, i1: i32, d1: f32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0];
    }
    let mut values = Vec::new();
    let a = (d1 - d0) / (i1 as f32 - i0 as f32);
    let mut d = d0;
    for _ in i0..i1 + 1 {
        values.push(d);
        d += a;
    }
    values
}

#[test]
fn test_matrix_mul_m() {
    assert_eq!(
        Matrix4::from_rotation_z(PI / 2.) * Matrix4::from_rotation_z(-PI / 2.),
        Matrix4::identity()
    );
    assert_eq!(
        Matrix4::from_translation(Point3::new(1., 2., 3.))
            * Matrix4::from_translation(Point3::new(-1., -2., -3.)),
        Matrix4::identity()
    );
    // Rotation after translation shouldn't touch the translation part
    let m = Matrix4::from_translation(Point3::new(-3., 1., 2.)) * Matrix4::from_rotation_y(PI / 6.);
    assert_eq!(m.w, Matrix4::from_translation(Point3::new(-3., 1., 2.)).w)
}

#[test]
fn test_matrix_mul_v() {
    assert_eq!(
        Matrix4::identity() * Vector4::new(1., 2., 3., 4.),
        Vector4::new(1., 2., 3., 4.)
    );
    assert_eq!(
        Matrix4::from_translation(Point3::new(1., 2., 3.)) * Vector4::new(0., 0., 0., 1.),
        Vector4::new(1., 2., 3., 1.)
    );
}
