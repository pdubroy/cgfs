use minifb::{Key, Window, WindowOptions};
use std::fmt;
use std::thread::sleep;
use std::time::Duration;

mod math;
use math::{interpolate, Point2};

mod scene;
use scene::Scene;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    // let mut p1 = Point2::xyh(-200, -250, 0.3);
    // let mut p2 = Point2::xyh(200, 50, 0.1);
    // let mut p3 = Point2::xyh(20, 250, 1.0);

    // let mut v1 = Point2::xy(2, 1);
    // let mut v2 = Point2::xy(1, 2);
    // let mut v3 = Point2::xy(1, 3);

    let scene = Scene::new(1, 1, 1.0);
    // scene.render(&mut canvas);
    scene.render2(&mut canvas);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // canvas.fill(0xFFFFFF);
        // canvas.draw_shaded_triangle(&p1, &p2, &p3, 0x00FF00);
        // canvas.draw_wireframe(&p1, &p2, &p3, 0);
        window
            .update_with_buffer(&canvas.data, WIDTH, HEIGHT)
            .unwrap();

        // update_point_and_velocity(&mut p1, &mut v1);
        // update_point_and_velocity(&mut p2, &mut v2);
        // update_point_and_velocity(&mut p3, &mut v3);

        sleep(Duration::from_millis(30));
    }
}

#[allow(dead_code)]
fn update_point_and_velocity(p: &mut Point2, v: &mut Point2) {
    let hw = WIDTH / 2;
    let hh = HEIGHT / 2;

    p.x += v.x;
    if p.x.unsigned_abs() as usize >= hw {
        v.x *= -1;
        p.x += v.x;
    }
    p.y += v.y;
    if p.y.unsigned_abs() as usize >= hh {
        v.y *= -1;
        p.y += v.y;
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    h: f32,
}

impl Color {
    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, h: 1.0 }
    }

    #[allow(dead_code)]
    fn mul(&mut self, h: f32) {
        self.h *= h;
    }
}

impl From<Color> for u32 {
    fn from(c: Color) -> Self {
        let r = ((c.r as f32 * c.h) as u32) << 16;
        let g = ((c.g as f32 * c.h) as u32) << 8;
        let b = (c.b as f32 * c.h) as u32;
        r | g | b
    }
}

impl From<u32> for Color {
    fn from(c: u32) -> Self {
        let r = (c >> 16) as u8;
        let g = (c >> 8) as u8;
        let b = c as u8;
        Color::rgb(r, g, b)
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

    #[allow(dead_code)]
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
        println!("x: {}, y: {}", x, y);
        println!("x_norm: {}, y_norm: {}", x_norm, y_norm);
        println!("width: {}, height: {}", self.width, self.height);
        self.data[y_norm as usize * self.width + x_norm as usize] = color;
    }

    // fn get_pixel(&mut self, x: i32, y: i32) -> u32 {
    //     let hw = self.width / 2;
    //     let hh = self.height / 2;
    //     let x_norm = x + hw as i32;
    //     let y_norm = hh as i32 - y;
    //     self.data[y_norm as usize * self.width + x_norm as usize]
    // }

