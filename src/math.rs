#[cfg(test)]
use core::f32::consts::PI;

#[derive(Debug)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
    pub h: f32,
}

impl Point2 {
    pub fn xy(x: i32, y: i32) -> Point2 {
        Point2 { x, y, h: 1.0 }
    }

    #[allow(dead_code)]
    pub fn xyh(x: i32, y: i32, h: f32) -> Point2 {
        Point2 { x, y, h }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { x, y, z }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    pub fn from_translation(translation: Vertex) -> Self {
        Self {
            x: [1., 0., 0., 0.],
            y: [0., 1., 0., 0.],
            z: [0., 0., 1., 0.],
            w: [translation.x, translation.y, translation.z, 1.],
        }
    }

    // pub fn from_rotation_x(angle: f32) -> Self {
    //     let cos = angle.cos();
    //     let sin = angle.sin();
    //     Self {
    //         x: [1., 0., 0., 0.],
    //         y: [0., cos, -sin, 0.],
    //         z: [0., sin, cos, 0.],
    //         w: [0., 0., 0., 1.],
    //     }
    // }

    // pub fn from_rotation_y(angle: f32) -> Self {
    //     let cos = angle.cos();
    //     let sin = angle.sin();
    //     Self {
    //         x: [cos, 0., sin, 0.],
    //         y: [0., 1., 0., 0.],
    //         z: [-sin, 0., cos, 0.],
    //         w: [0., 0., 0., 1.],
    //     }
    // }

    pub fn from_rotation_z(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x: [cos, -sin, 0., 0.],
            y: [sin, cos, 0., 0.],
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

    pub fn mul_m(&self, other: &Matrix4) -> Matrix4 {
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
}

#[test]
fn test_matrix_mul() {
    assert_eq!(
        Matrix4::from_rotation_z(PI / 2.).mul_m(&Matrix4::from_rotation_z(-PI / 2.)),
        Matrix4::identity()
    );
    assert_eq!(
        Matrix4::from_translation(Vertex::new(1., 2., 3.))
            .mul_m(&Matrix4::from_translation(Vertex::new(-1., -2., -3.))),
        Matrix4::identity()
    );
    assert_eq!(
        Matrix4::identity().mul_v(Vector4::new(1., 2., 3., 4.)),
        Vector4::new(1., 2., 3., 4.)
    );
    assert_eq!(
        Matrix4::from_translation(Vertex::new(1., 2., 3.)).mul_v(Vector4::new(0., 0., 0., 1.)),
        Vector4::new(1., 2., 3., 1.)
    );
}

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

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub scale: f32,
    pub rotation: f32,
    pub translation: Vertex,
}

impl Transform {
    pub fn apply(&self, vertex: Vertex) -> Vertex {
        vertex
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            scale: 1.,
            rotation: 0.,
            translation: Vertex::default(),
        }
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
