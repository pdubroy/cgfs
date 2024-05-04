use minifb::{Key, Window, WindowOptions};
use std::fmt;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

mod math;
use math::*;

mod scene;
use scene::*;

use std::f32::consts::PI;

const WIDTH: usize = 600;
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

    let mut scene = Scene::new(1, 1);
    init_cube_scene(&mut scene);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Left) {
            scene.camera.position.x -= 0.1;
        } else if window.is_key_down(Key::Right) {
            scene.camera.position.x += 0.1;
        }

        scene.render(&mut canvas);

        window
            .update_with_buffer(&canvas.data, WIDTH, HEIGHT)
            .unwrap();
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

pub struct Canvas {
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

    pub fn fill(&mut self, color: u32) {
        for b in self.data.iter_mut() {
            *b = color;
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        let hw = self.width / 2;
        let hh = self.height / 2;
        let x_norm = x + hw as i32;
        let y_norm = hh as i32 - y;
        self.data[y_norm as usize * self.width + x_norm as usize] = color;
    }

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

pub fn init_cube_scene(scene: &mut Scene) {
    let cube = Rc::new(Model::cube());

    let obj1 = Instance::new(
        Rc::clone(&cube),
        Point3::new(-1.5, 0., 7.),
        Matrix4::identity(),
        0.75,
    );
    let obj2 = Instance::new(
        Rc::clone(&cube),
        Point3::new(1.25, 2.5, 7.5),
        Matrix4::from_rotation_y(195. * PI / 2.),
        1.0,
    );
    scene.instances.push(obj1);
    scene.instances.push(obj2);
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
        canvas.draw_line(&Point2::new(-2, 2), &Point2::new(0, -2), 0xFFFFFF);
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
        let canvas = canvas_with_filled_triangle(
            &Point2::new(-1, 1),
            &Point2::new(0, 0),
            &Point2::new(-1, -1),
        );
        assert_eq!(
            canvas.to_string(),
            "
X - -
X X -
X - -
        "
            .trim()
        );

        let p1 = Point2::new(-1, 1);
        let p2 = Point2::new(1, 1);
        let p3 = Point2::new(-1, -1);
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
        let canvas = canvas_with_filled_triangle(
            &Point2::new(-1, 1),
            &Point2::new(0, 1),
            &Point2::new(1, 1),
        );
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
