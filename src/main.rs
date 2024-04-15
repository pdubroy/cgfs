use minifb::{Key, Window, WindowOptions};
use std::cmp::{max, min};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut canvas: Vec<u32> = vec![0; WIDTH * HEIGHT];

    for b in canvas.iter_mut() {
        *b = 0x008080;
    }

    // canvas.draw_line(0, 0, 240, 120, 0xFFFFFF);
    canvas.draw_line(-200, -100, 240, 120, 0xFFFFFF);
    canvas.draw_line(-50, -200, 60, 240, 0xFFFFFF);
    canvas.draw_line(10, 10, 200, 20, 0xFFFFFF);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&canvas, WIDTH, HEIGHT).unwrap();
    }
}

trait Canvas {
    fn set_pixel(&mut self, x: i32, y: i32, color: u32);
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32);
}

impl Canvas for [u32] {
    fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        let hw = WIDTH / 2;
        let hh = HEIGHT / 2;
        let x_norm = x + hw as i32;
        let y_norm = hh as i32 - y;
        self[y_norm as usize * WIDTH + x_norm as usize] = color;
    }

    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        if (x1 - x0).abs() > (y1 - y0).abs() {
            let x0 = min(x0, x1);
            let x1 = max(x0, x1);
            let ys = interpolate(x0, y0 as f32, x1, y1 as f32);
            for x in x0..x1 {
                self.set_pixel(x, ys[(x - x0) as usize] as i32, color);
            }
        } else {
            let y0 = min(y0, y1);
            let y1 = max(y0, y1);
            let xs = interpolate(y0, x0 as f32, y1, x1 as f32);
            for y in y0..y1 {
                self.set_pixel(xs[(y - y0) as usize] as i32, y, color);
            }
        }
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
    for _ in i0..i1 {
        values.push(d);
        d += a;
    }
    values
}
