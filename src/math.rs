#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub h: f32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y, h: 1.0 }
    }
    pub fn new3(x: i32, y: i32, h: f32) -> Point {
        Point { x, y, h }
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
