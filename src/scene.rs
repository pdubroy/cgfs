use std::f32::consts::PI;
use std::rc::Rc;

use crate::math::*;
use crate::{Canvas, Color};

const PROJECTION_PLANE_Z: f32 = 1.;

pub struct Camera {
    pub position: Point3,
    pub orientation: Matrix4,
}

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub models: Vec<Model>,
    pub instances: Vec<Instance>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(width: usize, height: usize) -> Self {
        let camera = Camera {
            position: Point3::new(-3., 1., 2.),
            orientation: Matrix4::from_rotation_y(PI / 6.),
        };

        Scene {
            width,
            height,
            models: Vec::new(),
            instances: Vec::new(),
            camera,
        }
    }

    pub fn viewport_to_canvas(&self, canvas: &Canvas, x: f32, y: f32) -> Point2 {
        Point2::new(
            (x * canvas.width as f32 / self.width as f32) as i32,
            (y * canvas.height as f32 / self.height as f32) as i32,
        )
    }

    pub fn project_vertex(&self, canvas: &Canvas, v: Point3) -> Point2 {
        // println!("project_vertex {:?}", v);
        self.viewport_to_canvas(
            canvas,
            v.x * PROJECTION_PLANE_Z / v.z,
            v.y * PROJECTION_PLANE_Z / v.z,
        )
    }

    // From Listing 10-5.
    pub fn render_model(&self, model: &Model, transform: Matrix4, canvas: &mut Canvas) {
        let mut projected = Vec::new();
        for v in &model.vertices {
            projected.push(self.project_vertex(canvas, transform * *v))
        }
        for t in &model.triangles {
            self.render_triangle(canvas, t, &projected);
        }
    }

    // From Listing 10-5.
    #[allow(dead_code)]
    pub fn render(&self, canvas: &mut Canvas) {
        canvas.fill(0);
        let m_camera =
            self.camera.orientation.transpose() * Matrix4::from_translation(-self.camera.position);
        for inst in &self.instances {
            let m = m_camera * inst.transform;
            self.render_model(inst.model.as_ref(), m, canvas);
        }
    }

    pub fn render_triangle(&self, canvas: &mut Canvas, triangle: &Triangle, projected: &[Point2]) {
        println!("{:?}", triangle);
        canvas.draw_wireframe(
            &projected[triangle.v.0],
            &projected[triangle.v.1],
            &projected[triangle.v.2],
            triangle.color,
        );
    }

    pub fn render1(&self, canvas: &mut Canvas) {
        let blue: u32 = Color::rgb(0, 0, 255).into();
        let red: u32 = Color::rgb(255, 0, 0).into();
        let green: u32 = Color::rgb(0, 255, 0).into();

        // The four "front" vertices
        let v_af = Point3::new(-2.0, -0.5, 5.0);
        let v_bf = Point3::new(-2.0, 0.5, 5.0);
        let v_cf = Point3::new(-1.0, 0.5, 5.0);
        let v_df = Point3::new(-1.0, -0.5, 5.0);

        // The four "back" vertices
        let v_ab = Point3::new(-2.0, -0.5, 6.0);
        let v_bb = Point3::new(-2.0, 0.5, 6.0);
        let v_cb = Point3::new(-1.0, 0.5, 6.0);
        let v_db = Point3::new(-1.0, -0.5, 6.0);

        // The front face
        canvas.draw_line(
            &self.project_vertex(canvas, v_af),
            &self.project_vertex(canvas, v_bf),
            blue,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, v_bf),
            &self.project_vertex(canvas, v_cf),
            blue,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, v_cf),
            &self.project_vertex(canvas, v_df),
            blue,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, v_df),
            &self.project_vertex(canvas, v_af),
            blue,
        );

        // The back face
        canvas.draw_line(
            &self.project_vertex(canvas, v_ab),
            &self.project_vertex(canvas, v_bb),
            red,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, v_bb),
            &self.project_vertex(canvas, v_cb),
            red,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, v_cb),
            &self.project_vertex(canvas, v_db),
            red,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, v_db),
            &self.project_vertex(canvas, v_ab),
            red,
        );

        // The front-to-back edges
        canvas.draw_line(
            &self.project_vertex(canvas, v_af),
            &self.project_vertex(canvas, v_ab),
            green,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, v_bf),
            &self.project_vertex(canvas, v_bb),
            green,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, v_cf),
            &self.project_vertex(canvas, v_cb),
            green,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, v_df),
            &self.project_vertex(canvas, v_db),
            green,
        );
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v: (usize, usize, usize),
    pub color: u32,
}

impl Triangle {
    pub fn new(v: (usize, usize, usize), color: impl Into<u32>) -> Self {
        Self {
            v,
            color: color.into(),
        }
    }
}

pub struct Model {
    pub vertices: Vec<Point3>,
    pub triangles: Vec<Triangle>,
}

impl Model {
    pub fn new(vertices: Vec<Point3>, triangles: Vec<Triangle>) -> Self {
        Self {
            vertices,
            triangles,
        }
    }

    pub fn cube() -> Self {
        let vertices = vec![
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(-1.0, 1.0, 1.0),
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(1.0, -1.0, 1.0),
            Point3::new(1.0, 1.0, -1.0),
            Point3::new(-1.0, 1.0, -1.0),
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(1.0, -1.0, -1.0),
        ];
        let triangles = vec![
            Triangle::new((0, 1, 2), Color::red()),
            Triangle::new((0, 2, 3), Color::red()),
            Triangle::new((4, 0, 3), Color::green()),
            Triangle::new((4, 3, 7), Color::green()),
            Triangle::new((5, 4, 7), Color::blue()),
            Triangle::new((5, 7, 6), Color::blue()),
            Triangle::new((1, 5, 6), Color::yellow()),
            Triangle::new((1, 6, 2), Color::yellow()),
            Triangle::new((4, 5, 1), Color::purple()),
            Triangle::new((4, 1, 0), Color::purple()),
            Triangle::new((2, 6, 7), Color::cyan()),
            Triangle::new((2, 7, 3), Color::cyan()),
        ];

        Self::new(vertices, triangles)
    }
}

pub struct Instance {
    pub model: Rc<Model>,
    pub transform: Matrix4,
}

impl Instance {
    pub fn new(model: Rc<Model>, position: Point3, orientation: Matrix4, scale: f32) -> Self {
        let transform =
            Matrix4::from_translation(position) * orientation * Matrix4::from_scale(scale);
        Self { model, transform }
    }
}

impl Color {
    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, h: 1.0 }
    }

    #[allow(dead_code)]
    fn mul(&mut self, h: f32) {
        self.h *= h;
    }

    pub fn blue() -> Color {
        Color::rgb(0, 0, 255)
    }
    pub fn red() -> Color {
        Color::rgb(255, 0, 0)
    }
    pub fn green() -> Color {
        Color::rgb(0, 255, 0)
    }
    pub fn yellow() -> Color {
        Color::rgb(255, 255, 0)
    }
    pub fn purple() -> Color {
        Color::rgb(255, 0, 255)
    }
    pub fn cyan() -> Color {
        Color::rgb(0, 255, 255)
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