    #[allow(dead_code)]
    fn draw_line(&mut self, p0: &Point2, p1: &Point2, color: u32) {
        println!("draw_line p0: {:?}, p1: {:?}", p0, p1);
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            let (p0, p1) = if p0.x > p1.x { (p1, p0) } else { (p0, p1) };
            let ys = interpolate(p0.x, p0.y as f32, p1.x, p1.y as f32);
            for x in p0.x..p1.x + 1 {
                self.set_pixel(x, ys[(x - p0.x) as usize] as i32, color);
            }
        } else {
            let (p0, p1) = if p0.y > p1.y { (p1, p0) } else { (p0, p1) };
            let xs = interpolate(p0.y, p0.x as f32, p1.y, p1.x as f32);
            for y in p0.y..p1.y + 1 {
                self.set_pixel(xs[(y - p0.y) as usize] as i32, y, color);
            }
        }
    }

    #[allow(dead_code)]
    fn draw_wireframe(&mut self, p0: &Point2, p1: &Point2, p2: &Point2, color: u32) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p0, color);
    }

    #[allow(dead_code)]
    fn draw_filled_triangle(&mut self, p0: &Point2, p1: &Point2, p2: &Point2, color: u32) {
        let mut p0 = p0;
        let mut p1 = p1;
        let mut p2 = p2;

        // Sort the points so that y0 <= y1 <= y2
        if p1.y < p0.y {
            std::mem::swap(&mut p1, &mut p0);
        }
        if p2.y < p0.y {
            std::mem::swap(&mut p2, &mut p0);
        }
        if p2.y < p1.y {
            std::mem::swap(&mut p2, &mut p1);
        }

        // Compute the x coordinates of the triangle edges
        let mut x01 = interpolate(p0.y, p0.x as f32, p1.y, p1.x as f32);
        let x12 = interpolate(p1.y, p1.x as f32, p2.y, p2.x as f32);
        let x02 = interpolate(p0.y, p0.x as f32, p2.y, p2.x as f32);

        // Concatenate the short sides
        x01.pop();
        let x012 = [x01, x12].concat();

        // Determine which is left and which is right
        let m = x012.len() / 2;
        let mut x_left = x02;
        let mut x_right = x012;
        if x_right[m] < x_left[m] {
            std::mem::swap(&mut x_left, &mut x_right);
        }

        // Draw the horizontal segments
        for y in p0.y..p2.y + 1 {
            let x_start = x_left[(y - p0.y) as usize] as i32;
            let x_end = x_right[(y - p0.y) as usize] as i32;
            for x in x_start..x_end + 1 {
                self.set_pixel(x, y, color);
            }
        }
    }

    #[allow(dead_code)]
    fn draw_shaded_triangle(&mut self, p0: &Point2, p1: &Point2, p2: &Point2, color: u32) {
        let mut p0 = p0;
        let mut p1 = p1;
        let mut p2 = p2;

        // Sort the points so that y0 <= y1 <= y2
        if p1.y < p0.y {
            std::mem::swap(&mut p1, &mut p0);
        }
        if p2.y < p0.y {
            std::mem::swap(&mut p2, &mut p0);
        }
        if p2.y < p1.y {
            std::mem::swap(&mut p2, &mut p1);
        }

        // Compute the x coordinates and h values of the triangle edges
        let mut x01 = interpolate(p0.y, p0.x as f32, p1.y, p1.x as f32);
        let mut h01 = interpolate(p0.y, p0.h, p1.y, p1.h);

        let x12 = interpolate(p1.y, p1.x as f32, p2.y, p2.x as f32);
        let h12 = interpolate(p1.y, p1.h, p2.y, p2.h);

        let x02 = interpolate(p0.y, p0.x as f32, p2.y, p2.x as f32);
        let h02 = interpolate(p0.y, p0.h, p2.y, p2.h);

        // Concatenate the short sides
        x01.pop();
        let x012 = [x01, x12].concat();

        h01.pop();
        let h012 = [h01, h12].concat();

        // Determine which is left and which is right
        let m = x012.len() / 2;
        let mut x_left = x02;
        let mut x_right = x012;
        let mut h_left = h02;
        let mut h_right = h012;

        if x_right[m] < x_left[m] {
            std::mem::swap(&mut x_left, &mut x_right);
            std::mem::swap(&mut h_left, &mut h_right);
        }

        // Draw the horizontal segments
        for y in p0.y..p2.y + 1 {
            let x_l = x_left[(y - p0.y) as usize] as i32;
            let x_r = x_right[(y - p0.y) as usize] as i32;

            let h_segment = interpolate(
                x_l,
                h_left[(y - p0.y) as usize],
                x_r,
                h_right[(y - p0.y) as usize],
            );
            for x in x_l..x_r + 1 {
                let mut shaded_color: Color = color.into();
                let h = h_segment[(x - x_l) as usize];
                shaded_color.mul(h);
                self.set_pixel(x, y, shaded_color.into());
            }
        }
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            let pixels: Vec<_> = (0..self.width)
                .map(|x| {
                    let px = self.data[y * self.width + x];
                    if px == 0 {
                        "-"
                    } else {
                        "X"
                    }
                })
                .collect();
            s.push_str(&pixels.join(" "));
            if y + 1 != self.height {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn canvas_with_filled_triangle(p1: &Point2, p2: &Point2, p3: &Point2) -> Canvas {
        let mut canvas = Canvas::new(3, 3);
        canvas.draw_filled_triangle(&p1, &p2, &p3, 0xFFFFFF);
        canvas
    }

    #[test]
    fn test_draw_line() {
        let mut canvas = Canvas::new(5, 5);
        canvas.draw_line(&Point2::xy(-2, 2), &Point2::xy(0, -2), 0xFFFFFF);
        assert_eq!(
            canvas.to_string(),
            "
X - - - -
- X - - -
- X - - -
- - X - -
- - X - -
        "
            .trim()
        );
    }

    #[test]
    fn test_draw_filled_triangle() {
        let canvas =
            canvas_with_filled_triangle(&Point2::xy(-1, 1), &Point2::xy(0, 0), &Point2::xy(-1, -1));
        assert_eq!(
            canvas.to_string(),
            "
X - -
X X -
X - -
        "
            .trim()
        );

        let p1 = Point2::xy(-1, 1);
        let p2 = Point2::xy(1, 1);
        let p3 = Point2::xy(-1, -1);
        let canvas = canvas_with_filled_triangle(&p1, &p2, &p3);
        assert_eq!(
            canvas.to_string(),
            "
X X X
X X -
X - -
        "
            .trim()
        );

        assert_eq!(
            canvas_with_filled_triangle(&p1, &p2, &p3).to_string(),
            canvas_with_filled_triangle(&p1, &p3, &p2).to_string()
        );
        assert_eq!(
            canvas_with_filled_triangle(&p1, &p2, &p3).to_string(),
            canvas_with_filled_triangle(&p3, &p1, &p2).to_string()
        );
        assert_eq!(
            canvas_with_filled_triangle(&p1, &p2, &p3).to_string(),
            canvas_with_filled_triangle(&p3, &p2, &p1).to_string()
        );

        let canvas = canvas_with_filled_triangle(&p1, &p1, &p1);
        assert_eq!(
            canvas.to_string(),
            "
X - -
- - -
- - -
        "
            .trim()
        );
    }

    #[test]
    #[should_panic]
    fn test_filled_triangle_corner_cases() {
        let canvas =
            canvas_with_filled_triangle(&Point2::xy(-1, 1), &Point2::xy(0, 1), &Point2::xy(1, 1));
        // Due to the special case in `interpolate`, this will not produce the expected result.
        assert_eq!(
            canvas.to_string(),
            "
X X X
- - -
- - -
        "
            .trim()
        );
    }
}
