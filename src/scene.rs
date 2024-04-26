use crate::math::{Point2, Vertex};
use crate::{Canvas, Color};

pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub d: f32, // distance from the camera to the screen
}

impl Scene {
    pub fn new(width: usize, height: usize, d: f32) -> Self {
        Scene { width, height, d }
    }
    pub fn viewport_to_canvas(&self, canvas: &Canvas, x: f32, y: f32) -> Point2 {
        Point2::xy(
            (x * canvas.width as f32 / self.width as f32) as i32,
            (y * canvas.height as f32 / self.height as f32) as i32,
        )
    }

    pub fn project_vertex(&self, canvas: &Canvas, v: &Vertex) -> Point2 {
        println!("project_vertex {:?}", v);
        dbg!(self.viewport_to_canvas(canvas, v.x * self.d / v.z, v.y * self.d / v.z))
    }

    pub fn render_instance(&self, canvas: &mut Canvas, inst: &Instance) {
        let mut projected = Vec::new();
        for v in inst.model.vertices.iter() {
            let mut v = *v;
            v.x += inst.position.x;
            v.y += inst.position.y;
            v.z += inst.position.z;
            projected.push(self.project_vertex(canvas, &v));
        }
        for t in &inst.model.triangles {
            self.render_triangle(canvas, t, &projected);
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

    #[allow(dead_code)]
    pub fn render(&self, canvas: &mut Canvas) {
        let blue: u32 = Color::rgb(0, 0, 255).into();
        let red: u32 = Color::rgb(255, 0, 0).into();
        let green: u32 = Color::rgb(0, 255, 0).into();

        // The four "front" vertices
        let v_af = Vertex::new(-2.0, -0.5, 5.0);
        let v_bf = Vertex::new(-2.0, 0.5, 5.0);
        let v_cf = Vertex::new(-1.0, 0.5, 5.0);
        let v_df = Vertex::new(-1.0, -0.5, 5.0);

        // The four "back" vertices
        let v_ab = Vertex::new(-2.0, -0.5, 6.0);
        let v_bb = Vertex::new(-2.0, 0.5, 6.0);
        let v_cb = Vertex::new(-1.0, 0.5, 6.0);
        let v_db = Vertex::new(-1.0, -0.5, 6.0);

        // The front face
        canvas.draw_line(
            &self.project_vertex(canvas, &v_af),
            &self.project_vertex(canvas, &v_bf),
            blue,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, &v_bf),
            &self.project_vertex(canvas, &v_cf),
            blue,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, &v_cf),
            &self.project_vertex(canvas, &v_df),
            blue,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, &v_df),
            &self.project_vertex(canvas, &v_af),
            blue,
        );

        // The back face
        canvas.draw_line(
            &self.project_vertex(canvas, &v_ab),
            &self.project_vertex(canvas, &v_bb),
            red,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, &v_bb),
            &self.project_vertex(canvas, &v_cb),
            red,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, &v_cb),
            &self.project_vertex(canvas, &v_db),
            red,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, &v_db),
            &self.project_vertex(canvas, &v_ab),
            red,
        );

        // The front-to-back edges
        canvas.draw_line(
            &self.project_vertex(canvas, &v_af),
            &self.project_vertex(canvas, &v_ab),
            green,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, &v_bf),
            &self.project_vertex(canvas, &v_bb),
            green,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, &v_cf),
            &self.project_vertex(canvas, &v_cb),
            green,
        );
        canvas.draw_line(
            &self.project_vertex(canvas, &v_df),
            &self.project_vertex(canvas, &v_db),
            green,
        );
    }

    pub fn render2(&self, canvas: &mut Canvas) {
        let blue: u32 = Color::rgb(0, 0, 255).into();
        let red: u32 = Color::rgb(255, 0, 0).into();
        let green: u32 = Color::rgb(0, 255, 0).into();
        let yellow: u32 = Color::rgb(255, 255, 0).into();
        let purple: u32 = Color::rgb(255, 0, 255).into();
        let cyan: u32 = Color::rgb(0, 255, 255).into();

        let vertices = vec![
            Vertex::new(1.0, 1.0, 1.0),
            Vertex::new(-1.0, 1.0, 1.0),
            Vertex::new(-1.0, -1.0, 1.0),
            Vertex::new(1.0, -1.0, 1.0),
            Vertex::new(1.0, 1.0, -1.0),
            Vertex::new(-1.0, 1.0, -1.0),
            Vertex::new(-1.0, -1.0, -1.0),
            Vertex::new(1.0, -1.0, -1.0),
        ];
        let triangles = vec![
            Triangle::new((0, 1, 2), red),
            Triangle::new((0, 2, 3), red),
            Triangle::new((4, 0, 3), green),
            Triangle::new((4, 3, 7), green),
            Triangle::new((5, 4, 7), blue),
            Triangle::new((5, 7, 6), blue),
            Triangle::new((1, 5, 6), yellow),
            Triangle::new((1, 6, 2), yellow),
            Triangle::new((4, 5, 1), purple),
            Triangle::new((4, 1, 0), purple),
            Triangle::new((2, 6, 7), cyan),
            Triangle::new((2, 7, 3), cyan),
        ];

        let cube = Model::new(vertices, triangles);

        let obj1 = Instance::new(&cube, Vertex::new(-1.5, 0., 7.));
        let obj2 = Instance::new(&cube, Vertex::new(1.25, 2., 7.5));

        self.render_instance(canvas, &obj1);
        self.render_instance(canvas, &obj2);

        // for vert in vertices.iter_mut() {
        //     vert.x += -1.5;
        //     vert.z += 7.;
        // }

        // self.render_object(canvas, &vertices, &triangles);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v: (usize, usize, usize),
    pub color: u32,
}

impl Triangle {
    pub fn new(v: (usize, usize, usize), color: u32) -> Self {
        Self { v, color }
    }
}

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl Model {
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<Triangle>) -> Self {
        Self {
            vertices,
            triangles,
        }
    }
}

pub struct Instance<'a> {
    pub model: &'a Model,
    pub position: Vertex,
}

impl<'a> Instance<'a> {
    pub fn new(model: &'a Model, position: Vertex) -> Self {
        Self { model, position }
    }
}
