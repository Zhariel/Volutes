use nannou::prelude::*;
use crate::settings::{Settings};
use crate::shapes::{Triangle, Mesh};
use vek::Vec2;

pub struct Renderer {
    pub rays: Rays,
    pub settings: Settings,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            rays: Rays,
            settings: Settings::load(),
        }
    }

    pub fn render(&self, draw: &Draw, mesh: &Mesh) {
        match self.settings.rendering.as_str() {
            "casting" => self.rays.casting(draw, mesh),
            "marching" => self.rays.marching(draw, mesh),
            "tracing" => self.rays.tracing(draw, mesh),
            "wireframe" => self.rays.wireframe(draw, mesh, self.settings.vertex_size),
            "vertex" => self.rays.vertex(draw, mesh, self.settings.vertex_size),
            _ => self.rays.casting(draw, mesh),
        }
    }
}

pub struct Rays;

impl Rays {
    fn vertex(&self, draw: &Draw, mesh: &Mesh, size: f32) {
        for vert in &mesh.projected_vertices {
            draw.rect()
                .color(BLACK)
                .w(size)
                .h(size)
                .x_y(vert.x as f32, vert.y as f32);
        }
    }

    fn wireframe(&self, draw: &Draw, mesh: &Mesh, size: f32) {
        fn draw_line(draw: &Draw, size: f32, w: &Vec2<f64>, q: &Vec2<f64>){
            draw.line()
                .start(pt2(w.x as f32, w.y as f32))
                .end(pt2(q.x as f32, q.y as f32))
                .weight(size)
                .color(BLACK);
        }

        for tri in &mesh.triangles {
            let vec_a: Vec2<f64> = mesh.projected_vertices[tri.a-1];
            let vec_b: Vec2<f64> = mesh.projected_vertices[tri.b-1];
            let vec_c: Vec2<f64> = mesh.projected_vertices[tri.c-1];

            draw_line(draw, size, &vec_a, &vec_b);
            draw_line(draw, size, &vec_b, &vec_c);
            draw_line(draw, size, &vec_c, &vec_a);
        }
    }

    fn casting(&self, draw: &Draw, mesh: &Mesh) {
        println!("Casting");
    }

    fn marching(&self, draw: &Draw, mesh: &Mesh) { println!("Marching"); }

    fn tracing(&self, draw: &Draw, mesh: &Mesh) { println!("tracing"); }
}

