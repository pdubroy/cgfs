use minifb::{Key, Window, WindowOptions};
use std::fmt;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    canvas.fill(0x008080);
    canvas.draw_wireframe(&Point::new(-200, -250), &Point::new(200, 50), &Point::new(20, 250), 0xFFFFFF);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&canvas.data, WIDTH, HEIGHT).unwrap();
    }
}

#[derive(Debug)]
struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Canvas {
    pub data: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    fn fill(&mut self, color: u32) {
        for b in self.data.iter_mut() {
            *b = color;
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        let hw = self.width / 2;
        let hh = self.height / 2;
        let x_norm = x + hw as i32;
        let y_norm = hh as i32 - y;
        self.data[y_norm as usize * self.width + x_norm as usize] = color;
    }

    // fn get_pixel(&mut self, x: i32, y: i32) -> u32 {
    //     let hw = self.width / 2;
    //     let hh = self.height / 2;
    //     let x_norm = x + hw as i32;
    //     let y_norm = hh as i32 - y;
    //     self.data[y_norm as usize * self.width + x_norm as usize]
    // }

    fn draw_line(&mut self, p0: &Point, p1: &Point, color: u32) {
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            let (p0, p1) = if p0.x > p1.x {
                (p1, p0)
            } else {
                (p0, p1)
            };
            let ys = interpolate(p0.x, p0.y as f32, p1.x, p1.y as f32);
            for x in p0.x..p1.x + 1 {
                self.set_pixel(x, ys[(x - p0.x) as usize] as i32, color);
            }
        } else {
            let (p0, p1) = if p0.y > p1.y {
                (p1, p0)
            } else {
                (p0, p1)
            };
            println!("{:?} {:?}", p0, p1);
            let xs = interpolate(p0.y, p0.x as f32, p1.y, p1.x as f32);
            println!("{:?}", xs);
            for y in p0.y..p1.y + 1 {
                self.set_pixel(xs[(y - p0.y) as usize] as i32, y, color);
            }
        }
    }

    fn draw_wireframe(&mut self, p0: &Point, p1: &Point, p2: &Point, color: u32) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p0, color);
        // self.draw_line(x0, y0, x2, y2, color);
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            let pixels: Vec<_> = (0..self.width).map(|x| {
                let px = self.data[y * self.width + x];
                if px == 0 { "-"} else { "X" }
            }).collect();
            s.push_str(&pixels.join(" "));
            if y + 1 != self.height { s.push('\n'); }
        }
        write!(f, "{}", s)
    }
}

// Convention: values of the independent variable i are always integers, as
// they represent pixels, while the values of the dependent variable d
// are always floating point values, as they represent values of a generic
// linear function.
fn interpolate(i0: i32, d0: f32, i1: i32, d1: f32) -> Vec<f32> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lines() {
        let mut canvas = Canvas::new(5, 5);
        canvas.draw_line(&Point::new(-2, 2), &Point::new(0, -2), 0xFFFFFF);
        assert_eq!(canvas.to_string(), "
X - - - -
- X - - -
- X - - -
- - X - -
- - X - -
        ".trim());
    }
}
